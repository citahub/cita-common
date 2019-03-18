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

use jsonrpc_types::{
    request::{RequestInfo, ResponseResult},
    response::{Output, RpcFailure, RpcSuccess},
    rpctypes::{
        Block, FilterChanges, Log, MetaData, Receipt, RpcBlock, RpcTransaction, SoftwareVersion,
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
                    Response_oneof_data::call_result(x) => {
                        success.set_result(ResponseResult::Call(x.into())).output()
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
                                    .set_result(ResponseResult::GetVersion(data.into()))
                                    .output()
                            })
                            .unwrap_or_else(|_| Output::system_error(0))
                    }
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
