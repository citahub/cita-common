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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    pub cmd_id: u32,
    pub origin: u32,
    pub operate: OperateType,
    // message oneof groups
    pub content: ::std::option::Option<Message_oneof_content>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

#[derive(Clone,PartialEq)]
pub enum Message_oneof_content {
    RawBytes(::std::vec::Vec<u8>),
    Request(super::request::Request),
    Response(super::response::Response),
    SyncRequest(super::sync::SyncRequest),
    SyncResponse(super::sync::SyncResponse),
    Status(super::blockchain::Status),
    RichStatus(super::blockchain::RichStatus),
    Block(super::blockchain::Block),
    BlockWithProof(super::blockchain::BlockWithProof),
    BlockHeader(super::blockchain::BlockHeader),
    BlockTxs(super::blockchain::BlockTxs),
    BlockTxHashes(super::auth::BlockTxHashes),
    BlockTxHashesReq(super::auth::BlockTxHashesReq),
    VerifyTxReq(super::auth::VerifyTxReq),
    VerifyTxResp(super::auth::VerifyTxResp),
    VerifyBlockReq(super::auth::VerifyBlockReq),
    VerifyBlockResp(super::auth::VerifyBlockResp),
    ExecutedResult(super::executor::ExecutedResult),
    SnapshotReq(super::snapshot::SnapshotReq),
    SnapshotResp(super::snapshot::SnapshotResp),
}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }

    // uint32 cmd_id = 1;

    pub fn clear_cmd_id(&mut self) {
        self.cmd_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_cmd_id(&mut self, v: u32) {
        self.cmd_id = v;
    }

    pub fn get_cmd_id(&self) -> u32 {
        self.cmd_id
    }

    fn get_cmd_id_for_reflect(&self) -> &u32 {
        &self.cmd_id
    }

    fn mut_cmd_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.cmd_id
    }

    // uint32 origin = 2;

    pub fn clear_origin(&mut self) {
        self.origin = 0;
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: u32) {
        self.origin = v;
    }

    pub fn get_origin(&self) -> u32 {
        self.origin
    }

    fn get_origin_for_reflect(&self) -> &u32 {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut u32 {
        &mut self.origin
    }

    // .OperateType operate = 3;

    pub fn clear_operate(&mut self) {
        self.operate = OperateType::BROADCAST;
    }

    // Param is passed by value, moved
    pub fn set_operate(&mut self, v: OperateType) {
        self.operate = v;
    }

    pub fn get_operate(&self) -> OperateType {
        self.operate
    }

    fn get_operate_for_reflect(&self) -> &OperateType {
        &self.operate
    }

    fn mut_operate_for_reflect(&mut self) -> &mut OperateType {
        &mut self.operate
    }

    // bytes RawBytes = 4;

    pub fn clear_RawBytes(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_RawBytes(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::RawBytes(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_RawBytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = ::std::option::Option::Some(Message_oneof_content::RawBytes(v))
    }

    // Mutable pointer to the field.
    pub fn mut_RawBytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(Message_oneof_content::RawBytes(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::RawBytes(::std::vec::Vec::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::RawBytes(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_RawBytes(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_RawBytes() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::RawBytes(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_RawBytes(&self) -> &[u8] {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::RawBytes(ref v)) => v,
            _ => &[],
        }
    }

    // .Request Request = 5;

    pub fn clear_Request(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Request(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Request(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Request(&mut self, v: super::request::Request) {
        self.content = ::std::option::Option::Some(Message_oneof_content::Request(v))
    }

    // Mutable pointer to the field.
    pub fn mut_Request(&mut self) -> &mut super::request::Request {
        if let ::std::option::Option::Some(Message_oneof_content::Request(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::Request(super::request::Request::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Request(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_Request(&mut self) -> super::request::Request {
        if self.has_Request() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::Request(v)) => v,
                _ => panic!(),
            }
        } else {
            super::request::Request::new()
        }
    }

    pub fn get_Request(&self) -> &super::request::Request {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Request(ref v)) => v,
            _ => super::request::Request::default_instance(),
        }
    }

    // .Response Response = 6;

    pub fn clear_Response(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Response(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Response(&mut self, v: super::response::Response) {
        self.content = ::std::option::Option::Some(Message_oneof_content::Response(v))
    }

    // Mutable pointer to the field.
    pub fn mut_Response(&mut self) -> &mut super::response::Response {
        if let ::std::option::Option::Some(Message_oneof_content::Response(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::Response(super::response::Response::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_Response(&mut self) -> super::response::Response {
        if self.has_Response() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::Response(v)) => v,
                _ => panic!(),
            }
        } else {
            super::response::Response::new()
        }
    }

    pub fn get_Response(&self) -> &super::response::Response {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Response(ref v)) => v,
            _ => super::response::Response::default_instance(),
        }
    }

    // .SyncRequest SyncRequest = 7;

    pub fn clear_SyncRequest(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_SyncRequest(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SyncRequest(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_SyncRequest(&mut self, v: super::sync::SyncRequest) {
        self.content = ::std::option::Option::Some(Message_oneof_content::SyncRequest(v))
    }

    // Mutable pointer to the field.
    pub fn mut_SyncRequest(&mut self) -> &mut super::sync::SyncRequest {
        if let ::std::option::Option::Some(Message_oneof_content::SyncRequest(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::SyncRequest(super::sync::SyncRequest::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SyncRequest(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_SyncRequest(&mut self) -> super::sync::SyncRequest {
        if self.has_SyncRequest() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::SyncRequest(v)) => v,
                _ => panic!(),
            }
        } else {
            super::sync::SyncRequest::new()
        }
    }

    pub fn get_SyncRequest(&self) -> &super::sync::SyncRequest {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SyncRequest(ref v)) => v,
            _ => super::sync::SyncRequest::default_instance(),
        }
    }

    // .SyncResponse SyncResponse = 8;

    pub fn clear_SyncResponse(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_SyncResponse(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SyncResponse(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_SyncResponse(&mut self, v: super::sync::SyncResponse) {
        self.content = ::std::option::Option::Some(Message_oneof_content::SyncResponse(v))
    }

    // Mutable pointer to the field.
    pub fn mut_SyncResponse(&mut self) -> &mut super::sync::SyncResponse {
        if let ::std::option::Option::Some(Message_oneof_content::SyncResponse(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::SyncResponse(super::sync::SyncResponse::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SyncResponse(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_SyncResponse(&mut self) -> super::sync::SyncResponse {
        if self.has_SyncResponse() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::SyncResponse(v)) => v,
                _ => panic!(),
            }
        } else {
            super::sync::SyncResponse::new()
        }
    }

    pub fn get_SyncResponse(&self) -> &super::sync::SyncResponse {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SyncResponse(ref v)) => v,
            _ => super::sync::SyncResponse::default_instance(),
        }
    }

    // .Status Status = 9;

    pub fn clear_Status(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Status(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Status(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Status(&mut self, v: super::blockchain::Status) {
        self.content = ::std::option::Option::Some(Message_oneof_content::Status(v))
    }

    // Mutable pointer to the field.
    pub fn mut_Status(&mut self) -> &mut super::blockchain::Status {
        if let ::std::option::Option::Some(Message_oneof_content::Status(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::Status(super::blockchain::Status::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Status(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_Status(&mut self) -> super::blockchain::Status {
        if self.has_Status() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::Status(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::Status::new()
        }
    }

    pub fn get_Status(&self) -> &super::blockchain::Status {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Status(ref v)) => v,
            _ => super::blockchain::Status::default_instance(),
        }
    }

    // .RichStatus RichStatus = 10;

    pub fn clear_RichStatus(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_RichStatus(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::RichStatus(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_RichStatus(&mut self, v: super::blockchain::RichStatus) {
        self.content = ::std::option::Option::Some(Message_oneof_content::RichStatus(v))
    }

    // Mutable pointer to the field.
    pub fn mut_RichStatus(&mut self) -> &mut super::blockchain::RichStatus {
        if let ::std::option::Option::Some(Message_oneof_content::RichStatus(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::RichStatus(super::blockchain::RichStatus::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::RichStatus(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_RichStatus(&mut self) -> super::blockchain::RichStatus {
        if self.has_RichStatus() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::RichStatus(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::RichStatus::new()
        }
    }

    pub fn get_RichStatus(&self) -> &super::blockchain::RichStatus {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::RichStatus(ref v)) => v,
            _ => super::blockchain::RichStatus::default_instance(),
        }
    }

    // .Block Block = 11;

    pub fn clear_Block(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Block(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Block(&mut self, v: super::blockchain::Block) {
        self.content = ::std::option::Option::Some(Message_oneof_content::Block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_Block(&mut self) -> &mut super::blockchain::Block {
        if let ::std::option::Option::Some(Message_oneof_content::Block(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::Block(super::blockchain::Block::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_Block(&mut self) -> super::blockchain::Block {
        if self.has_Block() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::Block(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::Block::new()
        }
    }

    pub fn get_Block(&self) -> &super::blockchain::Block {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::Block(ref v)) => v,
            _ => super::blockchain::Block::default_instance(),
        }
    }

    // .BlockWithProof BlockWithProof = 12;

    pub fn clear_BlockWithProof(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_BlockWithProof(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockWithProof(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BlockWithProof(&mut self, v: super::blockchain::BlockWithProof) {
        self.content = ::std::option::Option::Some(Message_oneof_content::BlockWithProof(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BlockWithProof(&mut self) -> &mut super::blockchain::BlockWithProof {
        if let ::std::option::Option::Some(Message_oneof_content::BlockWithProof(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::BlockWithProof(super::blockchain::BlockWithProof::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockWithProof(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BlockWithProof(&mut self) -> super::blockchain::BlockWithProof {
        if self.has_BlockWithProof() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::BlockWithProof(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::BlockWithProof::new()
        }
    }

    pub fn get_BlockWithProof(&self) -> &super::blockchain::BlockWithProof {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockWithProof(ref v)) => v,
            _ => super::blockchain::BlockWithProof::default_instance(),
        }
    }

    // .BlockHeader BlockHeader = 13;

    pub fn clear_BlockHeader(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_BlockHeader(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockHeader(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BlockHeader(&mut self, v: super::blockchain::BlockHeader) {
        self.content = ::std::option::Option::Some(Message_oneof_content::BlockHeader(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BlockHeader(&mut self) -> &mut super::blockchain::BlockHeader {
        if let ::std::option::Option::Some(Message_oneof_content::BlockHeader(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::BlockHeader(super::blockchain::BlockHeader::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockHeader(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BlockHeader(&mut self) -> super::blockchain::BlockHeader {
        if self.has_BlockHeader() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::BlockHeader(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::BlockHeader::new()
        }
    }

    pub fn get_BlockHeader(&self) -> &super::blockchain::BlockHeader {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockHeader(ref v)) => v,
            _ => super::blockchain::BlockHeader::default_instance(),
        }
    }

    // .BlockTxs BlockTxs = 14;

    pub fn clear_BlockTxs(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_BlockTxs(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockTxs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BlockTxs(&mut self, v: super::blockchain::BlockTxs) {
        self.content = ::std::option::Option::Some(Message_oneof_content::BlockTxs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BlockTxs(&mut self) -> &mut super::blockchain::BlockTxs {
        if let ::std::option::Option::Some(Message_oneof_content::BlockTxs(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::BlockTxs(super::blockchain::BlockTxs::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockTxs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BlockTxs(&mut self) -> super::blockchain::BlockTxs {
        if self.has_BlockTxs() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::BlockTxs(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::BlockTxs::new()
        }
    }

    pub fn get_BlockTxs(&self) -> &super::blockchain::BlockTxs {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockTxs(ref v)) => v,
            _ => super::blockchain::BlockTxs::default_instance(),
        }
    }

    // .BlockTxHashes BlockTxHashes = 15;

    pub fn clear_BlockTxHashes(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_BlockTxHashes(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockTxHashes(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BlockTxHashes(&mut self, v: super::auth::BlockTxHashes) {
        self.content = ::std::option::Option::Some(Message_oneof_content::BlockTxHashes(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BlockTxHashes(&mut self) -> &mut super::auth::BlockTxHashes {
        if let ::std::option::Option::Some(Message_oneof_content::BlockTxHashes(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::BlockTxHashes(super::auth::BlockTxHashes::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockTxHashes(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BlockTxHashes(&mut self) -> super::auth::BlockTxHashes {
        if self.has_BlockTxHashes() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::BlockTxHashes(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::BlockTxHashes::new()
        }
    }

    pub fn get_BlockTxHashes(&self) -> &super::auth::BlockTxHashes {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockTxHashes(ref v)) => v,
            _ => super::auth::BlockTxHashes::default_instance(),
        }
    }

    // .BlockTxHashesReq BlockTxHashesReq = 16;

    pub fn clear_BlockTxHashesReq(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_BlockTxHashesReq(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockTxHashesReq(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BlockTxHashesReq(&mut self, v: super::auth::BlockTxHashesReq) {
        self.content = ::std::option::Option::Some(Message_oneof_content::BlockTxHashesReq(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BlockTxHashesReq(&mut self) -> &mut super::auth::BlockTxHashesReq {
        if let ::std::option::Option::Some(Message_oneof_content::BlockTxHashesReq(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::BlockTxHashesReq(super::auth::BlockTxHashesReq::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockTxHashesReq(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BlockTxHashesReq(&mut self) -> super::auth::BlockTxHashesReq {
        if self.has_BlockTxHashesReq() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::BlockTxHashesReq(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::BlockTxHashesReq::new()
        }
    }

    pub fn get_BlockTxHashesReq(&self) -> &super::auth::BlockTxHashesReq {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::BlockTxHashesReq(ref v)) => v,
            _ => super::auth::BlockTxHashesReq::default_instance(),
        }
    }

    // .VerifyTxReq VerifyTxReq = 17;

    pub fn clear_VerifyTxReq(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_VerifyTxReq(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyTxReq(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_VerifyTxReq(&mut self, v: super::auth::VerifyTxReq) {
        self.content = ::std::option::Option::Some(Message_oneof_content::VerifyTxReq(v))
    }

    // Mutable pointer to the field.
    pub fn mut_VerifyTxReq(&mut self) -> &mut super::auth::VerifyTxReq {
        if let ::std::option::Option::Some(Message_oneof_content::VerifyTxReq(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::VerifyTxReq(super::auth::VerifyTxReq::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyTxReq(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_VerifyTxReq(&mut self) -> super::auth::VerifyTxReq {
        if self.has_VerifyTxReq() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::VerifyTxReq(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::VerifyTxReq::new()
        }
    }

    pub fn get_VerifyTxReq(&self) -> &super::auth::VerifyTxReq {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyTxReq(ref v)) => v,
            _ => super::auth::VerifyTxReq::default_instance(),
        }
    }

    // .VerifyTxResp VerifyTxResp = 18;

    pub fn clear_VerifyTxResp(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_VerifyTxResp(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyTxResp(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_VerifyTxResp(&mut self, v: super::auth::VerifyTxResp) {
        self.content = ::std::option::Option::Some(Message_oneof_content::VerifyTxResp(v))
    }

    // Mutable pointer to the field.
    pub fn mut_VerifyTxResp(&mut self) -> &mut super::auth::VerifyTxResp {
        if let ::std::option::Option::Some(Message_oneof_content::VerifyTxResp(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::VerifyTxResp(super::auth::VerifyTxResp::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyTxResp(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_VerifyTxResp(&mut self) -> super::auth::VerifyTxResp {
        if self.has_VerifyTxResp() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::VerifyTxResp(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::VerifyTxResp::new()
        }
    }

    pub fn get_VerifyTxResp(&self) -> &super::auth::VerifyTxResp {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyTxResp(ref v)) => v,
            _ => super::auth::VerifyTxResp::default_instance(),
        }
    }

    // .VerifyBlockReq VerifyBlockReq = 19;

    pub fn clear_VerifyBlockReq(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_VerifyBlockReq(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyBlockReq(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_VerifyBlockReq(&mut self, v: super::auth::VerifyBlockReq) {
        self.content = ::std::option::Option::Some(Message_oneof_content::VerifyBlockReq(v))
    }

    // Mutable pointer to the field.
    pub fn mut_VerifyBlockReq(&mut self) -> &mut super::auth::VerifyBlockReq {
        if let ::std::option::Option::Some(Message_oneof_content::VerifyBlockReq(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::VerifyBlockReq(super::auth::VerifyBlockReq::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyBlockReq(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_VerifyBlockReq(&mut self) -> super::auth::VerifyBlockReq {
        if self.has_VerifyBlockReq() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::VerifyBlockReq(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::VerifyBlockReq::new()
        }
    }

    pub fn get_VerifyBlockReq(&self) -> &super::auth::VerifyBlockReq {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyBlockReq(ref v)) => v,
            _ => super::auth::VerifyBlockReq::default_instance(),
        }
    }

    // .VerifyBlockResp VerifyBlockResp = 20;

    pub fn clear_VerifyBlockResp(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_VerifyBlockResp(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyBlockResp(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_VerifyBlockResp(&mut self, v: super::auth::VerifyBlockResp) {
        self.content = ::std::option::Option::Some(Message_oneof_content::VerifyBlockResp(v))
    }

    // Mutable pointer to the field.
    pub fn mut_VerifyBlockResp(&mut self) -> &mut super::auth::VerifyBlockResp {
        if let ::std::option::Option::Some(Message_oneof_content::VerifyBlockResp(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::VerifyBlockResp(super::auth::VerifyBlockResp::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyBlockResp(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_VerifyBlockResp(&mut self) -> super::auth::VerifyBlockResp {
        if self.has_VerifyBlockResp() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::VerifyBlockResp(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::VerifyBlockResp::new()
        }
    }

    pub fn get_VerifyBlockResp(&self) -> &super::auth::VerifyBlockResp {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::VerifyBlockResp(ref v)) => v,
            _ => super::auth::VerifyBlockResp::default_instance(),
        }
    }

    // .ExecutedResult ExecutedResult = 21;

    pub fn clear_ExecutedResult(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_ExecutedResult(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::ExecutedResult(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ExecutedResult(&mut self, v: super::executor::ExecutedResult) {
        self.content = ::std::option::Option::Some(Message_oneof_content::ExecutedResult(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ExecutedResult(&mut self) -> &mut super::executor::ExecutedResult {
        if let ::std::option::Option::Some(Message_oneof_content::ExecutedResult(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::ExecutedResult(super::executor::ExecutedResult::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::ExecutedResult(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ExecutedResult(&mut self) -> super::executor::ExecutedResult {
        if self.has_ExecutedResult() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::ExecutedResult(v)) => v,
                _ => panic!(),
            }
        } else {
            super::executor::ExecutedResult::new()
        }
    }

    pub fn get_ExecutedResult(&self) -> &super::executor::ExecutedResult {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::ExecutedResult(ref v)) => v,
            _ => super::executor::ExecutedResult::default_instance(),
        }
    }

    // .SnapshotReq SnapshotReq = 22;

    pub fn clear_SnapshotReq(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_SnapshotReq(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SnapshotReq(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_SnapshotReq(&mut self, v: super::snapshot::SnapshotReq) {
        self.content = ::std::option::Option::Some(Message_oneof_content::SnapshotReq(v))
    }

    // Mutable pointer to the field.
    pub fn mut_SnapshotReq(&mut self) -> &mut super::snapshot::SnapshotReq {
        if let ::std::option::Option::Some(Message_oneof_content::SnapshotReq(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::SnapshotReq(super::snapshot::SnapshotReq::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SnapshotReq(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_SnapshotReq(&mut self) -> super::snapshot::SnapshotReq {
        if self.has_SnapshotReq() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::SnapshotReq(v)) => v,
                _ => panic!(),
            }
        } else {
            super::snapshot::SnapshotReq::new()
        }
    }

    pub fn get_SnapshotReq(&self) -> &super::snapshot::SnapshotReq {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SnapshotReq(ref v)) => v,
            _ => super::snapshot::SnapshotReq::default_instance(),
        }
    }

    // .SnapshotResp SnapshotResp = 23;

    pub fn clear_SnapshotResp(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_SnapshotResp(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SnapshotResp(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_SnapshotResp(&mut self, v: super::snapshot::SnapshotResp) {
        self.content = ::std::option::Option::Some(Message_oneof_content::SnapshotResp(v))
    }

    // Mutable pointer to the field.
    pub fn mut_SnapshotResp(&mut self) -> &mut super::snapshot::SnapshotResp {
        if let ::std::option::Option::Some(Message_oneof_content::SnapshotResp(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(Message_oneof_content::SnapshotResp(super::snapshot::SnapshotResp::new()));
        }
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SnapshotResp(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_SnapshotResp(&mut self) -> super::snapshot::SnapshotResp {
        if self.has_SnapshotResp() {
            match self.content.take() {
                ::std::option::Option::Some(Message_oneof_content::SnapshotResp(v)) => v,
                _ => panic!(),
            }
        } else {
            super::snapshot::SnapshotResp::new()
        }
    }

    pub fn get_SnapshotResp(&self) -> &super::snapshot::SnapshotResp {
        match self.content {
            ::std::option::Option::Some(Message_oneof_content::SnapshotResp(ref v)) => v,
            _ => super::snapshot::SnapshotResp::default_instance(),
        }
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        if let Some(Message_oneof_content::Request(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::Response(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::SyncRequest(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::SyncResponse(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::Status(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::RichStatus(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::Block(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::BlockWithProof(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::BlockHeader(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::BlockTxs(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::BlockTxHashes(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::BlockTxHashesReq(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::VerifyTxReq(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::VerifyTxResp(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::VerifyBlockReq(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::VerifyBlockResp(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::ExecutedResult(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::SnapshotReq(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Message_oneof_content::SnapshotResp(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.cmd_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.origin = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.operate = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::RawBytes(is.read_bytes()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::Request(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::Response(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::SyncRequest(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::SyncResponse(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::Status(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::RichStatus(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::Block(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::BlockWithProof(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::BlockHeader(is.read_message()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::BlockTxs(is.read_message()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::BlockTxHashes(is.read_message()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::BlockTxHashesReq(is.read_message()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::VerifyTxReq(is.read_message()?));
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::VerifyTxResp(is.read_message()?));
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::VerifyBlockReq(is.read_message()?));
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::VerifyBlockResp(is.read_message()?));
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::ExecutedResult(is.read_message()?));
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::SnapshotReq(is.read_message()?));
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(Message_oneof_content::SnapshotResp(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.cmd_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.cmd_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.origin != 0 {
            my_size += ::protobuf::rt::value_size(2, self.origin, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.operate != OperateType::BROADCAST {
            my_size += ::protobuf::rt::enum_size(3, self.operate);
        }
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &Message_oneof_content::RawBytes(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(4, &v);
                },
                &Message_oneof_content::Request(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::Response(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::SyncRequest(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::SyncResponse(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::Status(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::RichStatus(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::Block(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::BlockWithProof(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::BlockHeader(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::BlockTxs(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::BlockTxHashes(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::BlockTxHashesReq(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::VerifyTxReq(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::VerifyTxResp(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::VerifyBlockReq(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::VerifyBlockResp(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::ExecutedResult(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::SnapshotReq(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Message_oneof_content::SnapshotResp(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.cmd_id != 0 {
            os.write_uint32(1, self.cmd_id)?;
        }
        if self.origin != 0 {
            os.write_uint32(2, self.origin)?;
        }
        if self.operate != OperateType::BROADCAST {
            os.write_enum(3, self.operate.value())?;
        }
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &Message_oneof_content::RawBytes(ref v) => {
                    os.write_bytes(4, v)?;
                },
                &Message_oneof_content::Request(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::Response(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::SyncRequest(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::SyncResponse(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::Status(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::RichStatus(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::Block(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::BlockWithProof(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::BlockHeader(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::BlockTxs(ref v) => {
                    os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::BlockTxHashes(ref v) => {
                    os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::BlockTxHashesReq(ref v) => {
                    os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::VerifyTxReq(ref v) => {
                    os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::VerifyTxResp(ref v) => {
                    os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::VerifyBlockReq(ref v) => {
                    os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::VerifyBlockResp(ref v) => {
                    os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::ExecutedResult(ref v) => {
                    os.write_tag(21, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::SnapshotReq(ref v) => {
                    os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Message_oneof_content::SnapshotResp(ref v) => {
                    os.write_tag(23, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cmd_id",
                    Message::get_cmd_id_for_reflect,
                    Message::mut_cmd_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "origin",
                    Message::get_origin_for_reflect,
                    Message::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OperateType>>(
                    "operate",
                    Message::get_operate_for_reflect,
                    Message::mut_operate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "RawBytes",
                    Message::has_RawBytes,
                    Message::get_RawBytes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::request::Request>(
                    "Request",
                    Message::has_Request,
                    Message::get_Request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::response::Response>(
                    "Response",
                    Message::has_Response,
                    Message::get_Response,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::sync::SyncRequest>(
                    "SyncRequest",
                    Message::has_SyncRequest,
                    Message::get_SyncRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::sync::SyncResponse>(
                    "SyncResponse",
                    Message::has_SyncResponse,
                    Message::get_SyncResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::Status>(
                    "Status",
                    Message::has_Status,
                    Message::get_Status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::RichStatus>(
                    "RichStatus",
                    Message::has_RichStatus,
                    Message::get_RichStatus,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::Block>(
                    "Block",
                    Message::has_Block,
                    Message::get_Block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::BlockWithProof>(
                    "BlockWithProof",
                    Message::has_BlockWithProof,
                    Message::get_BlockWithProof,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::BlockHeader>(
                    "BlockHeader",
                    Message::has_BlockHeader,
                    Message::get_BlockHeader,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::BlockTxs>(
                    "BlockTxs",
                    Message::has_BlockTxs,
                    Message::get_BlockTxs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::BlockTxHashes>(
                    "BlockTxHashes",
                    Message::has_BlockTxHashes,
                    Message::get_BlockTxHashes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::BlockTxHashesReq>(
                    "BlockTxHashesReq",
                    Message::has_BlockTxHashesReq,
                    Message::get_BlockTxHashesReq,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::VerifyTxReq>(
                    "VerifyTxReq",
                    Message::has_VerifyTxReq,
                    Message::get_VerifyTxReq,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::VerifyTxResp>(
                    "VerifyTxResp",
                    Message::has_VerifyTxResp,
                    Message::get_VerifyTxResp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::VerifyBlockReq>(
                    "VerifyBlockReq",
                    Message::has_VerifyBlockReq,
                    Message::get_VerifyBlockReq,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::VerifyBlockResp>(
                    "VerifyBlockResp",
                    Message::has_VerifyBlockResp,
                    Message::get_VerifyBlockResp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::executor::ExecutedResult>(
                    "ExecutedResult",
                    Message::has_ExecutedResult,
                    Message::get_ExecutedResult,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::snapshot::SnapshotReq>(
                    "SnapshotReq",
                    Message::has_SnapshotReq,
                    Message::get_SnapshotReq,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::snapshot::SnapshotResp>(
                    "SnapshotResp",
                    Message::has_SnapshotResp,
                    Message::get_SnapshotResp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_cmd_id();
        self.clear_origin();
        self.clear_operate();
        self.clear_RawBytes();
        self.clear_Request();
        self.clear_Response();
        self.clear_SyncRequest();
        self.clear_SyncResponse();
        self.clear_Status();
        self.clear_RichStatus();
        self.clear_Block();
        self.clear_BlockWithProof();
        self.clear_BlockHeader();
        self.clear_BlockTxs();
        self.clear_BlockTxHashes();
        self.clear_BlockTxHashesReq();
        self.clear_VerifyTxReq();
        self.clear_VerifyTxResp();
        self.clear_VerifyBlockReq();
        self.clear_VerifyBlockResp();
        self.clear_ExecutedResult();
        self.clear_SnapshotReq();
        self.clear_SnapshotResp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OperateType {
    BROADCAST = 0,
    SINGLE = 1,
    SUBTRACT = 2,
}

impl ::protobuf::ProtobufEnum for OperateType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OperateType> {
        match value {
            0 => ::std::option::Option::Some(OperateType::BROADCAST),
            1 => ::std::option::Option::Some(OperateType::SINGLE),
            2 => ::std::option::Option::Some(OperateType::SUBTRACT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OperateType] = &[
            OperateType::BROADCAST,
            OperateType::SINGLE,
            OperateType::SUBTRACT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OperateType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OperateType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OperateType {
}

impl ::std::default::Default for OperateType {
    fn default() -> Self {
        OperateType::BROADCAST
    }
}

impl ::protobuf::reflect::ProtobufValue for OperateType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13communication.proto\x1a\rrequest.proto\x1a\x0eresponse.proto\x1a\n\
    sync.proto\x1a\x10blockchain.proto\x1a\nauth.proto\x1a\x0eexecutor.proto\
    \x1a\x0esnapshot.proto\"\xc2\x08\n\x07Message\x12\x15\n\x06cmd_id\x18\
    \x01\x20\x01(\rR\x05cmdId\x12\x16\n\x06origin\x18\x02\x20\x01(\rR\x06ori\
    gin\x12&\n\x07operate\x18\x03\x20\x01(\x0e2\x0c.OperateTypeR\x07operate\
    \x12\x1c\n\x08RawBytes\x18\x04\x20\x01(\x0cH\0R\x08RawBytes\x12$\n\x07Re\
    quest\x18\x05\x20\x01(\x0b2\x08.RequestH\0R\x07Request\x12'\n\x08Respons\
    e\x18\x06\x20\x01(\x0b2\t.ResponseH\0R\x08Response\x120\n\x0bSyncRequest\
    \x18\x07\x20\x01(\x0b2\x0c.SyncRequestH\0R\x0bSyncRequest\x123\n\x0cSync\
    Response\x18\x08\x20\x01(\x0b2\r.SyncResponseH\0R\x0cSyncResponse\x12!\n\
    \x06Status\x18\t\x20\x01(\x0b2\x07.StatusH\0R\x06Status\x12-\n\nRichStat\
    us\x18\n\x20\x01(\x0b2\x0b.RichStatusH\0R\nRichStatus\x12\x1e\n\x05Block\
    \x18\x0b\x20\x01(\x0b2\x06.BlockH\0R\x05Block\x129\n\x0eBlockWithProof\
    \x18\x0c\x20\x01(\x0b2\x0f.BlockWithProofH\0R\x0eBlockWithProof\x120\n\
    \x0bBlockHeader\x18\r\x20\x01(\x0b2\x0c.BlockHeaderH\0R\x0bBlockHeader\
    \x12'\n\x08BlockTxs\x18\x0e\x20\x01(\x0b2\t.BlockTxsH\0R\x08BlockTxs\x12\
    6\n\rBlockTxHashes\x18\x0f\x20\x01(\x0b2\x0e.BlockTxHashesH\0R\rBlockTxH\
    ashes\x12?\n\x10BlockTxHashesReq\x18\x10\x20\x01(\x0b2\x11.BlockTxHashes\
    ReqH\0R\x10BlockTxHashesReq\x120\n\x0bVerifyTxReq\x18\x11\x20\x01(\x0b2\
    \x0c.VerifyTxReqH\0R\x0bVerifyTxReq\x123\n\x0cVerifyTxResp\x18\x12\x20\
    \x01(\x0b2\r.VerifyTxRespH\0R\x0cVerifyTxResp\x129\n\x0eVerifyBlockReq\
    \x18\x13\x20\x01(\x0b2\x0f.VerifyBlockReqH\0R\x0eVerifyBlockReq\x12<\n\
    \x0fVerifyBlockResp\x18\x14\x20\x01(\x0b2\x10.VerifyBlockRespH\0R\x0fVer\
    ifyBlockResp\x129\n\x0eExecutedResult\x18\x15\x20\x01(\x0b2\x0f.Executed\
    ResultH\0R\x0eExecutedResult\x120\n\x0bSnapshotReq\x18\x16\x20\x01(\x0b2\
    \x0c.SnapshotReqH\0R\x0bSnapshotReq\x123\n\x0cSnapshotResp\x18\x17\x20\
    \x01(\x0b2\r.SnapshotRespH\0R\x0cSnapshotRespB\t\n\x07content*6\n\x0bOpe\
    rateType\x12\r\n\tBROADCAST\x10\0\x12\n\n\x06SINGLE\x10\x01\x12\x0c\n\
    \x08SUBTRACT\x10\x02J\xc4\x0c\n\x06\x12\x04\0\04\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07\x16\n\t\n\x02\x03\x01\x12\
    \x03\x03\x07\x17\n\t\n\x02\x03\x02\x12\x03\x04\x07\x13\n\t\n\x02\x03\x03\
    \x12\x03\x05\x07\x19\n\t\n\x02\x03\x04\x12\x03\x06\x07\x13\n\t\n\x02\x03\
    \x05\x12\x03\x07\x07\x17\n\t\n\x02\x03\x06\x12\x03\x08\x07\x17\n\n\n\x02\
    \x05\0\x12\x04\n\0\x0e\x01\n\n\n\x03\x05\0\x01\x12\x03\n\x05\x10\n\x0b\n\
    \x04\x05\0\x02\0\x12\x03\x0b\x04\x12\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\
    \x0b\x04\r\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x0b\x10\x11\n\x0b\n\x04\
    \x05\0\x02\x01\x12\x03\x0c\x04\x0f\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\
    \x0c\x04\n\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x0c\r\x0e\n\x0b\n\x04\
    \x05\0\x02\x02\x12\x03\r\x04\x11\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\r\
    \x04\x0c\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\r\x0f\x10\n\n\n\x02\x04\0\
    \x12\x04\x10\04\x01\n\n\n\x03\x04\0\x01\x12\x03\x10\x08\x0f\n\x0b\n\x04\
    \x04\0\x02\0\x12\x03\x11\x04\x16\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x11\
    \x04\x10\x11\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x11\x04\n\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03\x11\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\
    \x11\x14\x15\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x12\x04\x16\n\r\n\x05\x04\
    \0\x02\x01\x04\x12\x04\x12\x04\x11\x16\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\x12\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x12\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x12\x14\x15\n\x0b\n\x04\x04\0\x02\x02\x12\
    \x03\x13\x04\x1c\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x13\x04\x12\x16\n\
    \x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x13\x04\x0f\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\x13\x10\x17\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x13\
    \x1a\x1b\n\x0c\n\x04\x04\0\x08\0\x12\x04\x15\x043\x05\n\x0c\n\x05\x04\0\
    \x08\0\x01\x12\x03\x15\n\x11\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x17\x08\
    \x1b\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x17\x08\r\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x03\x17\x0e\x16\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\
    \x17\x19\x1a\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x19\x08\x1c\n\x0c\n\x05\
    \x04\0\x02\x04\x06\x12\x03\x19\x08\x0f\n\x0c\n\x05\x04\0\x02\x04\x01\x12\
    \x03\x19\x10\x17\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x19\x1a\x1b\n\x0b\
    \n\x04\x04\0\x02\x05\x12\x03\x1a\x08\x1e\n\x0c\n\x05\x04\0\x02\x05\x06\
    \x12\x03\x1a\x08\x10\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x1a\x11\x19\n\
    \x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x1a\x1c\x1d\n\x0b\n\x04\x04\0\x02\
    \x06\x12\x03\x1c\x08$\n\x0c\n\x05\x04\0\x02\x06\x06\x12\x03\x1c\x08\x13\
    \n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x1c\x14\x1f\n\x0c\n\x05\x04\0\x02\
    \x06\x03\x12\x03\x1c\"#\n\x0b\n\x04\x04\0\x02\x07\x12\x03\x1d\x08&\n\x0c\
    \n\x05\x04\0\x02\x07\x06\x12\x03\x1d\x08\x14\n\x0c\n\x05\x04\0\x02\x07\
    \x01\x12\x03\x1d\x15!\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x1d$%\n\x0b\
    \n\x04\x04\0\x02\x08\x12\x03\x1f\x08\x1a\n\x0c\n\x05\x04\0\x02\x08\x06\
    \x12\x03\x1f\x08\x0e\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x1f\x0f\x15\n\
    \x0c\n\x05\x04\0\x02\x08\x03\x12\x03\x1f\x18\x19\n\x0b\n\x04\x04\0\x02\t\
    \x12\x03\x20\x08#\n\x0c\n\x05\x04\0\x02\t\x06\x12\x03\x20\x08\x12\n\x0c\
    \n\x05\x04\0\x02\t\x01\x12\x03\x20\x13\x1d\n\x0c\n\x05\x04\0\x02\t\x03\
    \x12\x03\x20\x20\"\n\x0b\n\x04\x04\0\x02\n\x12\x03\"\x08\x19\n\x0c\n\x05\
    \x04\0\x02\n\x06\x12\x03\"\x08\r\n\x0c\n\x05\x04\0\x02\n\x01\x12\x03\"\
    \x0e\x13\n\x0c\n\x05\x04\0\x02\n\x03\x12\x03\"\x16\x18\n\x0b\n\x04\x04\0\
    \x02\x0b\x12\x03#\x08+\n\x0c\n\x05\x04\0\x02\x0b\x06\x12\x03#\x08\x16\n\
    \x0c\n\x05\x04\0\x02\x0b\x01\x12\x03#\x17%\n\x0c\n\x05\x04\0\x02\x0b\x03\
    \x12\x03#(*\n\x0b\n\x04\x04\0\x02\x0c\x12\x03$\x08%\n\x0c\n\x05\x04\0\
    \x02\x0c\x06\x12\x03$\x08\x13\n\x0c\n\x05\x04\0\x02\x0c\x01\x12\x03$\x14\
    \x1f\n\x0c\n\x05\x04\0\x02\x0c\x03\x12\x03$\"$\n\x0b\n\x04\x04\0\x02\r\
    \x12\x03%\x08\x1f\n\x0c\n\x05\x04\0\x02\r\x06\x12\x03%\x08\x10\n\x0c\n\
    \x05\x04\0\x02\r\x01\x12\x03%\x11\x19\n\x0c\n\x05\x04\0\x02\r\x03\x12\
    \x03%\x1c\x1e\n\x0b\n\x04\x04\0\x02\x0e\x12\x03'\x08)\n\x0c\n\x05\x04\0\
    \x02\x0e\x06\x12\x03'\x08\x15\n\x0c\n\x05\x04\0\x02\x0e\x01\x12\x03'\x16\
    #\n\x0c\n\x05\x04\0\x02\x0e\x03\x12\x03'&(\n\x0b\n\x04\x04\0\x02\x0f\x12\
    \x03(\x08/\n\x0c\n\x05\x04\0\x02\x0f\x06\x12\x03(\x08\x18\n\x0c\n\x05\
    \x04\0\x02\x0f\x01\x12\x03(\x19)\n\x0c\n\x05\x04\0\x02\x0f\x03\x12\x03(,\
    .\n\x0b\n\x04\x04\0\x02\x10\x12\x03*\x08%\n\x0c\n\x05\x04\0\x02\x10\x06\
    \x12\x03*\x08\x13\n\x0c\n\x05\x04\0\x02\x10\x01\x12\x03*\x14\x1f\n\x0c\n\
    \x05\x04\0\x02\x10\x03\x12\x03*\"$\n\x0b\n\x04\x04\0\x02\x11\x12\x03+\
    \x08'\n\x0c\n\x05\x04\0\x02\x11\x06\x12\x03+\x08\x14\n\x0c\n\x05\x04\0\
    \x02\x11\x01\x12\x03+\x15!\n\x0c\n\x05\x04\0\x02\x11\x03\x12\x03+$&\n\
    \x0b\n\x04\x04\0\x02\x12\x12\x03,\x08+\n\x0c\n\x05\x04\0\x02\x12\x06\x12\
    \x03,\x08\x16\n\x0c\n\x05\x04\0\x02\x12\x01\x12\x03,\x17%\n\x0c\n\x05\
    \x04\0\x02\x12\x03\x12\x03,(*\n\x0b\n\x04\x04\0\x02\x13\x12\x03-\x08-\n\
    \x0c\n\x05\x04\0\x02\x13\x06\x12\x03-\x08\x17\n\x0c\n\x05\x04\0\x02\x13\
    \x01\x12\x03-\x18'\n\x0c\n\x05\x04\0\x02\x13\x03\x12\x03-*,\n\x0b\n\x04\
    \x04\0\x02\x14\x12\x03/\x08+\n\x0c\n\x05\x04\0\x02\x14\x06\x12\x03/\x08\
    \x16\n\x0c\n\x05\x04\0\x02\x14\x01\x12\x03/\x17%\n\x0c\n\x05\x04\0\x02\
    \x14\x03\x12\x03/(*\n\x0b\n\x04\x04\0\x02\x15\x12\x031\x08%\n\x0c\n\x05\
    \x04\0\x02\x15\x06\x12\x031\x08\x13\n\x0c\n\x05\x04\0\x02\x15\x01\x12\
    \x031\x14\x1f\n\x0c\n\x05\x04\0\x02\x15\x03\x12\x031\"$\n\x0b\n\x04\x04\
    \0\x02\x16\x12\x032\x08'\n\x0c\n\x05\x04\0\x02\x16\x06\x12\x032\x08\x14\
    \n\x0c\n\x05\x04\0\x02\x16\x01\x12\x032\x15!\n\x0c\n\x05\x04\0\x02\x16\
    \x03\x12\x032$&b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
