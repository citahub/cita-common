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

use std::fmt::Debug;

use jsonrpc_types::Error;

const ERR_CODE_INTERNAL_ERROR: i64 = 500;

const ERR_MSG_BLOCK_DECODE_ERROR: &str = "chain block decode error";
const ERR_MSG_TX_CONTENT_ENCODE_ERROR: &str = "transaction content encode error";

pub trait ErrorExt {
    fn rpc_block_decode_error(err: Box<dyn Debug>) -> Error {
        error!("jsonrpc_proto: fail to decode block from chain {:?}", err);
        Error::server_error(ERR_CODE_INTERNAL_ERROR, ERR_MSG_BLOCK_DECODE_ERROR)
    }

    fn bft_proof_decode_error(err: Box<bincode::ErrorKind>) -> Error {
        error!("jsonrpc_proto: fail to decode service bft proof {}", err);
        Error::server_error(ERR_CODE_INTERNAL_ERROR, err.to_string())
    }

    fn transaction_data_encode_error(err: libproto::TryIntoConvertError) -> Error {
        error!("jsonrpc_proto: fail to encode content {:?}", err);
        Error::server_error(ERR_CODE_INTERNAL_ERROR, ERR_MSG_TX_CONTENT_ENCODE_ERROR)
    }
}

impl ErrorExt for Error {}
