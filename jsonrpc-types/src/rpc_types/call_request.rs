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

use crate::rpc_types::{Data, Data20};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct CallRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Data20>,
    pub to: Data20,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

impl CallRequest {
    pub fn new(from: Option<Data20>, to: Data20, data: Option<Data>) -> Self {
        CallRequest { from, to, data }
    }
}

#[cfg(test)]
mod tests {
    use super::CallRequest;
    use cita_types::H160;
    use serde_json;

    #[test]
    fn deserialize() {
        let testdata = vec![(
            json!({
                "from": "0x0000000000000000000000000000000000000001",
                "to": "0x0000000000000000000000000000000000000002",
                "data": "0xabcdef"
            })
            .to_string(),
            Some(CallRequest::new(
                Some(H160::from(1).into()),
                H160::from(2).into(),
                Some(vec![0xab, 0xcd, 0xef].into()),
            )),
        )];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<CallRequest, serde_json::Error> = serde_json::from_str(&data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }
}
