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

//! `Cita` namespace

use web3::api::Namespace;
use web3::helpers::CallResult;
use web3::Transport;

use types::request;

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
    pub fn call<P>(&self, param: P) -> CallResult<P::Response, T::Out>
    where
        P: request::JsonRpcRequest,
    {
        CallResult::new(
            self.transport
                .execute(param.method_name(), param.value_vec()),
        )
    }
}
