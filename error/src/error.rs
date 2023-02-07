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
// L at last mean light node
//-32501             轻节点-无可用节点*
//-32502             轻节点-区块头未同步*
//-32503             轻节点-回执检查失败
//-32504             轻节点-未找到回执
//-32505             轻节点-未找到交易
//-32506             轻节点-未找到区块
//-32507             轻节点-未找到区块头
//-32508             轻节点-交易检查失败
//-32509             轻节点-状态数据查询失败
pub enum ErrorCode {
    QueryError,
    TxAuthError,
    TimeOut,
    NoneLinkableNodeL,
    NotSyncedHeaderL,
    ReceiptCheckErrorL,
    NotFoundReceiptL,
    NotFoundTransactionL,
    NotFoundBlockL,
    NotFoundBlockHeaderL,
    TransactionCheckErrorL,
    QueryErrorL,
}

impl ErrorCode {
    pub fn to_error_code(&self) -> i64 {
        match self {
            ErrorCode::QueryError => ErrorCode::query_error(),
            ErrorCode::TxAuthError => ErrorCode::tx_auth_error(),
            ErrorCode::TimeOut => ErrorCode::time_out_error(),
            ErrorCode::NoneLinkableNodeL => ErrorCode::no_linkable_node_l(),
            ErrorCode::NotSyncedHeaderL => ErrorCode::not_synced_header_l(),
            ErrorCode::ReceiptCheckErrorL => ErrorCode::receipt_check_error_l(),
            ErrorCode::NotFoundReceiptL => ErrorCode::not_found_receipt_l(),
            ErrorCode::NotFoundTransactionL => ErrorCode::not_found_transaction_l(),
            ErrorCode::NotFoundBlockL => ErrorCode::not_found_block_l(),
            ErrorCode::NotFoundBlockHeaderL => ErrorCode::not_found_block_header_l(),
            ErrorCode::TransactionCheckErrorL => ErrorCode::transaction_check_error_l(),
            ErrorCode::QueryErrorL => ErrorCode::query_error_l(),
        }
    }
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

    pub fn no_linkable_node_l() -> i64 {
        -32501
    }

    pub fn not_synced_header_l() -> i64 {
        -32502
    }

    pub fn receipt_check_error_l() -> i64 {
        -32503
    }

    pub fn not_found_receipt_l() -> i64 {
        -32504
    }

    pub fn not_found_transaction_l() -> i64 {
        -32505
    }

    pub fn not_found_block_l() -> i64 {
        -32506
    }

    pub fn not_found_block_header_l() -> i64 {
        -32507
    }

    pub fn transaction_check_error_l() -> i64 {
        -32508
    }

    pub fn query_error_l() -> i64 {
        -32509
    }
}
