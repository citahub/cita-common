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

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;

use super::request::PartialRequest;

/// Represents jsonrpc request.
#[derive(Debug, Clone, PartialEq)]
pub enum RpcRequest {
    /// Single request
    Single(PartialRequest),
    /// Batch of requests
    Batch(Vec<PartialRequest>),
}

impl Serialize for RpcRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RpcRequest::Single(ref req) => req.serialize(serializer),
            RpcRequest::Batch(ref reqs) => reqs.serialize(serializer),
        }
    }
}

impl<'a> Deserialize<'a> for RpcRequest {
    fn deserialize<D>(deserializer: D) -> Result<RpcRequest, D::Error>
    where
        D: Deserializer<'a>,
    {
        let v: serde_json::Value = Deserialize::deserialize(deserializer)?;
        serde_json::from_value(v.clone())
            .map(RpcRequest::Batch)
            .or_else(|_| serde_json::from_value(v).map(RpcRequest::Single))
            .map_err(|_| D::Error::custom("parse rpcrequest failed")) // unreachable, but types must match
    }
}
