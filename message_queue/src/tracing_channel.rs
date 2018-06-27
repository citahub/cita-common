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

use std::convert::Into;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use channel::Message;
use channel::Channel;

struct TracingMessage<'a> {
    msg: &'a Message
}

impl<'a> Message for TracingMessage<'a> {
    fn msg_type(&self) -> &str { "" }
    fn to_bytes(&self) -> Vec<u8> { vec![] }
}

struct TracingChannel<'a> {
    channel: &'a Channel,
    name: &'a str,
}

impl<'a> TracingChannel<'a> {
    fn new(channel: &'a Channel, name: &'a str) -> Self {
        TracingChannel { channel, name }
    }
}

impl<'a> Channel for TracingChannel<'a> {
    fn ack(&self, msg: &Message) {
        // end new span
    }

    fn sub(&self, f: &Fn(&Message)) {
        // start new span
        self.channel.sub(&|msg| {
            f(&TracingMessage { msg: msg });
        })
    }

    fn publish(&self, msg: &Message) {
        // inject properties with new tracing ID
    }
}
