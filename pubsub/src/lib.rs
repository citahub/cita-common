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
extern crate dotenv;
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
