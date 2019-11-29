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

//-32003             查询类错误
//-32006             交易认证类错误
//-32099             请求超时
pub enum ErrorCode {
    QueryError,
    TxAuthError,
    TimeOut,
}

impl ErrorCode {
    pub fn query_error() -> i64 {
        -32003
    }

    pub fn tx_auth_error() -> i64 {
        -32006
    }

    pub fn time_out_error() -> i64 {
        -32099
    }
}
