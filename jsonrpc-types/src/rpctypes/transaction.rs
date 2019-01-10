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

use cita_types::{Address, H256, U256};
use libproto::TryInto;
use libproto::{
    FullTransaction as ProtoFullTransaction, SignedTransaction as ProtoSignedTransaction,
};
use rpctypes::Data;

use crate::rpctypes::Data;

// TODO: No need Deserialize. Just because test in trans.rs
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FullTransaction {
    pub hash: H256,
    pub content: Data,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RpcTransaction {
    pub hash: H256,
    pub content: Data,
    pub from: Address,
    #[serde(rename = "blockNumber")]
    pub block_number: U256,
    #[serde(rename = "blockHash")]
    pub block_hash: H256,
    pub index: U256,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockTransaction {
    Full(FullTransaction),
    Hash(H256),
}
