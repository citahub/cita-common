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

//! jsonrpc errors

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use serde_json::{self, Value};

/// JSONRPC error code
#[derive(Debug, PartialEq, Clone)]
pub enum ErrorCode {
    /// Invalid JSON was received by the server.
    /// An error occurred on the server while parsing the JSON text.
    ParseError,
    /// The JSON sent is not a valid Request object.
    InvalidRequest,
    /// The method does not exist / is not available.
    MethodNotFound,
    /// Invalid method parameter(s).
    InvalidParams,
    /// Internal JSON-RPC error.
    InternalError,
    /// Reserved for implementation-defined server-errors.
    ServerError(i64),
}

impl ErrorCode {
    /// Returns integer code value
    pub fn code(&self) -> i64 {
        match *self {
            ErrorCode::ParseError => -32_700,
            ErrorCode::InvalidRequest => -32_600,
            ErrorCode::MethodNotFound => -32_601,
            ErrorCode::InvalidParams => -32_602,
            ErrorCode::InternalError => -32_603,
            ErrorCode::ServerError(code) => code,
        }
    }

    /// Returns human-readable description
    pub fn description(&self) -> String {
        let desc = match *self {
            ErrorCode::ParseError => "Parse error",
            ErrorCode::InvalidRequest => "Invalid request",
            ErrorCode::MethodNotFound => "Method not found",
            ErrorCode::InvalidParams => "Invalid params",
            ErrorCode::InternalError => "Internal error",
            ErrorCode::ServerError(_) => "Server error",
        };
        desc.to_string()
    }
}

impl<'a> Deserialize<'a> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<ErrorCode, D::Error>
    where
        D: Deserializer<'a>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        match v.as_i64() {
            Some(-32_700) => Ok(ErrorCode::ParseError),
            Some(-32_600) => Ok(ErrorCode::InvalidRequest),
            Some(-32_601) => Ok(ErrorCode::MethodNotFound),
            Some(-32_602) => Ok(ErrorCode::InvalidParams),
            Some(-32_603) => Ok(ErrorCode::InternalError),
            Some(code) => Ok(ErrorCode::ServerError(code)),
            _ => unreachable!(),
        }
    }
}

impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.code())
    }
}

/// Error object as defined in Spec
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Error {
    /// Code
    pub code: ErrorCode,
    /// Message
    pub message: String,
    /// Optional data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl Error {
    /// Wraps given `ErrorCode`
    pub fn new(code: ErrorCode) -> Self {
        Error {
            message: code.description(),
            code,
            data: None,
        }
    }

    /// Creates new `ParseError`
    pub fn parse_error() -> Self {
        Self::new(ErrorCode::ParseError)
    }

    /// Creates new `InvalidRequest`
    pub fn invalid_request() -> Self {
        Self::new(ErrorCode::InvalidRequest)
    }

    /// Creates new `MethodNotFound`
    pub fn method_not_found() -> Self {
        Self::new(ErrorCode::MethodNotFound)
    }

    /// Creates new `InvalidParams`
    pub fn invalid_params<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Error {
            code: ErrorCode::InvalidParams,
            message: message.into(),
            data: None,
        }
    }

    /// Creates new `InternalError`
    pub fn internal_error() -> Self {
        Self::new(ErrorCode::InternalError)
    }

    /// Creates new `InvalidRequest` with invalid version description
    pub fn invalid_version() -> Self {
        Error {
            code: ErrorCode::InvalidRequest,
            message: "Unsupported JSON-RPC protocol version".to_owned(),
            data: None,
        }
    }

    pub fn server_error<M>(err_code: i64, message: M) -> Self
    where
        M: Into<String>,
    {
        Error {
            code: ErrorCode::ServerError(err_code),
            message: message.into(),
            data: None,
        }
    }

    pub fn parse_error_with_message<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Error {
            code: ErrorCode::ParseError,
            message: message.into(),
            data: None,
        }
    }

    /// Creates new `InvalidParams`
    pub fn invalid_params_len() -> Self {
        Error {
            code: ErrorCode::InvalidParams,
            message: "Invalid JSON-RPC params length".to_owned(),
            data: None,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error {
            code: ErrorCode::ParseError,
            message: err.to_string(),
            data: None,
        }
    }
}
