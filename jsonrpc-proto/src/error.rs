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

use std::fmt::Debug;

use jsonrpc_types::Error;

const ERR_CODE_DECODE_ERROR: i64 = 500;

const ERR_MSG_BLOCK_DECODE_ERROR: &str = "chain block decode error";

pub trait ErrorExt {
    fn rpc_block_decode_error(err: Box<Debug>) -> Error {
        error!("jsonrpc_proto: fail to decode block from chain {:?}", err);
        Error::server_error(ERR_CODE_DECODE_ERROR, ERR_MSG_BLOCK_DECODE_ERROR)
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
