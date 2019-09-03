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

use std::vec::Vec;

use serde::de::Error as SError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{from_value, Value};

use crate::error::Error;
use crate::rpc_request::{RequestInfo, ResponseResult};
use crate::rpc_types::{Id, Version};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RpcFailure {
    pub jsonrpc: Option<Version>,
    pub id: Id,
    pub error: Error,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RpcSuccess {
    pub jsonrpc: Option<Version>,
    pub id: Id,
    pub result: ResponseResult,
}

impl RpcSuccess {
    pub fn new(info: RequestInfo) -> Self {
        RpcSuccess {
            jsonrpc: info.jsonrpc,
            id: info.id,
            result: ResponseResult::default(),
        }
    }

    pub fn set_result(mut self, reuslt: ResponseResult) -> Self {
        self.result = reuslt;
        self
    }

    pub fn output(self) -> Output {
        Output::Success(Box::new(self))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Output {
    /// Success
    Success(Box<RpcSuccess>),
    /// Failure
    Failure(RpcFailure),
}

impl Output {
    /// Creates new failure output indicating malformed request.
    pub fn invalid_request(info: RequestInfo) -> Self {
        Output::Failure(RpcFailure::from_options(info, Error::invalid_request()))
    }

    /// Creates a system error
    pub fn system_error(code: i64) -> Self {
        Output::Failure(RpcFailure::from(Error::server_error(code, "system error")))
    }
}

impl<'a> Deserialize<'a> for Output {
    fn deserialize<D>(deserializer: D) -> Result<Output, D::Error>
    where
        D: Deserializer<'a>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        from_value(v.clone())
            .map(Output::Failure)
            .or_else(|_| from_value(v).map(Output::Success))
            .map_err(|_| D::Error::custom("")) // types must match
    }
}

impl Serialize for Output {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Output::Success(ref s) => s.serialize(serializer),
            Output::Failure(ref f) => f.serialize(serializer),
        }
    }
}

impl From<Error> for RpcFailure {
    fn from(err: Error) -> Self {
        RpcFailure::from_options(RequestInfo::null(), err)
    }
}

impl RpcFailure {
    pub fn from_options(info: RequestInfo, err: Error) -> Self {
        RpcFailure {
            jsonrpc: info.jsonrpc,
            id: info.id,
            error: err,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RpcResponse {
    /// Single response
    Single(Box<Output>),
    /// Response to batch request (batch of responses)
    Batch(Vec<Output>),
}

impl<'a> Deserialize<'a> for RpcResponse {
    fn deserialize<D>(deserializer: D) -> Result<RpcResponse, D::Error>
    where
        D: Deserializer<'a>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        from_value(v.clone())
            .map(RpcResponse::Batch)
            .or_else(|_| from_value(v).map(RpcResponse::Single))
            .map_err(|_| D::Error::custom("")) // types must match
    }
}

impl Serialize for RpcResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RpcResponse::Single(ref o) => o.serialize(serializer),
            RpcResponse::Batch(ref b) => b.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RpcSuccess;
    use crate::rpc_request::{RequestInfo, ResponseResult};
    use crate::rpc_types::{Id, Version};
    use serde_json;

    #[test]
    fn test_rpc_deserialize() {
        let rpc = RpcSuccess::new(RequestInfo::new(Some(Version::V2), Id::Num(2)))
            .set_result(ResponseResult::Null);

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":2,"result":null}"#);
    }

    #[test]
    fn test_rpc_deserialize2() {
        let rpc = RpcSuccess::new(RequestInfo::new(
            Some(Version::V2),
            Id::Str("2".to_string()),
        ))
        .set_result(ResponseResult::BlockNumber(3u64.into()));

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":"2","result":"0x3"}"#);
    }
}
