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

#[macro_use]
extern crate cita_logger as logger;
pub extern crate crossbeam_channel as channel;
extern crate zmq;
use crate::channel::Receiver;
use crate::channel::Sender;
use std::thread;

pub fn start_zeromq(
    name: &str,
    keys: Vec<String>,
    tx: Sender<(String, Vec<u8>)>,
    rx: Receiver<(String, Vec<u8>)>,
) {
    let context = zmq::Context::new();
    //pub
    let publisher = context.socket(zmq::PUB).unwrap();
    match name {
        "network" => assert!(publisher.bind("tcp://*:5563").is_ok()),
        "chain" => assert!(publisher.bind("tcp://*:5564").is_ok()),
        "jsonrpc" => assert!(publisher.bind("tcp://*:5565").is_ok()),
        "consensus" => assert!(publisher.bind("tcp://*:5566").is_ok()),
        _ => error!("not hava {} module !", name),
    }

    let _ = thread::Builder::new()
        .name("publisher".to_string())
        .spawn(move || loop {
            let ret = rx.recv();

            if ret.is_err() {
                break;
            }
            let (topic, msg) = ret.unwrap();
            publisher
                .send_multipart(&[&(topic.into_bytes())], zmq::SNDMORE)
                .unwrap();
            publisher.send(&msg, 0).unwrap();
        });

    //sub

    let network_subscriber = context.socket(zmq::SUB).unwrap();
    assert!(network_subscriber.connect("tcp://localhost:5563").is_ok());

    let chain_subscriber = context.socket(zmq::SUB).unwrap();
    assert!(chain_subscriber.connect("tcp://localhost:5564").is_ok());

    let jsonrpc_subscriber = context.socket(zmq::SUB).unwrap();
    assert!(jsonrpc_subscriber.connect("tcp://localhost:5565").is_ok());

    let consensus_subscriber = context.socket(zmq::SUB).unwrap();
    assert!(network_subscriber.connect("tcp://localhost:5566").is_ok());

    let mut flag = 400;
    for topic in keys {
        flag = match name {
            "network" => {
                network_subscriber
                    .set_subscribe(&topic.into_bytes())
                    .unwrap();
                0
            }
            "chain" => {
                chain_subscriber
                    .set_subscribe(&topic.to_string().into_bytes())
                    .unwrap();
                1
            }
            "jsonrpc" => {
                jsonrpc_subscriber
                    .set_subscribe(&topic.to_string().into_bytes())
                    .unwrap();
                2
            }
            "consensus" => {
                consensus_subscriber
                    .set_subscribe(&topic.to_string().into_bytes())
                    .unwrap();
                3
            }
            _ => {
                error!("invalid  flag!");
                -1
            }
        }
    }

    let _ = thread::Builder::new()
        .name("subscriber".to_string())
        .spawn(move || loop {
            match flag {
                0 => {
                    let topic = network_subscriber.recv_string(0).unwrap().unwrap();
                    let msg = network_subscriber.recv_bytes(0).unwrap();
                    let _ = tx.send((topic, msg));
                }

                1 => {
                    let topic = chain_subscriber.recv_string(0).unwrap().unwrap();
                    let msg = chain_subscriber.recv_bytes(0).unwrap();
                    let _ = tx.send((topic, msg));
                }

                2 => {
                    let topic = jsonrpc_subscriber.recv_string(0).unwrap().unwrap();
                    let msg = jsonrpc_subscriber.recv_bytes(0).unwrap();
                    let _ = tx.send((topic, msg));
                }

                3 => {
                    let topic = consensus_subscriber.recv_string(0).unwrap().unwrap();
                    let msg = consensus_subscriber.recv_bytes(0).unwrap();
                    let _ = tx.send((topic, msg));
                }

                _ => {
                    break;
                }
            }
        });
}
