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

/// Structs for combine paramters and exchange between request handler and response handler.
use crate::rpc_types::BlockNumber;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct CountOrCode {
    pub address: Vec<u8>,
    pub block_id: BlockNumber,
}

impl CountOrCode {
    pub fn new(address: Vec<u8>, block_id: BlockNumber) -> CountOrCode {
        CountOrCode { address, block_id }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct BlockParamsByHash {
    pub hash: Vec<u8>,
    pub include_txs: bool,
}

impl BlockParamsByHash {
    pub fn new(hash: Vec<u8>, include_txs: bool) -> BlockParamsByHash {
        BlockParamsByHash { hash, include_txs }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct BlockParamsByNumber {
    pub block_id: BlockNumber,
    pub include_txs: bool,
}

impl BlockParamsByNumber {
    pub fn new(block_id: BlockNumber, include_txs: bool) -> BlockParamsByNumber {
        BlockParamsByNumber {
            block_id,
            include_txs,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RpcBlock {
    pub block: Vec<u8>,
    pub include_txs: bool,
    pub hash: Vec<u8>,
}

impl RpcBlock {
    pub fn new(hash: Vec<u8>, include_txs: bool, block: Vec<u8>) -> RpcBlock {
        RpcBlock {
            block,
            include_txs,
            hash,
        }
    }
}
