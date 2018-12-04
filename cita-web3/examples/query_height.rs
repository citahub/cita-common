// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

extern crate cita_web3;

use cita_web3::web3::futures::Future;

fn main() {
    let url = {
        let mut args = ::std::env::args();
        args.nth(1).unwrap()
    };
    let (_eloop, transport) = cita_web3::web3::transports::Http::new(url.as_str()).unwrap();
    let web3 = cita_web3::web3::Web3::new(transport);
    let param = cita_web3::types::request::BlockNumberParams::new();
    let resp = web3
        .api::<cita_web3::api::Cita<cita_web3::web3::transports::Http>>()
        .call(param)
        .wait()
        .unwrap();
    println!("Response = {:?}", resp);
}
