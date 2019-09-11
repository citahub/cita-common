// Copyright Cryptape Technologies LLC.
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

pub extern crate crossbeam_channel as channel;
use crate::channel::Receiver;
use crate::channel::Sender;
use std::collections::HashMap;
use std::thread;
//use rand::{Rng,thread_rng};
use lazy_static::lazy_static;
#[macro_use]
extern crate cita_logger as logger;

pub const ZEROMQ_IP: &'static str = "ZEROMQ_IP";
//pub const ZEROMQ_BASE_PORT: &'static str = "ZEROMQ_BASE_PORT";

const TMP_BORROW_AMQP_URL: &str = "AMQP_URL";

lazy_static! {
    static ref SERVICE_PORT_INDEX: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();
        m.insert("network", 0);
        m.insert("chain", 1);
        m.insert("jsonrpc", 2);
        m.insert("consensus", 3);
        m.insert("executor", 4);
        m.insert("auth", 5);
        m.insert("snapshot", 6);
        m.insert("network_auth", 7);
        m.insert("network_consensus", 8);
        m
    };
    static ref CONTEXT: zmq::Context = zmq::Context::new();
    //static ref BASE_PORT: u16 = thread_rng().gen::<u16>();
}

fn new_sub_init(ctx: &zmq::Context, url: String) -> zmq::Socket {
    let skt = ctx.socket(zmq::SUB).unwrap();
    skt.connect(&*url)
        .expect(&*format!("subscribe socket create error {}", url));
    skt
}

fn thread_work(sub: &zmq::Socket, msg_sender: &Sender<(String, Vec<u8>)>) {
    let topic = sub.recv_string(0).unwrap().unwrap();
    let msg = sub.recv_bytes(0).unwrap();
    let _ = msg_sender.send((topic, msg));
}

pub fn start_zeromq(
    name: &str,
    keys: Vec<String>,
    tx: Sender<(String, Vec<u8>)>,
    rx: Receiver<(String, Vec<u8>)>,
) {
    let ip = std::env::var(ZEROMQ_IP).unwrap_or("127.0.0.1".to_string());
    let base_port_string = std::env::var(TMP_BORROW_AMQP_URL)
        .expect(&*format!("{} must be number", TMP_BORROW_AMQP_URL));
    let port: u16 = *base_port_string.as_bytes().last().unwrap() as u16 - '0' as u16;
    let base: u16 = 6000;
    let base_port: u16 = (base + port * 1000).into();

    //pub
    let publisher = CONTEXT.socket(zmq::PUB).unwrap();
    let mut publish_flag = true;
    let delimiter = ":".to_string();
    let con_type = "tcp".to_string();
    let wild_cast = "*".to_string();
    /*if ip == "localhost" || ip == "127.0.0.1" {
        delimiter = "".to_string();
        con_type = "ipc".to_string();
        wild_cast = "ipc".to_string();
        ip = "ipc".to_string();
    }*/

    /*URL like tcp://8.8.8.8:6000  or like ipc://ipc6000*/
    let _ = SERVICE_PORT_INDEX.get(&name).map_or_else(
        || {
            publish_flag = false;
        },
        |idx| {
            let url = format!(
                "{}://{}{}{}",
                con_type,
                wild_cast,
                delimiter,
                base_port + idx
            );
            publisher
                .bind(&*url)
                .expect(&*format!("publisher bind error {}", url));
        },
    );

    if publish_flag {
        let _ = thread::Builder::new()
            .name("publisher".to_string())
            .spawn(move || loop {
                let ret = rx.recv();

                if ret.is_err() {
                    break;
                }
                let (topic, msg) = ret.unwrap();
                let _ = publisher.send_multipart(&[&(topic.into_bytes())], zmq::SNDMORE);
                let _ = publisher.send(&msg, 0);
            });
    }

    //sub

    let network_subscriber = new_sub_init(
        &CONTEXT,
        format!("{}://{}{}{}", con_type, ip, delimiter, base_port),
    );
    let chain_subscriber = new_sub_init(
        &CONTEXT,
        format!("{}://{}{}{}", con_type, ip, delimiter, base_port + 1),
    );
    let jsonrpc_subscriber = new_sub_init(
        &CONTEXT,
        format!("{}://{}{}{}", con_type, ip, delimiter, base_port + 2),
    );
    let consensus_subscriber = new_sub_init(
        &CONTEXT,
        format!("{}://{}{}{}", con_type, ip, delimiter, base_port + 3),
    );
    let executor_subscriber = new_sub_init(
        &CONTEXT,
        format!("{}://{}{}{}", con_type, ip, delimiter, base_port + 4),
    );
    let auth_subscriber = new_sub_init(
        &CONTEXT,
        format!("{}://{}{}{}", con_type, ip, delimiter, base_port + 5),
    );
    let snapshot_subscriber = new_sub_init(
        &CONTEXT,
        format!("{}://{}{}{}", con_type, ip, delimiter, base_port + 6),
    );
    let net_tx_subscriber = new_sub_init(
        &CONTEXT,
        format!("{}://{}{}{}", con_type, ip, delimiter, base_port + 7),
    );
    let net_consensus_subscriber = new_sub_init(
        &CONTEXT,
        format!("{}://{}{}{}", con_type, ip, delimiter, base_port + 8),
    );

    let mut flag = 0;
    for topic in keys {
        let tmp = topic.clone();
        let v: Vec<&str> = tmp.split('.').collect();

        flag = match v[0] {
            "net" => match v[1] {
                "raw_bytes" | "signed_proposal" | "compact_signed_proposal" => {
                    net_consensus_subscriber
                        .set_subscribe(&topic.into_bytes())
                        .unwrap();
                    flag | 0x100
                }
                "get_block_txn" | "block_txn" | "request" => {
                    net_tx_subscriber
                        .set_subscribe(&topic.into_bytes())
                        .unwrap();
                    flag | 0x80
                }
                _ => {
                    network_subscriber
                        .set_subscribe(&topic.into_bytes())
                        .unwrap();
                    flag | 0x01
                }
            },
            "chain" => {
                chain_subscriber.set_subscribe(&topic.into_bytes()).unwrap();
                flag | 0x02
            }
            "jsonrpc" => {
                jsonrpc_subscriber
                    .set_subscribe(&topic.into_bytes())
                    .unwrap();
                flag | 0x04
            }
            "consensus" => {
                consensus_subscriber
                    .set_subscribe(&topic.into_bytes())
                    .unwrap();
                flag | 0x08
            }
            "executor" => {
                executor_subscriber
                    .set_subscribe(&topic.into_bytes())
                    .unwrap();
                flag | 0x10
            }
            "auth" => {
                auth_subscriber.set_subscribe(&topic.into_bytes()).unwrap();
                flag | 0x20
            }
            "snapshot" => {
                snapshot_subscriber
                    .set_subscribe(&topic.into_bytes())
                    .unwrap();
                flag | 0x40
            }
            "synchronizer" => flag,
            _ => {
                error!("invalid  flag! topic {}", tmp);
                flag
            }
        }
    }

    if flag & 0x01 != 0 {
        let other_tx = tx.clone();
        let _ = thread::Builder::new()
            .name("network_subscriber".to_string())
            .spawn(move || loop {
                thread_work(&network_subscriber, &other_tx);
            });
    }

    if flag & 0x02 != 0 {
        let other_tx = tx.clone();
        let _ = thread::Builder::new()
            .name("chain_subscriber".to_string())
            .spawn(move || loop {
                thread_work(&chain_subscriber, &other_tx);
            });
    }

    if flag & 0x04 != 0 {
        let other_tx = tx.clone();
        let _ = thread::Builder::new()
            .name("jsonrpc_subscriber".to_string())
            .spawn(move || loop {
                thread_work(&jsonrpc_subscriber, &other_tx);
            });
    }

    if flag & 0x08 != 0 {
        let other_tx = tx.clone();
        let _ = thread::Builder::new()
            .name("consensus_subscriber".to_string())
            .spawn(move || loop {
                thread_work(&consensus_subscriber, &other_tx);
            });
    }

    if flag & 0x10 != 0 {
        let other_tx = tx.clone();
        let _ = thread::Builder::new()
            .name("executor_subscriber".to_string())
            .spawn(move || loop {
                thread_work(&executor_subscriber, &other_tx);
            });
    }

    if flag & 0x20 != 0 {
        let other_tx = tx.clone();
        let _ = thread::Builder::new()
            .name("auth_subscriber".to_string())
            .spawn(move || loop {
                thread_work(&auth_subscriber, &other_tx);
            });
    }

    if flag & 0x40 != 0 {
        let other_tx = tx.clone();
        let _ = thread::Builder::new()
            .name("snapshot_subscriber".to_string())
            .spawn(move || loop {
                thread_work(&snapshot_subscriber, &other_tx);
            });
    }

    if flag & 0x80 != 0 {
        let other_tx = tx.clone();
        let _ = thread::Builder::new()
            .name("net_tx_subscriber".to_string())
            .spawn(move || loop {
                thread_work(&net_tx_subscriber, &other_tx);
            });
    }

    if flag & 0x100 != 0 {
        let other_tx = tx.clone();
        let _ = thread::Builder::new()
            .name("net_consensus_subscriber".to_string())
            .spawn(move || loop {
                thread_work(&net_consensus_subscriber, &other_tx);
            });
    }
}
