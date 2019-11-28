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

use crate::rpc_request::{BlockNumberParams, GetTransactionReceiptParams, PartialRequest, Request};
use cita_types::H256;
use serde_json;
use std::convert::Into;

macro_rules! test_ser_and_de {
    ($type:ty, $data:ident, $json_params:tt) => {
        let serialized = serde_json::to_value(&$data).unwrap();
        let jsonval = json!($json_params);
        assert_eq!(serialized, jsonval);
        let jsonstr = jsonval.to_string();
        let deserialized = serde_json::from_str::<$type>(&jsonstr);
        if let Ok(deserialized) = deserialized {
            assert_eq!(deserialized, $data);
        } else {
            assert_eq!(&jsonstr, "");
        }
    };
}

#[test]
fn serialize_and_deserialize() {
    let params = GetTransactionReceiptParams::new(H256::from(10).into());
    test_ser_and_de!(
        GetTransactionReceiptParams,
        params,
        ["0x000000000000000000000000000000000000000000000000000000000000000a"]
    );

    let full_req = params.into_request(1);
    test_ser_and_de!(Request, full_req,  {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransactionReceipt",
        "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"],
    });

    let req_str: String = full_req.clone().into();
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransactionReceipt",
        "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"],
    });

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransactionReceipt",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransactionReceipt",
        "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"],
    });

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt"
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
        "jsonrpc": "2.0",
        "id": null,
        "method": "getTransactionReceipt",
        "params": null,
    });

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": [1, 2]
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
        "jsonrpc": "2.0",
        "id": null,
        "method": "getTransactionReceipt",
        "params": [1, 2],
    });

    let params = BlockNumberParams::new();
    test_ser_and_de!(BlockNumberParams, params, []);

    let full_req = params.into_request(2);
    test_ser_and_de!(Request, full_req,  {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "blockNumber",
        "params": [],
    });

    let req_str: String = full_req.clone().into();
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "blockNumber",
        "params": [],
    });

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": 2,
            "method": "blockNumber"
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "blockNumber",
        "params": null,
    });

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
        "jsonrpc": "2.0",
        "id": null,
    });

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "notAMethod",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
        "jsonrpc": "2.0",
        "id": null,
    });
}
