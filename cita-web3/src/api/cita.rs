// Copyright Cryptape Technologies LLC.
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

//! `Cita` namespace

use web3::api::Namespace;
use web3::helpers::CallFuture;
use web3::Transport;

use types::rpc_request::JsonRpcRequest;

/// `Cita` namespace
#[derive(Debug, Clone)]
pub struct Cita<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for Cita<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        Cita { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> Cita<T> {
    pub fn call<P>(&self, param: P) -> CallFuture<P::Response, T::Out>
    where
        P: JsonRpcRequest,
    {
        CallFuture::new(
            self.transport
                .execute(param.method_name(), param.value_vec()),
        )
    }
}
