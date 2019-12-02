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

/// Convert JSON-RPC request to proto request.
use jsonrpc_types::{
    rpc_request::{Call, Request},
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
    ($( ($enum_name:ident, $params_name:ident: $params_list:expr, $result_type:ident) ),+ ,) => {
        call_into_proto![$( $enum_name ),+];
    };
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

impl_for_each_jsonrpc_requests!(call_into_proto);
