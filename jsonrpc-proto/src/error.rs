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

const BLOCK_DECODE_ERROR_CODE: i64 = 500;
const BLOCK_DECODE_ERROR_MSG: &str = "chain block decode error";

pub trait ErrorExt<E: Debug> {
    fn rpc_block_decode_error(inner: E) -> Error {
        error!("jsonrpc_proto: fail to decode block from chain {:?}", inner);
        Error::server_error(BLOCK_DECODE_ERROR_CODE, BLOCK_DECODE_ERROR_MSG)
    }
}

impl<E: Debug> ErrorExt<E> for Error {}
