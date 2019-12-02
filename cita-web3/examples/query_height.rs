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

extern crate cita_web3;

use cita_web3::web3::futures::Future;

fn main() {
    let url = {
        let mut args = ::std::env::args();
        args.nth(1).unwrap()
    };
    let (_eloop, transport) = cita_web3::web3::transports::Http::new(url.as_str()).unwrap();
    let web3 = cita_web3::web3::Web3::new(transport);
    let param = cita_web3::types::rpc_request::BlockNumberParams::new();
    let resp = web3
        .api::<cita_web3::api::Cita<cita_web3::web3::transports::Http>>()
        .call(param)
        .wait()
        .unwrap();
    println!("Response = {:?}", resp);
}
