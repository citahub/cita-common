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

use crate::rpc_request::{
    BlockNumberParams, CallParams, EstimateQuotaParams, GetAbiParams, GetBalanceParams,
    GetBlockByHashParams, GetBlockByNumberParams, GetBlockHeaderParams, GetCodeParams,
    GetFilterChangesParams, GetFilterLogsParams, GetLogsParams, GetMetaDataParams,
    GetStateProofParams, GetTransactionCountParams, GetTransactionParams,
    GetTransactionProofParams, GetTransactionReceiptParams, GetVersionParams, NewBlockFilterParams,
    NewFilterParams, PeerCountParams, PeersInfoParams, SendRawTransactionParams,
    UninstallFilterParams,
};
use crate::rpc_types::{BlockNumber, CallRequest, Filter, VariadicValue};
use cita_types::{H160, H256, U256};
use serde_json;
use std::convert::Into;

macro_rules! test_ser_and_de {
    ($type:tt, $json_params:tt, $value:tt) => {
        let data = $type::new$value;
        let serialized = serde_json::to_string(&data).unwrap();
        let jsonstr = json!($json_params).to_string();
        assert_eq!(serialized, jsonstr);
        let deserialized: $type = serde_json::from_str(&jsonstr).unwrap();
        assert_eq!(deserialized, data);
    };
    (value, $type:tt, $json_params:tt, $value:tt) => {
        let data = $type::new$value;
        let serialized = serde_json::to_value(&data).unwrap();
        let jsonval = json!($json_params);
        assert_eq!(serialized, jsonval);
        let jsonstr = jsonval.to_string();
        let deserialized: $type = serde_json::from_str(&jsonstr).unwrap();
        assert_eq!(deserialized, data);
    };
}

#[test]
fn serialize_and_deserialize() {
    test_ser_and_de!(BlockNumberParams, [], ());

    test_ser_and_de!(PeerCountParams, [], ());

    test_ser_and_de!(
        SendRawTransactionParams,
        ["0xabcdef"],
        (vec![0xab, 0xcd, 0xef].into())
    );

    test_ser_and_de!(
        GetBlockByHashParams,
        [
            "0x000000000000000000000000000000000000000000000000000000000000000a",
            true
        ],
        (H256::from_low_u64_be(10).into(), true.into())
    );

    test_ser_and_de!(
        GetBlockByNumberParams,
        ["0x10", false],
        (BlockNumber::new(16u64.into()), false.into())
    );

    test_ser_and_de!(
        GetTransactionReceiptParams,
        ["0x000000000000000000000000000000000000000000000000000000000000000a"],
        (H256::from_low_u64_be(10).into())
    );

    test_ser_and_de!(
        value,
        GetLogsParams,
        [{
            "fromBlock": "0xb",
            "address": "0x0000000000000000000000000000000000000010",
            "topics": [
                "0x0000000000000000000000000000000000000000000000000000000000000001",
                [
                    "0x0000000000000000000000000000000000000000000000000000000000000002",
                    "0x0000000000000000000000000000000000000000000000000000000000000003",
                ],
                null,
            ]
        }],
        (Filter::new(
            BlockNumber::new(11u64.into()),
            BlockNumber::latest(),
            Some(VariadicValue::single(H160::from_low_u64_be(16).into())),
            Some(
                vec![
                    VariadicValue::single(H256::from_low_u64_be(1).into()),
                    VariadicValue::multiple(
                        vec![
                            H256::from_low_u64_be(2).into(),
                            H256::from_low_u64_be(3).into(),
                        ]),
                    VariadicValue::null(),
                ]
            ),
        ))
    );

    test_ser_and_de!(
    CallParams,
    [
        {
            "from": "0x000000000000000000000000000000000000000b",
            "to": "0x000000000000000000000000000000000000000c",
        },
        "latest",
    ],
    (
        CallRequest::new(Some(H160::from_low_u64_be(11).into()),
        H160::from_low_u64_be(12).into(), None),
        BlockNumber::latest()
    ));

    test_ser_and_de!(
    EstimateQuotaParams,
    [
        {
            "from": "0x000000000000000000000000000000000000000b",
            "to": "0x000000000000000000000000000000000000000c",
        },
        "latest",
    ],
    (
        CallRequest::new(Some(H160::from_low_u64_be(11).into()),
        H160::from_low_u64_be(12).into(), None),
        BlockNumber::latest()
    ));

    test_ser_and_de!(
        GetTransactionParams,
        ["0x000000000000000000000000000000000000000000000000000000000000000a"],
        (H256::from_low_u64_be(10).into())
    );

    test_ser_and_de!(
        GetTransactionCountParams,
        ["0x000000000000000000000000000000000000000a", "0x10"],
        (H160::from_low_u64_be(10).into(), BlockNumber::new(16u64.into()))
    );

    test_ser_and_de!(
        GetCodeParams,
        ["0x000000000000000000000000000000000000000b", "0x11"],
        (H160::from_low_u64_be(11).into(), BlockNumber::new(17u64.into()))
    );

    test_ser_and_de!(
        GetAbiParams,
        ["0x000000000000000000000000000000000000000c", "0x12"],
        (H160::from_low_u64_be(12).into(), BlockNumber::new(18u64.into()))
    );

    test_ser_and_de!(
        GetBalanceParams,
        ["0x000000000000000000000000000000000000000d", "0x13"],
        (H160::from_low_u64_be(13).into(), BlockNumber::new(19u64.into()))
    );

    test_ser_and_de!(
        value,
        NewFilterParams,
        [{
            "fromBlock": "0xc",
            "address": "0x0000000000000000000000000000000000000010",
            "topics": [
                "0x0000000000000000000000000000000000000000000000000000000000000001",
                [
                    "0x0000000000000000000000000000000000000000000000000000000000000002",
                    "0x0000000000000000000000000000000000000000000000000000000000000003",
                ],
                null,
            ]
        }],
        (Filter::new(
            BlockNumber::new(12u64.into()),
            BlockNumber::latest(),
            Some(VariadicValue::single(H160::from_low_u64_be(16).into())),
            Some(
                vec![
                    VariadicValue::single(H256::from_low_u64_be(1).into()),
                    VariadicValue::multiple(
                        vec![
                            H256::from_low_u64_be(2).into(),
                            H256::from_low_u64_be(3).into(),
                        ]),
                    VariadicValue::null(),
                ]
            ),
        ))
    );

    test_ser_and_de!(NewBlockFilterParams, [], ());

    test_ser_and_de!(UninstallFilterParams, ["0xa"], (U256::from(10).into()));

    test_ser_and_de!(GetFilterChangesParams, ["0xb"], (U256::from(11).into()));

    test_ser_and_de!(GetFilterLogsParams, ["0xc"], (U256::from(12).into()));

    test_ser_and_de!(
        GetTransactionProofParams,
        ["0x000000000000000000000000000000000000000000000000000000000000000b"],
        (H256::from_low_u64_be(11).into())
    );

    test_ser_and_de!(GetMetaDataParams, ["earliest"], (BlockNumber::earliest()));

    test_ser_and_de!(
        GetStateProofParams,
        [
            "0x000000000000000000000000000000000000000a",
            "0x000000000000000000000000000000000000000000000000000000000000000b",
            "earliest"
        ],
        (
            H160::from_low_u64_be(10).into(),
            H256::from_low_u64_be(11).into(),
            BlockNumber::earliest()
        )
    );

    test_ser_and_de!(
        GetBlockHeaderParams,
        ["earliest"],
        (BlockNumber::earliest())
    );

    test_ser_and_de!(GetVersionParams, [], ());
    test_ser_and_de!(PeersInfoParams, [], ());
}
