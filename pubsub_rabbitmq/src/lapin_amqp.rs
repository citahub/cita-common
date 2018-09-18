// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::io::{self, Error, ErrorKind};
use std::net::{SocketAddr, ToSocketAddrs};
use std::str::FromStr;
use std::sync::mpsc as std_mpsc;
use std::thread;

use amq_protocol::uri::AMQPUri;
use futures::future::Future;
use futures::sync::mpsc;
use futures::{stream, Stream};
use lapin::channel::{
    BasicConsumeOptions, BasicProperties, BasicPublishOptions, ExchangeDeclareOptions,
    QueueBindOptions, QueueDeclareOptions, QueuePurgeOptions,
};
use lapin::client::{Client, ConnectionOptions};
use lapin::message::Delivery;
use lapin::types::FieldTable;
use tokio;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio_io::{AsyncRead, AsyncWrite};

use super::{Payload, AMQP_URL, EXCHANGE, EXCHANGE_TYPE};

fn connect_consumer(
    addr: SocketAddr,
    conn_opts: ConnectionOptions,
    name: String,
    keys: Vec<String>,
    consumer_tx: std_mpsc::Sender<Payload>,
) -> impl Future<Item = (), Error = io::Error> + Send + 'static {
    TcpStream::connect(&addr)
        .and_then(|stream| Client::connect(stream, conn_opts))
        .and_then(move |(client, heartbeat)| {
            tokio::spawn(heartbeat.map_err(|_| ()));
            consumer(&client, name, keys, consumer_tx)
        })
}

fn connect_publisher(
    addr: SocketAddr,
    conn_opts: ConnectionOptions,
    name: String,
    publisher_rx: mpsc::Receiver<Payload>,
) -> impl Future<Item = (), Error = io::Error> + Send + 'static {
    TcpStream::connect(&addr)
        .and_then(|stream| Client::connect(stream, conn_opts))
        .and_then(move |(client, heartbeat)| {
            tokio::spawn(heartbeat.map_err(|_| ()));
            publisher(&client, name, publisher_rx)
        })
}

fn publisher<T>(
    client: &Client<T>,
    name: String,
    publisher_rx: mpsc::Receiver<Payload>,
) -> impl Future<Item = (), Error = io::Error> + Send + 'static
where
    T: AsyncRead + AsyncWrite,
    T: Send + Sync,
    T: 'static,
{
    client.create_channel().and_then(move |channel| {
        channel
            .queue_declare(&name, QueueDeclareOptions::default(), FieldTable::new())
            .and_then(move |_| {
                debug!("publisher queue declared");
                channel
                    .queue_purge(&name, QueuePurgeOptions::default())
                    .and_then(move |_| {
                        debug!("publisher queue purged");
                        publisher_rx
                            .for_each(move |(routing_key, msg): Payload| {
                                tokio::spawn(
                                    channel
                                        .basic_publish(
                                            EXCHANGE,
                                            &routing_key,
                                            msg,
                                            BasicPublishOptions::default(),
                                            BasicProperties::default(),
                                        )
                                        .map(|_| ())
                                        .map_err(|_| ()),
                                )
                            })
                            .map_err(|_| Error::new(ErrorKind::Other, "channel closed!"))
                    })
            })
    })
}

fn consumer<T>(
    client: &Client<T>,
    name: String,
    keys: Vec<String>,
    consumer_tx: std_mpsc::Sender<Payload>,
) -> impl Future<Item = (), Error = io::Error> + Send + 'static
where
    T: AsyncRead + AsyncWrite,
    T: Send + Sync,
    T: 'static,
{
    let keys = stream::iter_ok::<_, ()>(keys);
    client.create_channel().and_then(move |channel| {
        let id = channel.id;
        debug!("created channel with id: {}", id);

        let ch1 = channel.clone();
        let ch2 = channel.clone();

        let name_clone = name.clone();

        channel
            .exchange_declare(
                EXCHANGE,
                EXCHANGE_TYPE,
                ExchangeDeclareOptions::default(),
                FieldTable::new(),
            )
            .and_then(move |_| {
                channel
                    .queue_declare(&name, QueueDeclareOptions::default(), FieldTable::new())
                    .and_then(move |queue| {
                        debug!("channel {} declared queue {}", id, name_clone);

                        keys.for_each(move |key| {
                            channel
                                .queue_bind(
                                    &name_clone,
                                    EXCHANGE,
                                    &key,
                                    QueueBindOptions::default(),
                                    FieldTable::new(),
                                )
                                .map(|_| ())
                                .map_err(|_| ())
                        }).map_err(|_| Error::new(ErrorKind::Other, "queue_bind error!"))
                            .map(|_| queue)
                    })
                    .and_then(move |queue| {
                        ch1.basic_consume(
                            &queue,
                            &name,
                            BasicConsumeOptions::default(),
                            FieldTable::new(),
                        )
                    })
            })
            .and_then(|stream| {
                stream.for_each(move |message| {
                    let Delivery {
                        delivery_tag,
                        routing_key,
                        data,
                        ..
                    } = message;
                    let ret = consumer_tx.send((routing_key, data));
                    if ret.is_err() {
                        error!("amqp message send error {:?}", ret);
                    }
                    ch2.basic_ack(delivery_tag, false)
                })
            })
    })
}

pub fn start_rabbitmq(
    name: &str,
    keys: Vec<String>,
    tx: std_mpsc::Sender<Payload>,
    rx: std_mpsc::Receiver<Payload>,
) {
    debug!("Starting rabbitmq via lapin-amqp ...");

    let amqp_url = ::std::env::var(AMQP_URL).expect(format!("{} must be set", AMQP_URL).as_str());

    let uri = AMQPUri::from_str(&amqp_url)
        .unwrap_or_else(|err| panic!("Failed to parse AMQP Url {}: {}", amqp_url, err));

    let addr = format!("{}:{}", uri.authority.host, uri.authority.port)
        .as_str()
        .to_socket_addrs()
        .map_err(|err| panic!("Failed to parse or resolve addr from {}: {}", amqp_url, err))
        .ok()
        .and_then(|mut x| x.next())
        .unwrap_or_else(|| panic!("Failed to parse or resolve addr from {}", amqp_url));

    let conn_opts = ConnectionOptions::from_uri(uri);

    let (publisher_tx, publisher_rx) = mpsc::channel(65535);

    // Transfer data between different channels for api compatibility
    let _ = thread::Builder::new()
        .name("transfer".to_string())
        .spawn(move || {
            loop {
                if let Ok(data) = rx.recv() {
                    let _ = publisher_tx.clone().try_send(data);
                } else {
                    break;
                }
            }
            ::std::process::exit(0);
        });

    {
        let name = name.to_owned();
        let conn_opts = conn_opts.clone();
        thread::spawn(move || {
            Runtime::new().unwrap().block_on_all(connect_publisher(
                addr,
                conn_opts,
                name,
                publisher_rx,
            ))
        });
    }

    {
        let name = name.to_owned();
        let conn_opts = conn_opts.clone();
        thread::spawn(move || {
            Runtime::new()
                .unwrap()
                .block_on_all(connect_consumer(addr, conn_opts, name, keys, tx))
        });
    }
}
