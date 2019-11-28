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

use cita_types::{Address, H256, U256};

use crate::rpc_types::{BlockTransaction, Proof};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockBody {
    pub transactions: Vec<BlockTransaction>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub timestamp: u64,
    #[serde(rename = "prevHash")]
    pub prev_hash: H256,
    pub number: U256,
    #[serde(rename = "stateRoot")]
    pub state_root: H256,
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: H256,
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: H256,
    #[serde(rename = "quotaUsed")]
    pub quota_used: U256,
    pub proof: Option<Proof>,
    pub proposer: Address,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Block {
    pub version: u32,
    pub hash: H256,
    pub header: BlockHeader,
    pub body: BlockBody,
}
