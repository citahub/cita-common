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

pub struct MQMessage {
    pub routing_key: String,
    pub msg_type: String,
    pub payload: Vec<u8>,
    pub delivery_id: u64,
}

pub trait Message {
    fn msg_type(&self) -> &str;
    fn to_bytes(&self) -> Vec<u8>;
}

pub enum Type {
    Broadcast,
    Queue,
}

pub trait Channel {
    fn ack(&mut self, msg: &MQMessage);
    fn sub(&mut self, f: Box<Fn(MQMessage)>);
    fn publish(&mut self, routing_key: &str, payload: &Message);
}

