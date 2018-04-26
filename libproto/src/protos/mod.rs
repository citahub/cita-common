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

// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

pub mod auth;
pub mod blockchain;
pub mod citacode;
pub mod communication;
pub mod consensus;
pub mod executor;
pub mod request;
pub mod response;
pub mod snapshot;
pub mod sync;

pub use self::auth::{Ret, BlockTxHashes, BlockTxHashesReq, Miscellaneous, MiscellaneousReq, VerifyBlockReq, VerifyBlockResp, VerifyTxReq, VerifyTxResp};
pub use self::blockchain::{Crypto, ProofType, AccountGasLimit, Block, BlockBody, BlockHeader, BlockTxs, BlockWithProof, Proof, RichStatus, SignedTransaction, Status, Transaction, UnverifiedTransaction};
pub use self::citacode::{ActionParams, EnvInfo, InvokeRequest, InvokeResponse, KV, Log};
pub use self::communication::{InnerMessage_oneof_content, InnerMessage};
pub use self::consensus::{Proposal, SignedProposal, Vote};
pub use self::executor::{ReceiptError, ConsensusConfig, ExecutedHeader, ExecutedInfo, ExecutedResult, LoadRequest, LoadResponse, LogEntry, Receipt, ReceiptErrorWithOption, ReceiptWithOption, RegisterRequest, RegisterResponse, StateRoot};
pub use self::request::{BlockTag, Request_oneof_req, BatchRequest, Call, Request};
pub use self::response::{Response_oneof_data, FullTransaction, Response};
pub use self::snapshot::{Cmd, Resp, SnapshotReq, SnapshotResp};
pub use self::sync::{SyncRequest, SyncResponse};

// For gprc
pub mod citacode_grpc;
pub use self::citacode_grpc::{CitacodeServiceClient, CitacodeService};
pub mod executor_grpc;
pub use self::executor_grpc::{ExecutorServiceClient, ExecutorService};
