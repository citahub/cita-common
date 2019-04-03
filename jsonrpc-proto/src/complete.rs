// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use jsonrpc_types::rpc_request::{
    BlockNumberParams, CallParams, GetAbiParams, GetBalanceParams, GetBlockByHashParams,
    GetBlockByNumberParams, GetBlockHeaderParams, GetCodeParams, GetFilterChangesParams,
    GetFilterLogsParams, GetLogsParams, GetMetaDataParams, GetStateProofParams,
    GetStorageKeyParams, GetTransactionCountParams, GetTransactionParams,
    GetTransactionProofParams, GetTransactionReceiptParams, GetVersionParams, NewBlockFilterParams,
    NewFilterParams, PeerCountParams, PeersInfoParams, SendRawTransactionParams,
    SendTransactionParams, UninstallFilterParams,
};
use jsonrpc_types::rpc_request::{Call, JsonRpcRequest, PartialCall, PartialRequest, Request};
use jsonrpc_types::{rpc_types::Params as PartialParams, Error};
use libproto::Request as ProtoRequest;
use serde_json;

pub trait Complete {
    type Output;
    type Error;

    fn complete(self) -> Result<Self::Output, Self::Error>;
}

impl Complete for PartialRequest {
    type Output = Request;
    type Error = Error;

    fn complete(self) -> Result<Self::Output, Self::Error> {
        let PartialRequest { jsonrpc, id, call } = self;
        if let Some(part_call) = call {
            part_call
                .complete()
                .map(|full_call| Request::new(jsonrpc, id, full_call))
        } else {
            Err(Error::method_not_found())
        }
    }
}

macro_rules! partial_call_complete {
    ($( ($enum_name:ident, $params_name:ident: $params_list:expr, $result_type:ident) ),+ ,) => {
        partial_call_complete!($( ($enum_name, $params_name) ),+);
    };
    ($( ($enum_name:ident, $params_name:ident) ),+) => {
        impl Complete for PartialCall {
            type Output = Call;
            type Error = Error;

            fn complete(self) -> Result<Self::Output, Self::Error> {
                match self {
                    $(
                        PartialCall::$enum_name { params } => {
                            if let Some(params) = params {
                                let pparams: PartialParams = serde_json::from_value(params.clone())?;
                                if pparams.len() != $params_name::required_len() {
                                    Err(Error::invalid_params_len())
                                } else {
                                    Ok(Call::$enum_name{ params: serde_json::from_value(params)? })
                                }
                            } else {
                                if $params_name::required_len() == 0 {
                                    Ok(Call::$enum_name{
                                        params: serde_json::from_value(
                                                    serde_json::Value::Array(Vec::new()))?})
                                } else {
                                    Err(Error::invalid_params("params is requeired"))
                                }
                            }
                        },
                    )+
                }
            }
        }
    }
}

impl_for_each_jsonrpc_requests!(partial_call_complete);

pub trait CompleteInto {
    fn complete_and_into_proto(self) -> Result<(Request, ProtoRequest), Error>;
}

impl CompleteInto for PartialRequest {
    fn complete_and_into_proto(self) -> Result<(Request, ProtoRequest), Error> {
        use crate::from_into::TryIntoProto;

        self.complete().and_then(|full_req| {
            full_req
                .clone()
                .try_into_proto()
                .map(|proto_req| (full_req, proto_req))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cita_types::H256;

    #[test]
    fn test_get_transaction_receipt_params_complete() {
        let params = GetTransactionReceiptParams::new(H256::from(10).into());
        let full_req = params.into_request(1);

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransactionReceipt",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        assert_eq!(part_req.complete().unwrap(), full_req);
    }

    #[test]
    fn test_get_transaction_receipt_params_complete_error() {
        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt"
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();

        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::invalid_params("params is requeired")
        );

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": [1, 2]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::invalid_params_len()
        );
    }

    #[test]
    fn test_block_number_params_complete() {
        let params = BlockNumberParams::new();
        let full_req = params.into_request(2);

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": 2,
            "method": "blockNumber"
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        assert_eq!(part_req.complete().unwrap(), full_req);
    }

    #[test]
    fn test_method_not_found_complete_error() {
        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();

        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::method_not_found()
        );

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "notAMethod",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::method_not_found()
        );
    }
}
