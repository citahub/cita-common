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

extern crate amqp;

extern crate amq_protocol;
extern crate futures;
extern crate lapin_futures as lapin;
extern crate tokio;
extern crate tokio_io;
#[macro_use]
extern crate logger;

pub mod lapin_amqp;
pub mod rust_amqp;

pub type Payload = (String, Vec<u8>);

pub const EXCHANGE: &str = "cita";
pub const EXCHANGE_TYPE: &str = "topic";

pub const AMQP_URL: &'static str = "AMQP_URL";

#[cfg(feature = "rust-amqp")]
pub use rust_amqp::start_rabbitmq;

#[cfg(not(feature = "rust-amqp"))]
pub use lapin_amqp::start_rabbitmq;
