// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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
