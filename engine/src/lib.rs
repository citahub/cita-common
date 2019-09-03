// Copyright Cryptape Technologies LLC.
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

extern crate cita_crypto as crypto;
extern crate cita_types as types;
extern crate crossbeam_channel as channel;

mod error;

pub use crate::error::*;
use libproto::Request;
pub use util::instrument::*;

use crate::channel::Sender;
use crate::types::H256;
use libproto::blockchain::{Block, RichStatus};
use std::time::Duration;
use util::SemVer;

pub trait Engine: Sync + Send {
    fn name(&self) -> &str;

    fn version(&self) -> SemVer {
        SemVer::new(1, 2, 3)
    }

    fn duration(&self) -> Duration;

    fn verify_block(&self, block: &Block) -> Result<(), EngineError>;

    fn receive_new_transaction(
        &self,
        tx_req: &Request,
        tx_pub: Sender<(String, Vec<u8>)>,
        _origin: u32,
        from_broadcast: bool,
    );

    fn receive_new_block(&self, block: &Block, tx_pub: Sender<(String, Vec<u8>)>);

    fn receive_new_status(&self, status: &RichStatus);

    fn new_block(&self, tx_pub: Sender<(String, Vec<u8>)>);

    fn set_new_status(&self, height: usize, pre_hash: H256);

    fn new_messages(&self, tx_pub: Sender<(String, Vec<u8>)>);

    fn handle_message(
        &self,
        _message: Vec<u8>,
        tx_pub: Sender<(String, Vec<u8>)>,
    ) -> Result<(), EngineError>;

    fn handle_proposal(
        &self,
        _message: Vec<u8>,
        tx_pub: Sender<(String, Vec<u8>)>,
    ) -> Result<(), EngineError>;
}

#[test]
fn it_works() {}
