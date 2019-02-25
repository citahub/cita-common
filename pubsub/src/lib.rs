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
pub extern crate crossbeam_channel as channel;
extern crate dotenv;
#[cfg(feature = "kafka")]
extern crate pubsub_kafka;
#[cfg(feature = "rabbitmq")]
extern crate pubsub_rabbitmq;
#[cfg(feature = "zeromq")]
extern crate pubsub_zeromq;
use dotenv::dotenv;

#[cfg(feature = "kafka")]
use pubsub_kafka::start_kafka;

#[cfg(feature = "rabbitmq")]
use pubsub_rabbitmq::start_rabbitmq;

#[cfg(feature = "zeromq")]
use pubsub_zeromq::start_zeromq;

use channel::Receiver;
use channel::Sender;
use std::convert::Into;

#[cfg(feature = "rabbitmq")]
pub fn start_pubsub<K>(
    name: &str,
    keys: Vec<K>,
    tx: Sender<(String, Vec<u8>)>,
    rx: Receiver<(String, Vec<u8>)>,
) where
    K: Into<String>,
{
    dotenv().ok();
    let keys: Vec<String> = keys.into_iter().map(|e| e.into()).collect();
    start_rabbitmq(name, keys, tx, rx);
}

#[cfg(feature = "zeromq")]
pub fn start_pubsub<K>(
    name: &str,
    keys: Vec<K>,
    tx: Sender<(String, Vec<u8>)>,
    rx: Receiver<(String, Vec<u8>)>,
) where
    K: Into<String>,
{
    dotenv().ok();
    let keys: Vec<String> = keys.into_iter().map(|e| e.into()).collect();
    start_zeromq(name, keys, tx, rx);
}
#[cfg(feature = "kafka")]
pub fn start_pubsub<K>(
    name: &str,
    keys: Vec<K>,
    tx: Sender<(String, Vec<u8>)>,
    rx: Receiver<(String, Vec<u8>)>,
) where
    K: Into<String>,
{
    dotenv().ok();
    let keys: Vec<String> = keys.into_iter().map(|e| e.into()).collect();
    start_kafka(name, &keys, tx, rx);
}

/* TODO Fix this test.
#[cfg(test)]
mod test {
    extern crate crossbeam_channel as channel;
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn basics() {
        let (ntx_sub, nrx_sub) = channel::unbounded();
        let (ntx_pub, nrx_pub) = channel()::unbounded();
        start_pubsub(
            "network",
            vec!["chain.newtx", "chain.newblk"],
            ntx_sub,
            nrx_pub,
        );

        let (ctx_sub, crx_sub) = channel()::unbounded();
        let (ctx_pub, crx_pub) = channel()::unbounded();
        start_pubsub(
            "chain",
            vec!["network.newtx", "network.newblk"],
            ctx_sub,
            crx_pub,
        );

        ntx_pub
            .send(("network.newtx".to_string(), vec![49]))
            .unwrap();
        ntx_pub
            .send(("network.newblk".to_string(), vec![50]))
            .unwrap();

        ctx_pub.send(("chain.newtx".to_string(), vec![51])).unwrap();
        ctx_pub
            .send(("chain.newblk".to_string(), vec![52]))
            .unwrap();


        let mut chain = HashMap::new();
        let (key1, msg1) = crx_sub.recv().unwrap();
        chain.insert(key1, msg1);
        let (key2, msg2) = crx_sub.recv().unwrap();
        chain.insert(key2, msg2);

        let mut network = HashMap::new();
        let (key3, msg3) = nrx_sub.recv().unwrap();
        network.insert(key3, msg3);
        let (key4, msg4) = nrx_sub.recv().unwrap();
        network.insert(key4, msg4);

        assert_eq!(chain.get(&"network.newtx".to_string()).unwrap(), &vec![49]);
        assert_eq!(chain.get(&"network.newblk".to_string()).unwrap(), &vec![50]);

        assert_eq!(network.get(&"chain.newtx".to_string()).unwrap(), &vec![51]);
        assert_eq!(network.get(&"chain.newblk".to_string()).unwrap(), &vec![52]);
    }
}
*/
