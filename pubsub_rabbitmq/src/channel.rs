// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

use std;
use std::thread;
use amqp::{protocol, Consumer, Table, Basic};
use mq::channel::{Channel, Type, Payload, Message};
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct MQChannel {
    channel: amqp::Channel,
    queue: String,
}

pub struct Handler {
    f: Box<Fn(Message)>
}

unsafe impl Send for Handler {}

impl Consumer for Handler {
    fn handle_delivery(
        &mut self,
        channel: &mut amqp::Channel,
        deliver: protocol::basic::Deliver,
        _: protocol::basic::BasicProperties,
        body: Vec<u8>,
    ) {
        let msg = Message {
            delivery_id: deliver.delivery_tag,
            payload: body,
            msg_type: "".to_string(),
            routing_key: deliver.routing_key,
        };
        (*self.f)(msg);
    }
}

impl Channel for MQChannel {
    fn sub(&mut self, f: Box<Fn(Message)>) {
        let callback = Handler { f };
        self.channel
            .basic_consume(
                callback,
                self.queue.clone(),
                "".to_string(),
                false,
                false,
                false,
                false,
                Table::new(),
            )
            .unwrap();
        self.channel.start_consuming();
        let _ = self.channel.close(200, "Bye");
    }

    fn ack(&mut self, msg: &Message) {
        self.channel.basic_ack(msg.delivery_id, false);
    }

    fn publish(&mut self, routing_key: &str, payload: &Payload) {
        self.channel.basic_publish(
            "cita",
            &routing_key,
            false,
            false,
            protocol::basic::BasicProperties {
                content_type: Some("text".to_string()),
                ..Default::default()
            },
            payload.to_bytes(),
        );
    }
}

pub const AMQP_URL: &'static str = "AMQP_URL";

pub fn new_channel(queue: &str, keys: Vec<String>, channel_type: Type) -> MQChannel {
    let amqp_url = std::env::var(AMQP_URL).expect(format!("{} must be set", AMQP_URL).as_str());
    let mut session = match amqp::Session::open_url(&amqp_url) {
        Ok(session) => session,
        Err(error) => panic!("failed to open url {} : {:?}", amqp_url, error),
    };

    let mut channel = session.open_channel(1).ok().expect("Can't open channel");
    {
        let channel = &mut channel;
        let _ = channel.basic_prefetch(10);

        let channel_type = match channel_type {
            Type::Broadcast => "topic",
            Type::Queue => "direct",
        };
        channel
            .exchange_declare(
                "cita",
                channel_type,
                false,
                true,
                false,
                false,
                false,
                Table::new(),
            )
            .unwrap();

        channel
            .queue_declare(queue.clone(), false, true, false, false, false, Table::new())
            .unwrap();

        for key in keys {
            channel
                .queue_bind(queue.clone(), "cita", &key, false, Table::new())
                .unwrap();
        }
    }
    let queue = queue.to_string();
    MQChannel { queue: queue, channel: channel }
}

#[cfg(test)]
mod tests {
    use std;
    use super::Channel;

    struct A {
        name: String
    }

    impl super::Payload for A {
        fn msg_type(&self) -> &str {
            "A"
        }
        fn to_bytes(&self) -> Vec<u8> {
            self.name.to_string().into_bytes()
        }
    }

    #[test]
    fn test() {
        if std::env::var(super::AMQP_URL).is_ok() {
            let mut mq = super::new_channel("hehe", vec!["a".to_string()], super::Type::Broadcast);
            let t = super::thread::spawn(move || {
                let msg = A { name: "nihao".to_string() };
                mq.publish("", &msg);
                println!("val is {}", mq.queue);
            });
            t.join().unwrap();
        }
    }
}