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

use cita_types::Address;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PeersInfo {
    pub amount: u32,
    pub peers: Option<HashMap<Address, String>>,

    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::PeersInfo;
    use cita_types::Address;
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn peers_info_serialization_without_error_msg() {
        let addr1 = Address::random();
        let addr2 = Address::random();
        let addr3 = Address::random();

        let value = json!({
            "amount": 3,
            "peers": {
                format!("0x{:x}", addr1).to_string(): "12.123.14.53",
                format!("0x{:x}", addr2).to_string(): "32.52.64.32",
                format!("0x{:x}", addr3).to_string(): "67.68.32.21",
            },
            "errorMessage": serde_json::Value::Null,
        });

        let mut peers = HashMap::new();
        peers.insert(addr1, "12.123.14.53".to_owned());
        peers.insert(addr2, "32.52.64.32".to_owned());
        peers.insert(addr3, "67.68.32.21".to_owned());

        let peers_info = PeersInfo {
            amount: 3,
            peers: Some(peers),
            error_message: None,
        };

        assert_eq!(serde_json::to_value(peers_info).unwrap(), value);
    }

    #[test]
    fn peers_info_serialization_with_error_msg() {
        let value = json!({
            "amount": 0,
            "peers": serde_json::Value::Null,
            "errorMessage": "Disabled interface",
        });

        let peers_info = PeersInfo {
            amount: 0,
            peers: None,
            error_message: Some("Disabled interface".to_owned()),
        };

        assert_eq!(serde_json::to_value(peers_info).unwrap(), value);
    }
}
