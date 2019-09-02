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

#[macro_use]
extern crate serde_derive;
#[cfg(not(test))]
extern crate serde_json;
#[cfg(test)]
#[macro_use]
extern crate serde_json;

pub extern crate jsonrpc_types_internals as internals;

#[macro_use]
mod macros;

mod error;
pub use crate::error::{Error, ErrorCode};
pub mod rpc_request;
pub mod rpc_response;
pub mod rpc_types;
