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

/// Convert JSON-RPC request to proto request.
use jsonrpc_types::{
    request::*, // bring in varied Params
    Error,
};
use libproto::request::Request as ProtoRequest;

use crate::from_into::TryIntoProto;

pub mod params;

impl TryIntoProto<ProtoRequest> for Request {
    type Error = Error;

    fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
        self.call.try_into_proto()
    }
}

macro_rules! call_into_proto {
    [$( $enum_name:ident ),+] => {
        impl TryIntoProto<ProtoRequest> for Call {
            type Error = Error;

            fn try_into_proto(self) -> Result<ProtoRequest, Self::Error> {
                match self {
                    $(
                        Call::$enum_name{ params } => params.try_into_proto(),
                    )+
                }
            }
        }
    };
}

call_into_proto![
    BlockNumber,
    PeerCount,
    SendRawTransaction,
    SendTransaction,
    GetBlockByHash,
    GetBlockByNumber,
    GetTransactionReceipt,
    GetLogs,
    Call,
    GetTransaction,
    GetTransactionCount,
    GetCode,
    GetAbi,
    GetBalance,
    NewFilter,
    NewBlockFilter,
    UninstallFilter,
    GetFilterChanges,
    GetFilterLogs,
    GetTransactionProof,
    GetMetaData,
    GetStateProof,
    GetBlockHeader,
    GetStorageAt
];
