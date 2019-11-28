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

use crate::rpc_types::Data;

// TODO: No need Deserialize. Just because test in trans.rs
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FullTransaction {
    pub hash: H256,
    pub content: Data,
    pub from: Address,
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
