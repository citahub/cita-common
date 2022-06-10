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

pub use crate::protos::InnerMessage_oneof_content as MsgClass;
use crate::protos::*;
use protobuf::{parse_from_bytes, Message as MessageTrait};
use snappy;
use std::convert::{From, Into};

pub use std::u32::MAX as ZERO_ORIGIN;

#[derive(Copy, Clone, Debug, Default)]
pub struct TryFromConvertError(());

#[derive(Copy, Clone, Debug, Default)]
pub struct TryIntoConvertError(());

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OperateType {
    Broadcast = 0,
    Single = 1,
    Subtract = 2,
}

pub const DEFAULT_OPERATE_TYPE: OperateType = OperateType::Broadcast;

pub trait TryFrom<T>
where
    Self: ::std::marker::Sized,
{
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T> {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

impl TryFrom<u8> for OperateType {
    type Error = TryFromConvertError;
    #[inline]
    fn try_from(ot: u8) -> Result<Self, TryFromConvertError> {
        match ot {
            0 => Ok(OperateType::Broadcast),
            1 => Ok(OperateType::Single),
            2 => Ok(OperateType::Subtract),
            // Default
            _ => Err(TryFromConvertError(())),
        }
    }
}

pub type Origin = u32;
pub type RawBytes = Vec<u8>;

impl InnerMessage {
    // Param is passed by value, moved
    pub fn set_content(&mut self, v: MsgClass) {
        self.content = Some(v);
    }

    pub fn take_content(&mut self) -> Option<MsgClass> {
        self.content.take()
    }
}

impl From<MsgClass> for InnerMessage {
    fn from(mc: MsgClass) -> Self {
        let mut im = InnerMessage::new();
        im.set_content(mc);
        im
    }
}

macro_rules! impl_convert_for_struct {
    ($( $struct:ident, )+) => {
        $(
            impl_convert_for_struct!($struct);
        )+
    };
    ($struct:ident) => {

        impl<'a> TryFrom<&'a [u8]> for $struct {
            type Error = TryFromConvertError;
            fn try_from(b: &[u8]) -> Result<Self, TryFromConvertError> {
                parse_from_bytes::<$struct>(b).map_err(|_| { TryFromConvertError(()) })
            }
        }

        impl<'a> TryFrom<&'a Vec<u8>> for $struct {
            type Error = TryFromConvertError;
            fn try_from(v: &Vec<u8>) -> Result<Self, TryFromConvertError> {
                parse_from_bytes::<$struct>(v.as_slice()).map_err(|_| { TryFromConvertError(()) })
            }
        }

       impl<'a> TryInto<Vec<u8>> for &'a $struct {
           type Error = TryIntoConvertError;
           fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
               self.write_to_bytes().map_err(|_| { TryIntoConvertError(()) })
           }
       }

       impl TryInto<Vec<u8>> for $struct {
           type Error = TryIntoConvertError;
           fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
               self.write_to_bytes().map_err(|_| { TryIntoConvertError(()) })
           }
       }
    };
}

macro_rules! impl_convert_for_struct_in_msg {
    ($( $struct:ident, )+) => {
        $(
            impl_convert_for_struct_in_msg!($struct);
        )+
    };
    ($struct:ident) => {

       impl Into<MsgClass> for $struct {
           fn into(self) -> MsgClass {
               MsgClass::$struct(self)
           }
       }

        impl Into<Message> for $struct {
            fn into(self) -> Message {
                Message::init(DEFAULT_OPERATE_TYPE, ZERO_ORIGIN, self.into())
            }
        }
    };
}

macro_rules! loop_macro_for_structs {
    ($macro:ident) => {
        $macro!(
            // Generate ALL-PROTOS automatically begin:
            BlockTxHashes,
            BlockTxHashesReq,
            GetTxList,
            Miscellaneous,
            MiscellaneousReq,
            VerifyBlockReq,
            VerifyBlockResp,
            VerifyTxReq,
            AccountGasLimit,
            BlackList,
            Block,
            BlockBody,
            BlockHeader,
            BlockTxs,
            BlockWithProof,
            CompactBlock,
            CompactBlockBody,
            Proof,
            RichStatus,
            SignedTransaction,
            StateSignal,
            Status,
            Transaction,
            UnverifiedTransaction,
            InnerMessage,
            BlockTxn,
            GetBlockTxn,
            CompactProposal,
            CompactSignedProposal,
            Proposal,
            SignedProposal,
            Vote,
            AbiResponse,
            BlockResponse,
            CodeResponse,
            ConsensusConfig,
            ExecutedHeader,
            ExecutedInfo,
            ExecutedResult,
            LogEntry,
            Receipt,
            ReceiptErrorWithOption,
            ReceiptResponse,
            ReceiptWithOption,
            StateRoot,
            TrieID,
            TrieResponse,
            BatchRequest,
            Call,
            LightRequest,
            Request,
            StateProof,
            StorageKey,
            FullTransaction,
            LightResponse,
            Response,
            SnapshotReq,
            SnapshotResp,
            SyncLightRequest,
            SyncLightResponse,
            SyncRequest,
            SyncResponse,
            // Generate ALL-PROTOS automatically end.
        );
    };
}

macro_rules! loop_macro_for_structs_in_msg {
    ($macro:ident) => {
        $macro!(
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
            TrieResponse,
            ReceiptResponse,
            CodeResponse,
            AbiResponse,
            BlockResponse,
            SyncLightRequest,
            SyncLightResponse,
            LightRequest,
            LightResponse,
            // Generate MSG-PROTOS struct automatically end.
        );
    };
}

loop_macro_for_structs!(impl_convert_for_struct);
loop_macro_for_structs_in_msg!(impl_convert_for_struct_in_msg);

/// Message for communication between microservices or nodes.
///
/// # Message in Bytes:
///
/// +---------------------------------------------------------------+
/// | Bytes | Type | Function                                       |
/// |-------+------+------------------------------------------------|
/// |   0   |  u8  | Reserved (For Version?)                        |
/// |   1   |  u8  | Reserved                                       |
/// |   2   |  u8  | Reserved                                       |
/// |-------+------+------------------------------------------------|
/// |   3   |  u4  | Reserved                                       |
/// |       |  u1  | Reserved                                       |
/// |       |  u1  | Compress: true 1, false 0                      |
/// |       |  u2  | OperateType                                    |
/// |-------+------+------------------------------------------------|
/// |  4~7  |  u32 | Origin                                         |
/// |-------+------+------------------------------------------------|
/// |  8~   |      | Payload (Serialized Data with Compress)        |
/// +-------+------+------------------------------------------------+
///
/// We DO NOT have to known the contents of payloads (uncompress and deserialize them) if we just
/// want to distribute them.
/// So we use first 8 bytes to store `OperateType` and `Origin`.
/// And we uncompress and deserialize the payloads only before when we use the contents of them.

#[derive(Clone, Debug)]
pub struct Message {
    raw: Vec<u8>,
}

impl Message {
    pub fn init(operate: OperateType, origin: Origin, mc: MsgClass) -> Self {
        let mut msg = Message { raw: vec![0; 8] };
        msg.set_operate(operate);
        msg.set_origin(origin);
        msg.set_content(mc);
        msg
    }

    #[inline]
    fn is_raw_ok(&self) -> bool {
        self.raw.len() >= 8
    }

    #[inline]
    fn let_raw_be_ok(&mut self) {
        if self.raw.len() < 8 {
            let mut vec = vec![0; 8 - self.raw.len()];
            self.raw.append(&mut vec);
        }
    }

    pub fn set_operate(&mut self, ot: OperateType) {
        self.let_raw_be_ok();
        self.raw[3] = (self.raw[3] & 0b1111_1100) + ((ot as u8) & 0b0000_0011);
    }

    pub fn get_operate(&self) -> OperateType {
        if self.is_raw_ok() {
            OperateType::try_from(self.raw[3] & 0b0000_0011).unwrap_or(DEFAULT_OPERATE_TYPE)
        } else {
            DEFAULT_OPERATE_TYPE
        }
    }

    pub fn set_origin(&mut self, o: Origin) {
        self.let_raw_be_ok();
        self.raw[4] = ((o & 0xFF00_0000) >> 24) as u8;
        self.raw[5] = ((o & 0x00FF_0000) >> 16) as u8;
        self.raw[6] = ((o & 0x0000_FF00) >> 8) as u8;
        self.raw[7] = (o & 0x0000_00FF) as u8;
    }

    pub fn get_origin(&self) -> Origin {
        if self.is_raw_ok() {
            (u32::from(self.raw[4]) << 24)
                + (u32::from(self.raw[5]) << 16)
                + (u32::from(self.raw[6]) << 8)
                + (u32::from(self.raw[7]))
        } else {
            ZERO_ORIGIN
        }
    }

    fn set_compressed(&mut self, c: bool) {
        self.let_raw_be_ok();
        let c_val: u8 = if c { 0b0000_0100 } else { 0b0000_0000 };
        self.raw[3] = (self.raw[3] & 0b1111_1011) + (c_val & 0b0000_0100);
    }

    pub fn get_compressed(&self) -> bool {
        if self.is_raw_ok() {
            (self.raw[3] & 0b0000_0100) != 0
        } else {
            false // default
        }
    }

    pub fn set_content(&mut self, v: MsgClass) {
        let im: InnerMessage = v.into();
        let im_vec: Vec<u8> = im.try_into().unwrap();
        self.raw.drain(8..);
        match snappy::cita_compress_to(&im_vec[..], &mut self.raw) {
            Ok(true) => {
                self.set_compressed(true);
            }
            Ok(false) | Err(_) => {
                self.set_compressed(false);
                self.raw.extend_from_slice(&im_vec[..]);
            }
        }
    }

    pub fn take_content(&mut self) -> Option<MsgClass> {
        let im_opt = if self.get_compressed() {
            let mut im_vec: Vec<u8> = Vec::new();
            match snappy::cita_decompress_to(&self.raw[8..], &mut im_vec) {
                Ok(_) => InnerMessage::try_from(&im_vec).ok(),
                Err(_) => None,
            }
        } else {
            InnerMessage::try_from(&self.raw[8..]).ok()
        };
        if let Some(mut im) = im_opt {
            im.take_content()
        } else {
            None
        }
    }

    // Generate MSG-PROTOS methods automatically begin:
    pub fn take_raw_bytes(&mut self) -> Option<RawBytes> {
        match self.take_content() {
            Some(MsgClass::RawBytes(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_request(&mut self) -> Option<Request> {
        match self.take_content() {
            Some(MsgClass::Request(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_response(&mut self) -> Option<Response> {
        match self.take_content() {
            Some(MsgClass::Response(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_sync_request(&mut self) -> Option<SyncRequest> {
        match self.take_content() {
            Some(MsgClass::SyncRequest(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_sync_response(&mut self) -> Option<SyncResponse> {
        match self.take_content() {
            Some(MsgClass::SyncResponse(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_status(&mut self) -> Option<Status> {
        match self.take_content() {
            Some(MsgClass::Status(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_rich_status(&mut self) -> Option<RichStatus> {
        match self.take_content() {
            Some(MsgClass::RichStatus(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_signed_proposal(&mut self) -> Option<SignedProposal> {
        match self.take_content() {
            Some(MsgClass::SignedProposal(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_block(&mut self) -> Option<Block> {
        match self.take_content() {
            Some(MsgClass::Block(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_block_with_proof(&mut self) -> Option<BlockWithProof> {
        match self.take_content() {
            Some(MsgClass::BlockWithProof(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_block_header(&mut self) -> Option<BlockHeader> {
        match self.take_content() {
            Some(MsgClass::BlockHeader(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_block_txs(&mut self) -> Option<BlockTxs> {
        match self.take_content() {
            Some(MsgClass::BlockTxs(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_block_tx_hashes(&mut self) -> Option<BlockTxHashes> {
        match self.take_content() {
            Some(MsgClass::BlockTxHashes(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_block_tx_hashes_req(&mut self) -> Option<BlockTxHashesReq> {
        match self.take_content() {
            Some(MsgClass::BlockTxHashesReq(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_verify_block_req(&mut self) -> Option<VerifyBlockReq> {
        match self.take_content() {
            Some(MsgClass::VerifyBlockReq(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_verify_block_resp(&mut self) -> Option<VerifyBlockResp> {
        match self.take_content() {
            Some(MsgClass::VerifyBlockResp(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_executed_result(&mut self) -> Option<ExecutedResult> {
        match self.take_content() {
            Some(MsgClass::ExecutedResult(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_snapshot_req(&mut self) -> Option<SnapshotReq> {
        match self.take_content() {
            Some(MsgClass::SnapshotReq(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_snapshot_resp(&mut self) -> Option<SnapshotResp> {
        match self.take_content() {
            Some(MsgClass::SnapshotResp(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_miscellaneous(&mut self) -> Option<Miscellaneous> {
        match self.take_content() {
            Some(MsgClass::Miscellaneous(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_miscellaneous_req(&mut self) -> Option<MiscellaneousReq> {
        match self.take_content() {
            Some(MsgClass::MiscellaneousReq(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_black_list(&mut self) -> Option<BlackList> {
        match self.take_content() {
            Some(MsgClass::BlackList(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_state_signal(&mut self) -> Option<StateSignal> {
        match self.take_content() {
            Some(MsgClass::StateSignal(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_get_block_txn(&mut self) -> Option<GetBlockTxn> {
        match self.take_content() {
            Some(MsgClass::GetBlockTxn(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_block_txn(&mut self) -> Option<BlockTxn> {
        match self.take_content() {
            Some(MsgClass::BlockTxn(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_compact_signed_proposal(&mut self) -> Option<CompactSignedProposal> {
        match self.take_content() {
            Some(MsgClass::CompactSignedProposal(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_get_tx_list(&mut self) -> Option<GetTxList> {
        match self.take_content() {
            Some(MsgClass::GetTxList(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_trie_response(&mut self) -> Option<TrieResponse> {
        match self.take_content() {
            Some(MsgClass::TrieResponse(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_receipt_response(&mut self) -> Option<ReceiptResponse> {
        match self.take_content() {
            Some(MsgClass::ReceiptResponse(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_code_response(&mut self) -> Option<CodeResponse> {
        match self.take_content() {
            Some(MsgClass::CodeResponse(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_abi_response(&mut self) -> Option<AbiResponse> {
        match self.take_content() {
            Some(MsgClass::AbiResponse(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_block_response(&mut self) -> Option<BlockResponse> {
        match self.take_content() {
            Some(MsgClass::BlockResponse(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_sync_light_request(&mut self) -> Option<SyncLightRequest> {
        match self.take_content() {
            Some(MsgClass::SyncLightRequest(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_sync_light_response(&mut self) -> Option<SyncLightResponse> {
        match self.take_content() {
            Some(MsgClass::SyncLightResponse(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_light_request(&mut self) -> Option<LightRequest> {
        match self.take_content() {
            Some(MsgClass::LightRequest(v)) => Some(v),
            _ => None,
        }
    }
    pub fn take_light_response(&mut self) -> Option<LightResponse> {
        match self.take_content() {
            Some(MsgClass::LightResponse(v)) => Some(v),
            _ => None,
        }
    }
    // Generate MSG-PROTOS methods automatically end.
}

impl TryFrom<Vec<u8>> for Message {
    type Error = TryFromConvertError;
    fn try_from(v: Vec<u8>) -> Result<Self, TryFromConvertError> {
        if v.len() < 8 {
            Err(TryFromConvertError(()))
        } else {
            Ok(Message { raw: v })
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Message {
    type Error = TryFromConvertError;
    fn try_from(v: &[u8]) -> Result<Self, TryFromConvertError> {
        if v.len() < 8 {
            Err(TryFromConvertError(()))
        } else {
            Ok(Message { raw: v.to_vec() })
        }
    }
}

impl<'a> TryFrom<&'a Vec<u8>> for Message {
    type Error = TryFromConvertError;
    fn try_from(v: &Vec<u8>) -> Result<Self, TryFromConvertError> {
        if v.len() < 8 {
            Err(TryFromConvertError(()))
        } else {
            Ok(Message { raw: v.clone() })
        }
    }
}

impl TryInto<Vec<u8>> for Message {
    type Error = TryIntoConvertError;
    fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
        if self.raw.len() < 8 {
            Err(TryIntoConvertError(()))
        } else {
            Ok(self.raw)
        }
    }
}

impl<'a> TryInto<Vec<u8>> for &'a Message {
    type Error = TryIntoConvertError;
    fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
        if self.raw.len() < 8 {
            Err(TryIntoConvertError(()))
        } else {
            Ok(self.raw.clone())
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn convert_operate_type_works() {
        use super::OperateType;
        use super::TryFrom;
        let ot1 = OperateType::Broadcast;
        assert_eq!(ot1, OperateType::Broadcast);
        assert_eq!(ot1 as u8, OperateType::Broadcast as u8);
        assert_eq!(ot1 as u8, 0);
        assert_ne!(ot1, OperateType::Single);
        let ot2 = OperateType::Broadcast;
        assert_eq!(ot1, ot2);
        let ot3 = OperateType::Subtract;
        assert_ne!(ot1, ot3);
        let ot4_rst = OperateType::try_from(0);
        assert!(ot4_rst.is_ok());
        assert_eq!(ot1, ot4_rst.unwrap());
        let ot5_rst = OperateType::try_from(255);
        assert!(ot5_rst.is_err());
    }

    #[test]
    fn class_funcs_for_inner_message_works() {
        use super::{InnerMessage, Request};
        use std::convert::From;
        let mut im = InnerMessage::new();
        assert!(im.content.is_none());
        let req_origin = Request::new();
        im.set_content(req_origin.into());
        assert!(im.content.is_some());
        let req_take_opt = im.take_content();
        assert!(req_take_opt.is_some());
        let req_take = req_take_opt.unwrap();
        assert!(im.content.is_none());
        im.set_content(req_take.clone());
        assert!(im.content.is_some());
        let im2 = InnerMessage::from(req_take);
        assert_eq!(im, im2);
    }

    #[test]
    fn traits_for_all_protos_works() {
        use super::blockchain;
        use super::{TryFrom, TryInto};
        let height: u64 = 13579;
        let mut status = blockchain::Status::new();
        status.set_height(height);
        let status_bytes: Vec<u8> = (&status).try_into().unwrap();
        let status_bytes_consume: Vec<u8> = status.try_into().unwrap();
        assert_eq!(status_bytes, status_bytes_consume);
        let status_new = blockchain::Status::try_from(&status_bytes).unwrap();
        let status_new_from_slice = blockchain::Status::try_from(&status_bytes[..]).unwrap();
        assert_eq!(status_new, status_new_from_slice);
        let height_new = status_new.get_height();
        assert_eq!(height, height_new);
    }

    #[test]
    fn traits_for_msg_protos_works() {
        use super::{Message, MsgClass, Request, DEFAULT_OPERATE_TYPE, ZERO_ORIGIN};
        use std::convert::Into;
        let req = Request::new();
        let mc: MsgClass = req.clone().into();
        assert!(if let MsgClass::Request(_) = mc {
            true
        } else {
            false
        });
        let mut msg: Message = req.into();
        assert_eq!(msg.get_operate(), DEFAULT_OPERATE_TYPE);
        assert_eq!(msg.get_origin(), ZERO_ORIGIN);
        let mc_take = msg.take_content().unwrap();
        assert!(mc == mc_take)
    }

    #[test]
    fn class_funcs_for_message_works() {
        use super::{Message, MsgClass, OperateType, Request, Response};
        use super::{TryFrom, TryInto};
        use std::convert::Into;

        let req_origin = Request::new();
        let resp_origin = Response::new();

        let mut msg: Message = req_origin.clone().into();
        assert_ne!(msg.get_operate(), OperateType::Single);
        msg.set_operate(OperateType::Single);
        assert_eq!(msg.get_operate(), OperateType::Single);
        assert_ne!(msg.get_origin(), 1234567890);
        msg.set_origin(1234567890);
        assert_eq!(msg.get_origin(), 1234567890);

        let mut msg_same = Message::init(OperateType::Single, 1234567890, req_origin.into());
        assert_eq!(msg_same.get_origin(), 1234567890);
        assert_eq!(msg_same.get_operate(), OperateType::Single);

        let req_take_opt = msg.take_content();
        assert!(if let Some(MsgClass::Request(_)) = req_take_opt {
            true
        } else {
            false
        });

        let req_take = req_take_opt.unwrap();
        let req_same = msg_same.take_content().unwrap();
        assert!(req_take == req_same);

        let msg_bytes_rst: Result<Vec<u8>, _> = msg.clone().try_into();
        assert!(msg_bytes_rst.is_ok());
        let msg_bytes = msg_bytes_rst.unwrap();
        let msg_same_bytes: Vec<u8> = msg_same.clone().try_into().unwrap();
        assert_eq!(msg_bytes, msg_same_bytes);

        let mut msg_from_bytes = Message::try_from(msg_bytes).unwrap();
        assert_eq!(msg_from_bytes.get_origin(), 1234567890);
        assert_eq!(msg_from_bytes.get_operate(), OperateType::Single);
        let req_from_bytes = msg_from_bytes.take_content().unwrap();
        assert!(req_take == req_from_bytes);

        msg.set_content(resp_origin.into());
        let resp_take_opt = msg.take_content();
        assert!(if let Some(MsgClass::Response(_)) = resp_take_opt {
            true
        } else {
            false
        });

        assert!(msg.take_response().is_some());
        assert!(msg.take_request().is_none());
    }

    #[test]
    fn compress_and_decompress() {
        use super::Message;
        use snappy::CITA_SKIP_COMPRESS_SIZE;
        use std::convert::Into;

        let raw_bytes: Vec<u8> = vec![1, 2, 3, 4];
        let mut msg: Message = raw_bytes.clone().into();
        assert!(!msg.get_compressed());
        let raw_bytes_opt = msg.take_raw_bytes();
        assert!(raw_bytes_opt.is_some());
        assert_eq!(raw_bytes_opt.unwrap(), raw_bytes);
        let raw_bytes: Vec<u8> = [1; CITA_SKIP_COMPRESS_SIZE + 1].to_vec();
        let mut msg: Message = raw_bytes.clone().into();
        assert!(msg.get_compressed());
        let raw_bytes_opt = msg.take_raw_bytes();
        assert!(raw_bytes_opt.is_some());
        assert_eq!(raw_bytes_opt.unwrap(), raw_bytes);
    }
}
