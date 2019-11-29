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

#[cfg(feature = "kafka")]
extern crate pubsub_kafka;
#[cfg(feature = "rabbitmq")]
extern crate pubsub_rabbitmq;
#[cfg(feature = "zeromq")]
extern crate pubsub_zeromq;
use dotenv::dotenv;

#[cfg(feature = "kafka")]
pub use pubsub_kafka::{channel, start_kafka};

#[cfg(feature = "rabbitmq")]
pub use pubsub_rabbitmq::{channel, start_rabbitmq};

#[cfg(feature = "zeromq")]
pub use pubsub_zeromq::{channel, start_zeromq};

use crate::channel::Receiver;
use crate::channel::Sender;
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
    let keys: Vec<String> = keys.into_iter().map(Into::into).collect();
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
    let keys: Vec<String> = keys.into_iter().map(Into::into).collect();
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
    let keys: Vec<String> = keys.into_iter().map(Into::into).collect();
    start_kafka(name, &keys, tx, rx);
}
