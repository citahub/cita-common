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

use cita_types::{clean_0x, traits::LowerHex};
use jsonrpc_types::rpc_types::Integer;
use jsonrpc_types::{
    rpc_request::*, // bring in varied Params
    rpc_types::{BlockParamsByHash, BlockParamsByNumber, CountOrCode},
    Error,
};
use libproto::{
    request::Request as ProtoRequest, CensorAddress, TransactionParam, UnverifiedTransaction,
};
use serde_json;

use crate::from_into::TryIntoProto;

pub trait SendRawTransactionParamsExt {
    fn extract_unverified_tx(data: &[u8]) -> Result<UnverifiedTransaction, Error>;
}

fn create_request() -> ProtoRequest {
    let request_id = uuid::Uuid::new_v4().as_bytes().to_vec();
    let mut request = ProtoRequest::new();

    request.set_request_id(request_id);
    request
}

impl TryIntoProto<ProtoRequest> for BlockNumberParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_block_number(true);
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for GetVersionParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_software_version(true);
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for PeerCountParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_peercount(true);
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for PeersInfoParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_peers_info(self.0.into());
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for LicenseInfoParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_license_info(true);
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for SendRawTransactionParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let data: Vec<u8> = self.0.into();

        // SendRawTransactionParamsExt
        match SendRawTransactionParams::extract_unverified_tx(&data[..]) {
            Ok(un_tx) => {
                request.set_un_tx(un_tx);
                Ok(request)
            }
            Err(err) => Err(err),
        }
    }
}

impl TryIntoProto<ProtoRequest> for SendTransactionParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let data: Vec<u8> = self.0.into();

        // SendRawTransactionParamsExt
        match SendRawTransactionParams::extract_unverified_tx(&data[..]) {
            Ok(un_tx) => {
                request.set_un_tx(un_tx);
                Ok(request)
            }
            Err(err) => Err(err),
        }
    }
}

impl TryIntoProto<ProtoRequest> for GetBlockByHashParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        serde_json::to_string(&BlockParamsByHash::new(self.0.into(), self.1.into()))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|block_hash| {
                request.set_block_by_hash(block_hash);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for GetBlockByNumberParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        serde_json::to_string(&BlockParamsByNumber::new(self.0, self.1.into()))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|block_height| {
                request.set_block_by_height(block_height);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for GetTransactionReceiptParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_transaction_receipt(self.0.into());
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for GetLogsParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        serde_json::to_string(&self.0)
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|filter| {
                request.set_filter(filter);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for CallParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let mut call = libproto::Call::new();

        call.set_from(self.0.from.unwrap_or_default().into());
        call.set_to(self.0.to.into());
        call.set_data(self.0.data.unwrap_or_default().into());
        call.set_extra(self.2.into());

        serde_json::to_string(&self.1)
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|height| {
                call.set_height(height);
                request.set_call(call);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for GetTransactionParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let mut param = TransactionParam::new();
        param.set_hash(self.0.into());
        param.set_find_in_pool(self.1.into());

        request.set_transaction(param);
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for GetTransactionCountParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        serde_json::to_string(&CountOrCode::new(self.0.into(), self.1))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|jsonstr| {
                request.set_transaction_count(jsonstr);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for GetCodeParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        serde_json::to_string(&CountOrCode::new(self.0.into(), self.1))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|jsonstr| {
                request.set_code(jsonstr);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for GetAbiParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        serde_json::to_string(&CountOrCode::new(self.0.into(), self.1))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|jsonstr| {
                request.set_abi(jsonstr);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for GetBalanceParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        serde_json::to_string(&CountOrCode::new(self.0.into(), self.1))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|jsonstr| {
                request.set_balance(jsonstr);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for NewFilterParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let filter = serde_json::to_string(&self.0)
            .map_err(|err| Error::invalid_params(format!("{:?}", err)))?;

        request.set_new_filter(filter);
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for NewBlockFilterParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_new_block_filter(true);
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for UninstallFilterParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_uninstall_filter(self.0.into());
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for GetFilterChangesParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_filter_changes(self.0.into());
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for GetFilterLogsParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_filter_logs(self.0.into());
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for GetTransactionProofParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_transaction_proof(self.0.into());
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for GetMetaDataParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        serde_json::to_string(&self.0)
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|data| {
                request.set_meta_data(data);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for GetStateProofParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let mut state_proof = libproto::StateProof::new();

        state_proof.set_address(self.0.into());
        state_proof.set_position(self.1.into());

        serde_json::to_string(&self.2)
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|height| {
                state_proof.set_height(height);
                request.set_state_proof(state_proof);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for GetBlockHeaderParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        serde_json::to_string(&self.0)
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|height| {
                request.set_block_header_height(height);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for GetStorageKeyParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let mut skey = libproto::StorageKey::new();

        skey.set_address(self.0.into());
        skey.set_position(self.1.into());

        serde_json::to_string(&self.2)
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|height| {
                skey.set_height(height);
                request.set_storage_key(skey);
                request
            })
    }
}

impl TryIntoProto<ProtoRequest> for EstimateQuotaParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let mut call = libproto::Call::new();

        call.set_from(self.0.from.unwrap_or_default().into());
        call.set_to(self.0.to.into());
        call.set_data(self.0.data.unwrap_or_default().into());

        serde_json::to_string(&self.1)
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|height| {
                call.set_height(height);
                request.set_estimate_quota(call);
                request
            })
    }
}

impl SendRawTransactionParamsExt for SendRawTransactionParams {
    fn extract_unverified_tx(data: &[u8]) -> Result<UnverifiedTransaction, Error> {
        use libproto::TryFrom;
        use rustc_serialize::hex::FromHex;

        let un_tx = <UnverifiedTransaction as TryFrom<&[u8]>>::try_from(data).map_err(|_err| {
            let err_msg = format!(
                "parse protobuf UnverifiedTransaction data error : {:?}",
                _err
            );
            Error::parse_error_with_message(err_msg)
        })?;
        {
            let tx = un_tx.get_transaction();
            let version = tx.get_version();
            if version == 0 {
                let to = clean_0x(tx.get_to());
                if to.len() != 40 && !to.is_empty() {
                    return Err(Error::invalid_params(
                        "param 'to' has invalid length, expected 40, or are you creating contract?",
                    ));
                } else {
                    let _ = to.from_hex().map_err(|err| {
                        let err_msg = format!("param not hex string : {:?}", err);
                        Error::parse_error_with_message(err_msg)
                    })?;
                }
                trace!(
                    "SEND ProtoTransaction: nonce {:?}, block_limit {:?}, data {}, quota {:?}, to {:?}, hash {}",
                    tx.get_nonce(),
                    tx.get_valid_until_block(),
                    tx.get_data().lower_hex(),
                    tx.get_quota(),
                    tx.get_to(),
                    un_tx.crypt_hash().lower_hex()
                );
            } else if version < 3 {
                let to = tx.get_to_v1();
                if to.len() != 20 && !to.is_empty() {
                    return Err(Error::invalid_params(
                        "param 'to' has invalid length, expected 40, or are you creating contract?",
                    ));
                }
            } else {
                error!("unexpected version {}!", version);
                return Err(Error::invalid_params("param 'version' is unexpected"));
            }
        }
        Ok(un_tx)
    }
}

impl TryIntoProto<ProtoRequest> for GetPoolTxNumParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_pool_tx_num(true);
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for OpCensoredAddressParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let mut censor_address = CensorAddress::new();

        censor_address.address = self.1.into();
        match self.0.into() {
            1 => censor_address.action = 1,
            2 => censor_address.action = 2,
            _ => {
                return Err(Error::invalid_params(
                    "param 'action' is invalid, expected 1 or 2",
                ))
            }
        }

        request.set_censor_address(censor_address);
        Ok(request)
    }
}

impl TryIntoProto<ProtoRequest> for GetCensoredAddrsParams {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();

        request.set_query_censor_addrs(true);
        Ok(request)
    }
}
