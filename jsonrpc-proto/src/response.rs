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

use jsonrpc_types::rpc_types::CallResult;
use jsonrpc_types::{
    rpc_request::{RequestInfo, ResponseResult},
    rpc_response::{Output, RpcFailure, RpcSuccess},
    rpc_types::{
        Block, FilterChanges, LicenseInfo, Log, MetaData, PeersInfo, PoolTxNum, Receipt, RpcBlock,
        RpcTransaction, SoftwareVersion,
    },
    Error,
};
use libproto::response::{Response, Response_oneof_data};

pub trait OutputExt {
    fn from_res_info(resp: Response, info: RequestInfo) -> Self;
}

impl OutputExt for Output {
    /// Creates new output given `Result`, `Id` and `Version`.
    fn from_res_info(resp: Response, info: RequestInfo) -> Self {
        use crate::{block::BlockExt, error::ErrorExt, from_into::TryFromProto};

        let code = resp.get_code();
        if let Some(response) = resp.data {
            if code == 0 {
                let success = RpcSuccess::new(info.clone());
                match response {
                    Response_oneof_data::tx_state(tx_state) => {
                        serde_json::from_str(&tx_state)
                            .map(|tx_response| {
                                // SendTransaction, SendRawTransaction
                                success
                                    .set_result(ResponseResult::SendTransaction(tx_response))
                                    .output()
                            })
                            .unwrap_or_else(|_| Output::system_error(0))
                    }
                    Response_oneof_data::block_number(bn) => success
                        .set_result(ResponseResult::BlockNumber(bn.into()))
                        .output(),
                    Response_oneof_data::none(_) => success.output(),
                    Response_oneof_data::block(rpc_block) => {
                        serde_json::from_str::<RpcBlock>(&rpc_block)
                            .map_err(|err| Error::rpc_block_decode_error(Box::new(err)))
                            .and_then(Block::try_from_rpc_block)
                            .map(|block| {
                                // GetBlockByHash, GetBlockByNumber
                                success
                                    .set_result(ResponseResult::GetBlockByHash(block))
                                    .output()
                            })
                            .unwrap_or_else(|_| Output::system_error(0))
                    }
                    Response_oneof_data::ts(x) => RpcTransaction::try_from_proto(x)
                        .map(|tx| {
                            success
                                .set_result(ResponseResult::GetTransaction(tx))
                                .output()
                        })
                        .unwrap_or_else(|_| Output::system_error(0)),
                    Response_oneof_data::peercount(x) => success
                        .set_result(ResponseResult::PeerCount(x.into()))
                        .output(),
                    Response_oneof_data::call_result(x) => success
                        .set_result(ResponseResult::GetCode(x.into()))
                        .output(),
                    Response_oneof_data::call_full_result(serialized) => {
                        serde_json::from_str::<CallResult>(&serialized)
                            .map(|result| success.set_result(ResponseResult::Call(result)).output())
                            .unwrap_or_else(|_| Output::system_error(0))
                    }
                    Response_oneof_data::logs(serialized) => {
                        serde_json::from_str::<Vec<Log>>(&serialized)
                            .map(|logs| success.set_result(ResponseResult::GetLogs(logs)).output())
                            .unwrap_or_else(|_| Output::system_error(0))
                    }
                    Response_oneof_data::receipt(serialized) => {
                        success
                            .set_result(serde_json::from_str::<Receipt>(&serialized).ok().map_or(
                                ResponseResult::Null,
                                ResponseResult::GetTransactionReceipt,
                            ))
                            .output()
                    }
                    Response_oneof_data::transaction_count(x) => success
                        .set_result(ResponseResult::GetTransactionCount(x.into()))
                        .output(),
                    Response_oneof_data::contract_code(x) => success
                        .set_result(ResponseResult::GetCode(x.into()))
                        .output(),
                    Response_oneof_data::contract_abi(x) => success
                        .set_result(ResponseResult::GetAbi(x.into()))
                        .output(),
                    Response_oneof_data::balance(x) => success
                        .set_result(ResponseResult::GetBalance(x.as_slice().into()))
                        .output(),
                    Response_oneof_data::filter_id(id) => {
                        // NewFilter, NewBlockFilter
                        success
                            .set_result(ResponseResult::NewFilter(id.into()))
                            .output()
                    }
                    Response_oneof_data::uninstall_filter(is_uninstall) => success
                        .set_result(ResponseResult::UninstallFilter(is_uninstall.into()))
                        .output(),
                    Response_oneof_data::filter_changes(data) => {
                        serde_json::from_str::<FilterChanges>(&data)
                            .map(|changes| {
                                success
                                    .set_result(ResponseResult::GetFilterChanges(changes))
                                    .output()
                            })
                            .unwrap_or_else(|_| Output::system_error(0))
                    }
                    Response_oneof_data::filter_logs(log) => serde_json::from_str::<Vec<Log>>(&log)
                        .map(|log| {
                            success
                                .set_result(ResponseResult::GetFilterLogs(log))
                                .output()
                        })
                        .unwrap_or_else(|_| Output::system_error(0)),
                    Response_oneof_data::transaction_proof(proof) => success
                        .set_result(ResponseResult::GetTransactionProof(proof.into()))
                        .output(),
                    Response_oneof_data::error_msg(err_msg) => Output::Failure(
                        RpcFailure::from_options(info, Error::server_error(code, err_msg)),
                    ),
                    Response_oneof_data::meta_data(data) => serde_json::from_str::<MetaData>(&data)
                        .map(|data| {
                            success
                                .set_result(ResponseResult::GetMetaData(data))
                                .output()
                        })
                        .unwrap_or_else(|_| Output::system_error(0)),
                    Response_oneof_data::state_proof(data) => success
                        .set_result(ResponseResult::GetStateProof(data.into()))
                        .output(),
                    Response_oneof_data::block_header(data) => success
                        .set_result(ResponseResult::GetBlockHeader(data.into()))
                        .output(),
                    Response_oneof_data::storage_value(data) => success
                        .set_result(ResponseResult::GetStorageAt(data.into()))
                        .output(),
                    Response_oneof_data::software_version(data) => {
                        serde_json::from_str::<SoftwareVersion>(&data)
                            .map(|data| {
                                success
                                    .set_result(ResponseResult::GetVersion(data))
                                    .output()
                            })
                            .unwrap_or_else(|_| Output::system_error(0))
                    }
                    Response_oneof_data::peers_info(data) => {
                        serde_json::from_str::<PeersInfo>(&data)
                            .map(|data| {
                                success.set_result(ResponseResult::PeersInfo(data)).output()
                            })
                            .unwrap_or_else(|_| Output::system_error(0))
                    }
                    Response_oneof_data::estimate_quota(x) => success
                        .set_result(ResponseResult::EstimateQuota(x.as_slice().into()))
                        .output(),
                    Response_oneof_data::license_info(data) => {
                        serde_json::from_str::<LicenseInfo>(&data)
                            .map(|data| {
                                success
                                    .set_result(ResponseResult::LicenseInfo(data))
                                    .output()
                            })
                            .unwrap_or_else(|_| Output::system_error(0))
                    }
                    Response_oneof_data::pool_tx_num(data) => success
                        .set_result(ResponseResult::GetPoolTxNum(PoolTxNum { num: data }))
                        .output(),
                    Response_oneof_data::censor_address(data) => success
                        .set_result(ResponseResult::OpCensoredAddress(data.into()))
                        .output(),
                }
            } else {
                match response {
                    Response_oneof_data::error_msg(err_msg) => Output::Failure(
                        RpcFailure::from_options(info, Error::server_error(code, err_msg)),
                    ),
                    _ => {
                        error!(
                            "system error, code[{}] != 0, response is not an error",
                            code
                        );
                        Output::system_error(code)
                    }
                }
            }
        } else {
            error!("system error, code = {}, data is empty", code);
            Output::system_error(code)
        }
    }
}
