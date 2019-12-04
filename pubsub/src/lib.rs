// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use amqp::{protocol, Basic, Channel, Consumer, Session, Table};
// re-export crossbeam_channel
pub extern crate crossbeam_channel as channel;
use crate::channel::Receiver;
use crate::channel::Sender;
use dotenv::dotenv;
use std::process;
use std::thread;

pub struct Handler {
    tx: Sender<(String, Vec<u8>)>,
}

impl Handler {
    pub fn new(tx: Sender<(String, Vec<u8>)>) -> Self {
        Handler { tx }
    }
}

impl Consumer for Handler {
    fn handle_delivery(
        &mut self,
        channel: &mut Channel,
        deliver: protocol::basic::Deliver,
        _: protocol::basic::BasicProperties,
        body: Vec<u8>,
    ) {
        let _ = self.tx.send((deliver.routing_key, body));
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}

pub const AMQP_URL: &str = "AMQP_URL";

pub fn start_pubsub<K>(
    name: &str,
    keys: Vec<K>,
    tx: Sender<(String, Vec<u8>)>,
    rx: Receiver<(String, Vec<u8>)>,
) where
    K: Into<String>,
{
    dotenv().ok();
    let keys: Vec<String> = keys.into_iter().map(Into::into).collect();
    let amqp_url = std::env::var(AMQP_URL).unwrap_or_else(|_| panic!("{} must be set", AMQP_URL));
    let mut session = match Session::open_url(&amqp_url) {
        Ok(session) => session,
        Err(error) => panic!("failed to open url {} : {:?}", amqp_url, error),
    };

    let mut channel = session.open_channel(1).expect("Can't open channel");
    let _ = channel.basic_prefetch(10);
    channel
        .exchange_declare(
            "cita",
            "topic",
            false,
            true,
            false,
            false,
            false,
            Table::new(),
        )
        .unwrap();

    //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
    channel
        .queue_declare(name, false, true, false, false, false, Table::new())
        .unwrap();

    for key in keys {
        channel
            .queue_bind(name, "cita", &key, false, Table::new())
            .unwrap();
    }
    let callback = Handler::new(tx);
    //queue: &str, consumer_tag: &str, no_local: bool, no_ack: bool, exclusive: bool, nowait: bool, arguments: Table
    channel
        .basic_consume(callback, name, "", false, false, false, false, Table::new())
        .unwrap();

    // thread recv msg from mq
    let _ = thread::Builder::new()
        .name("subscriber".to_string())
        .spawn(move || {
            channel.start_consuming();
            let _ = channel.close(200, "Bye");
            // Rabbitmq connection closed, amqp didn't handle this failure
            // So we must exit the process, wait for restart
            process::exit(0);
        });

    let mut session = match Session::open_url(&amqp_url) {
        Ok(session) => session,
        Err(error) => panic!("failed to open url {} : {:?}", amqp_url, error),
    };
    let mut channel = session.open_channel(1).expect("Can't open channel");
    let _ = channel.basic_prefetch(10);
    channel
        .exchange_declare(
            "cita",
            "topic",
            false,
            true,
            false,
            false,
            false,
            Table::new(),
        )
        .unwrap();

    // thread send msg to mq
    let _ = thread::Builder::new()
        .name("publisher".to_string())
        .spawn(move || {
            loop {
                let ret = rx.recv();
                if ret.is_err() {
                    break;
                }
                let (routing_key, msg) = ret.unwrap();
                let ret = channel.basic_publish(
                    "cita",
                    &routing_key,
                    false,
                    false,
                    protocol::basic::BasicProperties {
                        content_type: Some("text".to_string()),
                        ..Default::default()
                    },
                    msg,
                );
                if ret.is_err() {
                    break;
                }
            }
            let _ = channel.close(200, "Bye");
            // Rabbitmq connection closed, amqp didn't handle this failure
            // So we must exit the process, wait for restart
            process::exit(0);
        });
}
