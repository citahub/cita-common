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

use std::convert::{From, Into};
use std::fmt;

#[macro_export]
macro_rules! routing_key {
    ([$( $sm:tt >> $mt:tt ),+ ,]) => {{
        routing_key![[ $( $sm >> $mt ),+ ]]
    }};
    ([$( $sm:tt >> $mt:tt ),+]) => {{
        vec![ $( routing_key!($sm >> $mt) ),+ ]
    }};
    ($sm:ident >> $mt:ident) => {
        RoutingKey (
            SubModules::$sm,
            MsgType::$mt,
        )
    };
}

const UNKNOWN: &str = "__unknown__";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SubModules {
    Jsonrpc,
    Net,
    Chain,
    Consensus,
    Auth,
    Executor,
    Synchronizer,
    Snapshot,
    All,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MsgType {
    // Generate MSG-PROTOS struct automatically begin:
    RawBytes,
    Request,
    Response,
    SyncRequest,
    SyncResponse,
    Status,
    RichStatus,
    SignedProposal,
    Block,
    BlockWithProof,
    BlockHeader,
    BlockTxs,
    BlockTxHashes,
    BlockTxHashesReq,
    VerifyBlockReq,
    VerifyBlockResp,
    ExecutedResult,
    SnapshotReq,
    SnapshotResp,
    Miscellaneous,
    MiscellaneousReq,
    BlackList,
    StateSignal,
    GetBlockTxn,
    BlockTxn,
    CompactSignedProposal,
    GetTxList,
    // Generate MSG-PROTOS struct automatically end.
    All,
    Unknown,
    // TODO This is a issue left over by history.
    //      The Request is too big (send from Jsonrpc).
    //      To remove follow items should be better.
    RequestNewTx,
    RequestNewTxBatch,
    RequestNet,
    LocalSync,
    RequestRpc,
    RequestPeersInfo,
    GetCrl,
    GetCrlResp,
    InvalidLicense,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoutingKey(pub SubModules, pub MsgType);

impl RoutingKey {
    pub fn set_sub_module(&mut self, sm: SubModules) {
        self.0 = sm;
    }
    pub fn set_msg_type(&mut self, mt: MsgType) {
        self.1 = mt;
    }
    pub fn get_sub_module(self) -> SubModules {
        self.0
    }
    pub fn get_msg_type(self) -> MsgType {
        self.1
    }
    pub fn is_sub_module(self, sm: SubModules) -> bool {
        self.0 == sm
    }
    pub fn is_msg_type(self, mt: MsgType) -> bool {
        self.1 == mt
    }
}

pub const SUBMODULES_UNKNOWN: SubModules = SubModules::Unknown;
pub const MSGTYPE_UNKNOWN: MsgType = MsgType::Unknown;
pub const ROUTINGKEY_UNKNOWN: RoutingKey = RoutingKey(SUBMODULES_UNKNOWN, MSGTYPE_UNKNOWN);

impl fmt::Display for SubModules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Please use the same rules to name the string
                SubModules::Jsonrpc => "jsonrpc",
                SubModules::Net => "net",
                SubModules::Chain => "chain",
                SubModules::Consensus => "consensus",
                SubModules::Auth => "auth",
                SubModules::Executor => "executor",
                SubModules::Synchronizer => "synchronizer",
                SubModules::Snapshot => "snapshot",
                SubModules::All => "*",
                SubModules::Unknown => UNKNOWN,
            }
        )
    }
}

impl fmt::Display for MsgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Generate MSG-PROTOS display automatically begin:
                &MsgType::RawBytes => "raw_bytes",
                &MsgType::Request => "request",
                &MsgType::Response => "response",
                &MsgType::SyncRequest => "sync_request",
                &MsgType::SyncResponse => "sync_response",
                &MsgType::Status => "status",
                &MsgType::RichStatus => "rich_status",
                &MsgType::SignedProposal => "signed_proposal",
                &MsgType::Block => "block",
                &MsgType::BlockWithProof => "block_with_proof",
                &MsgType::BlockHeader => "block_header",
                &MsgType::BlockTxs => "block_txs",
                &MsgType::BlockTxHashes => "block_tx_hashes",
                &MsgType::BlockTxHashesReq => "block_tx_hashes_req",
                &MsgType::VerifyBlockReq => "verify_block_req",
                &MsgType::VerifyBlockResp => "verify_block_resp",
                &MsgType::ExecutedResult => "executed_result",
                &MsgType::SnapshotReq => "snapshot_req",
                &MsgType::SnapshotResp => "snapshot_resp",
                &MsgType::Miscellaneous => "miscellaneous",
                &MsgType::MiscellaneousReq => "miscellaneous_req",
                &MsgType::BlackList => "black_list",
                &MsgType::StateSignal => "state_signal",
                &MsgType::GetBlockTxn => "get_block_txn",
                &MsgType::BlockTxn => "block_txn",
                &MsgType::CompactSignedProposal => "compact_signed_proposal",
                &MsgType::GetTxList => "get_tx_list",
                // Generate MSG-PROTOS display automatically end.
                MsgType::All => "*",
                MsgType::Unknown => UNKNOWN,
                MsgType::RequestNewTx => "req_new_tx",
                MsgType::RequestNewTxBatch => "req_new_tx_batch",
                MsgType::RequestNet => "req_net",
                MsgType::LocalSync => "local_sync",
                MsgType::RequestRpc => "req_rpc",
                MsgType::RequestPeersInfo => "req_peers_info",
                MsgType::GetCrl => "get_crl",
                MsgType::GetCrlResp => "get_crl_resp",
                MsgType::InvalidLicense => "invalid_license",
            }
        )
    }
}

impl fmt::Display for RoutingKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

impl<'a> From<&'a str> for SubModules {
    fn from(s: &'a str) -> SubModules {
        match s {
            "jsonrpc" => SubModules::Jsonrpc,
            "net" => SubModules::Net,
            "chain" => SubModules::Chain,
            "consensus" => SubModules::Consensus,
            "auth" => SubModules::Auth,
            "executor" => SubModules::Executor,
            "synchronizer" => SubModules::Synchronizer,
            "snapshot" => SubModules::Snapshot,
            "*" => SubModules::All,
            _ => SubModules::Unknown,
        }
    }
}

impl<'a> From<&'a str> for MsgType {
    fn from(s: &'a str) -> MsgType {
        match s {
            // Generate MSG-PROTOS from_str automatically begin:
            "raw_bytes" => MsgType::RawBytes,
            "request" => MsgType::Request,
            "response" => MsgType::Response,
            "sync_request" => MsgType::SyncRequest,
            "sync_response" => MsgType::SyncResponse,
            "status" => MsgType::Status,
            "rich_status" => MsgType::RichStatus,
            "signed_proposal" => MsgType::SignedProposal,
            "block" => MsgType::Block,
            "block_with_proof" => MsgType::BlockWithProof,
            "block_header" => MsgType::BlockHeader,
            "block_txs" => MsgType::BlockTxs,
            "block_tx_hashes" => MsgType::BlockTxHashes,
            "block_tx_hashes_req" => MsgType::BlockTxHashesReq,
            "verify_block_req" => MsgType::VerifyBlockReq,
            "verify_block_resp" => MsgType::VerifyBlockResp,
            "executed_result" => MsgType::ExecutedResult,
            "snapshot_req" => MsgType::SnapshotReq,
            "snapshot_resp" => MsgType::SnapshotResp,
            "miscellaneous" => MsgType::Miscellaneous,
            "miscellaneous_req" => MsgType::MiscellaneousReq,
            "black_list" => MsgType::BlackList,
            "state_signal" => MsgType::StateSignal,
            "get_block_txn" => MsgType::GetBlockTxn,
            "block_txn" => MsgType::BlockTxn,
            "compact_signed_proposal" => MsgType::CompactSignedProposal,
            "get_tx_list" => MsgType::GetTxList,
            // Generate MSG-PROTOS from_str automatically end.
            "*" => MsgType::All,
            "req_new_tx" => MsgType::RequestNewTx,
            "req_new_tx_batch" => MsgType::RequestNewTxBatch,
            "req_net" => MsgType::RequestNet,
            "local_sync" => MsgType::LocalSync,
            "req_rpc" => MsgType::RequestRpc,
            "req_peers_info" => MsgType::RequestPeersInfo,
            "get_crl" => MsgType::GetCrl,
            "get_crl_resp" => MsgType::GetCrlResp,
            "invalid_license" => MsgType::InvalidLicense,
            _ => MsgType::Unknown,
        }
    }
}

impl<'a> From<&'a str> for RoutingKey {
    fn from(s: &'a str) -> RoutingKey {
        let mut items = s.split('.').take(3);
        match (items.next(), items.next(), items.next()) {
            (Some(sm), Some(mt), None) => RoutingKey(SubModules::from(sm), MsgType::from(mt)),
            _ => ROUTINGKEY_UNKNOWN,
        }
    }
}

macro_rules! impl_some_traits {
    ($struct:ident) => {
        impl From<String> for $struct {
            fn from(s: String) -> $struct {
                $struct::from(s.as_str())
            }
        }

        impl<'a> From<&'a String> for $struct {
            fn from(s: &'a String) -> $struct {
                $struct::from(s.as_str())
            }
        }

        impl Into<String> for $struct {
            fn into(self) -> String {
                self.to_string()
            }
        }
    };
}

impl_some_traits!(SubModules);
impl_some_traits!(MsgType);
impl_some_traits!(RoutingKey);

#[cfg(test)]
mod tests {

    #[test]
    fn macros_works() {
        use super::{MsgType, RoutingKey, SubModules};
        let rk_ar = routing_key!(Auth >> Request);
        assert_eq!(rk_ar, RoutingKey(SubModules::Auth, MsgType::Request));
    }

    #[test]
    fn traits_from_works() {
        use super::{MsgType, RoutingKey, SubModules};
        use std::convert::From;
        let sm = SubModules::from("jsonrpc");
        assert_eq!(sm, SubModules::Jsonrpc);
        assert_eq!(sm.to_string().as_str(), "jsonrpc");
        let mt = MsgType::from("request");
        assert_eq!(mt, MsgType::Request);
        assert_eq!(mt.to_string().as_str(), "request");
        let mut rk = RoutingKey::from("jsonrpc.request");
        assert_eq!(rk.get_sub_module(), SubModules::Jsonrpc);
        assert_eq!(rk.get_msg_type(), MsgType::Request);
        assert_eq!(rk.to_string().as_str(), "jsonrpc.request");
        rk.set_sub_module(SubModules::Net);
        rk.set_msg_type(MsgType::Response);
        assert_eq!(rk.to_string().as_str(), "net.response");
        let rk_custom = RoutingKey::from("net.anything-is-ok");
        assert_eq!(rk_custom.to_string().as_str(), "net.__unknown__");
        assert!(rk_custom.is_sub_module(SubModules::Net));
        assert!(rk_custom.is_msg_type(MsgType::Unknown));
        assert!(!rk_custom.is_sub_module(SubModules::Jsonrpc));
        assert!(!rk_custom.is_msg_type(MsgType::All));
        let rk_custom = RoutingKey::from("anything-is-ok.*");
        assert_eq!(rk_custom.to_string().as_str(), "__unknown__.*");
        assert!(rk_custom.is_sub_module(SubModules::Unknown));
        assert!(rk_custom.is_msg_type(MsgType::All));
        assert!(!rk_custom.is_sub_module(SubModules::Jsonrpc));
        assert!(!rk_custom.is_msg_type(MsgType::Request));
        let rk_error = RoutingKey::from("an-unknown-string");
        assert_eq!(rk_error.to_string().as_str(), "__unknown__.__unknown__");
        let rk_error = RoutingKey::from("an-unknown.string");
        assert_eq!(rk_error.to_string().as_str(), "__unknown__.__unknown__");
        let rk_error = RoutingKey::from("an.unknown.string");
        assert_eq!(rk_error.to_string().as_str(), "__unknown__.__unknown__");
    }
}
