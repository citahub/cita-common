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

use crate::rpc_types::Data;
use cita_types::{H160, H256, U256};

/// Log
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Log {
    /// H160
    pub address: H160,
    /// Topics
    pub topics: Vec<H256>,
    /// Data
    pub data: Data,
    /// Block Hash
    #[serde(rename = "blockHash")]
    pub block_hash: Option<H256>,
    /// Block Height
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U256>,
    /// Transaction Hash
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<H256>,
    /// Transaction Index
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U256>,
    /// Log Index in Block
    #[serde(rename = "logIndex")]
    pub log_index: Option<U256>,
    /// Log Index in Transaction
    #[serde(rename = "transactionLogIndex")]
    pub transaction_log_index: Option<U256>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn log_serialization() {
        let value = json!({
            "address":"0x33990122638b9132ca29c723bdf037f1a891a70c",
            "topics":[
                "0xa6697e974e6a320f454390be03f74955e8978f1a6971ea6730542e37b66179bc",
                "0x4861736852656700000000000000000000000000000000000000000000000000"
            ],
            "data":"0x",
            "blockHash":"0xed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5",
            "blockNumber":"0x4510c",
            "transactionHash":"0x0000000000000000000000000000000000000000000000000000000000000000",
            "transactionIndex":"0x0",
            "transactionLogIndex":"0x1",
            "logIndex":"0x1"
        });

        let log = Log {
            address: H160::from_str("33990122638b9132ca29c723bdf037f1a891a70c").unwrap(),
            topics: vec![
                H256::from_str("a6697e974e6a320f454390be03f74955e8978f1a6971ea6730542e37b66179bc")
                    .unwrap(),
                H256::from_str("4861736852656700000000000000000000000000000000000000000000000000")
                    .unwrap(),
            ],
            data: vec![].into(),
            block_hash: Some(
                H256::from_str("ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5")
                    .unwrap(),
            ),
            block_number: Some(U256::from(0x4510c)),
            transaction_hash: Some(H256::default()),
            transaction_index: Some(U256::default()),
            transaction_log_index: Some(1.into()),
            log_index: Some(U256::from(1)),
        };

        assert_eq!(serde_json::to_value(log).unwrap(), value);
    }
}
