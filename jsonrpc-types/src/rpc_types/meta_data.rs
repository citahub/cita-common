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

use crate::rpc_types::{Data20, EconomicalModel, Quantity};

/// Metadata of current chain.
///
/// Related system contract: scripts/contracts/system/sys_config.sol
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct MetaData {
    /// The id of current chain
    #[serde(rename = "chainId")]
    pub chain_id: u32,
    /// The id v1 of current chain
    #[serde(rename = "chainIdV1")]
    pub chain_id_v1: Quantity,
    /// The name of current chain
    #[serde(rename = "chainName")]
    pub chain_name: String,
    /// The operator of current chain
    pub operator: String,
    /// Current operator's website URL
    pub website: String,
    /// Genesis block's timestamp (milliseconds)
    #[serde(rename = "genesisTimestamp")]
    pub genesis_timestamp: u64,
    /// Node address list which validate blocks
    pub validators: Vec<Data20>,
    /// The interval time for creating a block (milliseconds)
    #[serde(rename = "blockInterval")]
    pub block_interval: u64,
    /// Token name
    #[serde(rename = "tokenName")]
    pub token_name: String,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: String,
    #[serde(rename = "tokenAvatar")]
    pub token_avatar: String,
    pub version: u32,
    #[serde(rename = "economicalModel")]
    pub economical_model: EconomicalModel,
}

#[cfg(test)]
mod tests {
    use super::{EconomicalModel, MetaData};
    use cita_types::{Address, U256};
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn metadata_serialization() {
        let value = json!({
            "chainId": 123,
            "chainIdV1": "0x7b",
            "chainName": "test-chain-name",
            "operator": "test-operator",
            "website": "https://www.google.com",
            "genesisTimestamp": 1524000000000u64,
            "validators": [
                "0xa83ca59edc87a9cc7e384afa8d218dcca71cae88",
                "0xbc1fafd5ba5485f97e937fe574f836b275e593dd",
                "0xfc788efe3fda574e21691d383e429be02c530e4c",
                "0xe9deeae8b2a43675f113d11573119b9c68e5e3d8",
            ],
            "blockInterval": 3000,
            "tokenName": "Nervos",
            "tokenSymbol": "NOS",
            "tokenAvatar": "https://cdn.cryptape.com/icon_appchain.png",
            "version":108,
            "economicalModel": 1
        });
        let metadata = MetaData {
            chain_id: 123,
            chain_id_v1: U256::from(123).into(),
            chain_name: "test-chain-name".to_owned(),
            operator: "test-operator".to_owned(),
            website: "https://www.google.com".to_owned(),
            genesis_timestamp: 1524000000000,
            validators: vec![
                "a83ca59edc87a9cc7e384afa8d218dcca71cae88",
                "bc1fafd5ba5485f97e937fe574f836b275e593dd",
                "fc788efe3fda574e21691d383e429be02c530e4c",
                "e9deeae8b2a43675f113d11573119b9c68e5e3d8",
            ]
            .into_iter()
            .map(|s| Address::from_str(s).unwrap())
            .map(|s| s.into())
            .collect::<Vec<_>>(),
            block_interval: 3000,
            token_name: "Nervos".to_owned(),
            token_symbol: "NOS".to_owned(),
            token_avatar: "https://cdn.cryptape.com/icon_appchain.png".to_owned(),
            version: 108,
            economical_model: EconomicalModel::Charge,
        };
        assert_eq!(serde_json::to_value(metadata).unwrap(), value);
    }
}
