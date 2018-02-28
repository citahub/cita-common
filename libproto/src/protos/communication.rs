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
pub struct InnerMessage {
    // message oneof groups
    pub content: ::std::option::Option<InnerMessage_oneof_content>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InnerMessage {}

#[derive(Clone,PartialEq)]
pub enum InnerMessage_oneof_content {
    RawBytes(::std::vec::Vec<u8>),
    Request(super::request::Request),
    Response(super::response::Response),
    SyncRequest(super::sync::SyncRequest),
    SyncResponse(super::sync::SyncResponse),
    Status(super::blockchain::Status),
    RichStatus(super::blockchain::RichStatus),
    SignedProposal(super::consensus::SignedProposal),
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
}

impl InnerMessage {
    pub fn new() -> InnerMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InnerMessage {
        static mut instance: ::protobuf::lazy::Lazy<InnerMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InnerMessage,
        };
        unsafe {
            instance.get(InnerMessage::new)
        }
    }

    // bytes RawBytes = 1;

    pub fn clear_RawBytes(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_RawBytes(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::RawBytes(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_RawBytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::RawBytes(v))
    }

    // Mutable pointer to the field.
    pub fn mut_RawBytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::RawBytes(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::RawBytes(::std::vec::Vec::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::RawBytes(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_RawBytes(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_RawBytes() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::RawBytes(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_RawBytes(&self) -> &[u8] {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::RawBytes(ref v)) => v,
            _ => &[],
        }
    }

    // .Request Request = 2;

    pub fn clear_Request(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Request(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Request(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Request(&mut self, v: super::request::Request) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Request(v))
    }

    // Mutable pointer to the field.
    pub fn mut_Request(&mut self) -> &mut super::request::Request {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::Request(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Request(super::request::Request::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Request(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_Request(&mut self) -> super::request::Request {
        if self.has_Request() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::Request(v)) => v,
                _ => panic!(),
            }
        } else {
            super::request::Request::new()
        }
    }

    pub fn get_Request(&self) -> &super::request::Request {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Request(ref v)) => v,
            _ => super::request::Request::default_instance(),
        }
    }

    // .Response Response = 3;

    pub fn clear_Response(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Response(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Response(&mut self, v: super::response::Response) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Response(v))
    }

    // Mutable pointer to the field.
    pub fn mut_Response(&mut self) -> &mut super::response::Response {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::Response(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Response(super::response::Response::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_Response(&mut self) -> super::response::Response {
        if self.has_Response() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::Response(v)) => v,
                _ => panic!(),
            }
        } else {
            super::response::Response::new()
        }
    }

    pub fn get_Response(&self) -> &super::response::Response {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Response(ref v)) => v,
            _ => super::response::Response::default_instance(),
        }
    }

    // .SyncRequest SyncRequest = 4;

    pub fn clear_SyncRequest(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_SyncRequest(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::SyncRequest(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_SyncRequest(&mut self, v: super::sync::SyncRequest) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::SyncRequest(v))
    }

    // Mutable pointer to the field.
    pub fn mut_SyncRequest(&mut self) -> &mut super::sync::SyncRequest {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::SyncRequest(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::SyncRequest(super::sync::SyncRequest::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::SyncRequest(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_SyncRequest(&mut self) -> super::sync::SyncRequest {
        if self.has_SyncRequest() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::SyncRequest(v)) => v,
                _ => panic!(),
            }
        } else {
            super::sync::SyncRequest::new()
        }
    }

    pub fn get_SyncRequest(&self) -> &super::sync::SyncRequest {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::SyncRequest(ref v)) => v,
            _ => super::sync::SyncRequest::default_instance(),
        }
    }

    // .SyncResponse SyncResponse = 5;

    pub fn clear_SyncResponse(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_SyncResponse(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::SyncResponse(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_SyncResponse(&mut self, v: super::sync::SyncResponse) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::SyncResponse(v))
    }

    // Mutable pointer to the field.
    pub fn mut_SyncResponse(&mut self) -> &mut super::sync::SyncResponse {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::SyncResponse(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::SyncResponse(super::sync::SyncResponse::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::SyncResponse(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_SyncResponse(&mut self) -> super::sync::SyncResponse {
        if self.has_SyncResponse() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::SyncResponse(v)) => v,
                _ => panic!(),
            }
        } else {
            super::sync::SyncResponse::new()
        }
    }

    pub fn get_SyncResponse(&self) -> &super::sync::SyncResponse {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::SyncResponse(ref v)) => v,
            _ => super::sync::SyncResponse::default_instance(),
        }
    }

    // .Status Status = 6;

    pub fn clear_Status(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Status(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Status(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Status(&mut self, v: super::blockchain::Status) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Status(v))
    }

    // Mutable pointer to the field.
    pub fn mut_Status(&mut self) -> &mut super::blockchain::Status {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::Status(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Status(super::blockchain::Status::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Status(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_Status(&mut self) -> super::blockchain::Status {
        if self.has_Status() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::Status(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::Status::new()
        }
    }

    pub fn get_Status(&self) -> &super::blockchain::Status {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Status(ref v)) => v,
            _ => super::blockchain::Status::default_instance(),
        }
    }

    // .RichStatus RichStatus = 7;

    pub fn clear_RichStatus(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_RichStatus(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::RichStatus(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_RichStatus(&mut self, v: super::blockchain::RichStatus) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::RichStatus(v))
    }

    // Mutable pointer to the field.
    pub fn mut_RichStatus(&mut self) -> &mut super::blockchain::RichStatus {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::RichStatus(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::RichStatus(super::blockchain::RichStatus::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::RichStatus(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_RichStatus(&mut self) -> super::blockchain::RichStatus {
        if self.has_RichStatus() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::RichStatus(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::RichStatus::new()
        }
    }

    pub fn get_RichStatus(&self) -> &super::blockchain::RichStatus {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::RichStatus(ref v)) => v,
            _ => super::blockchain::RichStatus::default_instance(),
        }
    }

    // .SignedProposal SignedProposal = 8;

    pub fn clear_SignedProposal(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_SignedProposal(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::SignedProposal(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_SignedProposal(&mut self, v: super::consensus::SignedProposal) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::SignedProposal(v))
    }

    // Mutable pointer to the field.
    pub fn mut_SignedProposal(&mut self) -> &mut super::consensus::SignedProposal {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::SignedProposal(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::SignedProposal(super::consensus::SignedProposal::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::SignedProposal(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_SignedProposal(&mut self) -> super::consensus::SignedProposal {
        if self.has_SignedProposal() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::SignedProposal(v)) => v,
                _ => panic!(),
            }
        } else {
            super::consensus::SignedProposal::new()
        }
    }

    pub fn get_SignedProposal(&self) -> &super::consensus::SignedProposal {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::SignedProposal(ref v)) => v,
            _ => super::consensus::SignedProposal::default_instance(),
        }
    }

    // .Block Block = 9;

    pub fn clear_Block(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Block(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Block(&mut self, v: super::blockchain::Block) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_Block(&mut self) -> &mut super::blockchain::Block {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::Block(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Block(super::blockchain::Block::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_Block(&mut self) -> super::blockchain::Block {
        if self.has_Block() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::Block(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::Block::new()
        }
    }

    pub fn get_Block(&self) -> &super::blockchain::Block {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::Block(ref v)) => v,
            _ => super::blockchain::Block::default_instance(),
        }
    }

    // .BlockWithProof BlockWithProof = 10;

    pub fn clear_BlockWithProof(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_BlockWithProof(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockWithProof(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BlockWithProof(&mut self, v: super::blockchain::BlockWithProof) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockWithProof(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BlockWithProof(&mut self) -> &mut super::blockchain::BlockWithProof {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::BlockWithProof(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockWithProof(super::blockchain::BlockWithProof::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockWithProof(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BlockWithProof(&mut self) -> super::blockchain::BlockWithProof {
        if self.has_BlockWithProof() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::BlockWithProof(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::BlockWithProof::new()
        }
    }

    pub fn get_BlockWithProof(&self) -> &super::blockchain::BlockWithProof {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockWithProof(ref v)) => v,
            _ => super::blockchain::BlockWithProof::default_instance(),
        }
    }

    // .BlockHeader BlockHeader = 11;

    pub fn clear_BlockHeader(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_BlockHeader(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockHeader(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BlockHeader(&mut self, v: super::blockchain::BlockHeader) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockHeader(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BlockHeader(&mut self) -> &mut super::blockchain::BlockHeader {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::BlockHeader(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockHeader(super::blockchain::BlockHeader::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockHeader(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BlockHeader(&mut self) -> super::blockchain::BlockHeader {
        if self.has_BlockHeader() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::BlockHeader(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::BlockHeader::new()
        }
    }

    pub fn get_BlockHeader(&self) -> &super::blockchain::BlockHeader {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockHeader(ref v)) => v,
            _ => super::blockchain::BlockHeader::default_instance(),
        }
    }

    // .BlockTxs BlockTxs = 12;

    pub fn clear_BlockTxs(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_BlockTxs(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BlockTxs(&mut self, v: super::blockchain::BlockTxs) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BlockTxs(&mut self) -> &mut super::blockchain::BlockTxs {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxs(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxs(super::blockchain::BlockTxs::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BlockTxs(&mut self) -> super::blockchain::BlockTxs {
        if self.has_BlockTxs() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxs(v)) => v,
                _ => panic!(),
            }
        } else {
            super::blockchain::BlockTxs::new()
        }
    }

    pub fn get_BlockTxs(&self) -> &super::blockchain::BlockTxs {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxs(ref v)) => v,
            _ => super::blockchain::BlockTxs::default_instance(),
        }
    }

    // .BlockTxHashes BlockTxHashes = 13;

    pub fn clear_BlockTxHashes(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_BlockTxHashes(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashes(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BlockTxHashes(&mut self, v: super::auth::BlockTxHashes) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashes(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BlockTxHashes(&mut self) -> &mut super::auth::BlockTxHashes {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashes(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashes(super::auth::BlockTxHashes::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashes(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BlockTxHashes(&mut self) -> super::auth::BlockTxHashes {
        if self.has_BlockTxHashes() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashes(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::BlockTxHashes::new()
        }
    }

    pub fn get_BlockTxHashes(&self) -> &super::auth::BlockTxHashes {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashes(ref v)) => v,
            _ => super::auth::BlockTxHashes::default_instance(),
        }
    }

    // .BlockTxHashesReq BlockTxHashesReq = 14;

    pub fn clear_BlockTxHashesReq(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_BlockTxHashesReq(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashesReq(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BlockTxHashesReq(&mut self, v: super::auth::BlockTxHashesReq) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashesReq(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BlockTxHashesReq(&mut self) -> &mut super::auth::BlockTxHashesReq {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashesReq(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashesReq(super::auth::BlockTxHashesReq::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashesReq(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BlockTxHashesReq(&mut self) -> super::auth::BlockTxHashesReq {
        if self.has_BlockTxHashesReq() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashesReq(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::BlockTxHashesReq::new()
        }
    }

    pub fn get_BlockTxHashesReq(&self) -> &super::auth::BlockTxHashesReq {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashesReq(ref v)) => v,
            _ => super::auth::BlockTxHashesReq::default_instance(),
        }
    }

    // .VerifyTxReq VerifyTxReq = 15;

    pub fn clear_VerifyTxReq(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_VerifyTxReq(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxReq(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_VerifyTxReq(&mut self, v: super::auth::VerifyTxReq) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxReq(v))
    }

    // Mutable pointer to the field.
    pub fn mut_VerifyTxReq(&mut self) -> &mut super::auth::VerifyTxReq {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxReq(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxReq(super::auth::VerifyTxReq::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxReq(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_VerifyTxReq(&mut self) -> super::auth::VerifyTxReq {
        if self.has_VerifyTxReq() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxReq(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::VerifyTxReq::new()
        }
    }

    pub fn get_VerifyTxReq(&self) -> &super::auth::VerifyTxReq {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxReq(ref v)) => v,
            _ => super::auth::VerifyTxReq::default_instance(),
        }
    }

    // .VerifyTxResp VerifyTxResp = 16;

    pub fn clear_VerifyTxResp(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_VerifyTxResp(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxResp(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_VerifyTxResp(&mut self, v: super::auth::VerifyTxResp) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxResp(v))
    }

    // Mutable pointer to the field.
    pub fn mut_VerifyTxResp(&mut self) -> &mut super::auth::VerifyTxResp {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxResp(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxResp(super::auth::VerifyTxResp::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxResp(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_VerifyTxResp(&mut self) -> super::auth::VerifyTxResp {
        if self.has_VerifyTxResp() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxResp(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::VerifyTxResp::new()
        }
    }

    pub fn get_VerifyTxResp(&self) -> &super::auth::VerifyTxResp {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxResp(ref v)) => v,
            _ => super::auth::VerifyTxResp::default_instance(),
        }
    }

    // .VerifyBlockReq VerifyBlockReq = 17;

    pub fn clear_VerifyBlockReq(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_VerifyBlockReq(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockReq(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_VerifyBlockReq(&mut self, v: super::auth::VerifyBlockReq) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockReq(v))
    }

    // Mutable pointer to the field.
    pub fn mut_VerifyBlockReq(&mut self) -> &mut super::auth::VerifyBlockReq {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockReq(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockReq(super::auth::VerifyBlockReq::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockReq(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_VerifyBlockReq(&mut self) -> super::auth::VerifyBlockReq {
        if self.has_VerifyBlockReq() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockReq(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::VerifyBlockReq::new()
        }
    }

    pub fn get_VerifyBlockReq(&self) -> &super::auth::VerifyBlockReq {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockReq(ref v)) => v,
            _ => super::auth::VerifyBlockReq::default_instance(),
        }
    }

    // .VerifyBlockResp VerifyBlockResp = 18;

    pub fn clear_VerifyBlockResp(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_VerifyBlockResp(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockResp(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_VerifyBlockResp(&mut self, v: super::auth::VerifyBlockResp) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockResp(v))
    }

    // Mutable pointer to the field.
    pub fn mut_VerifyBlockResp(&mut self) -> &mut super::auth::VerifyBlockResp {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockResp(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockResp(super::auth::VerifyBlockResp::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockResp(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_VerifyBlockResp(&mut self) -> super::auth::VerifyBlockResp {
        if self.has_VerifyBlockResp() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockResp(v)) => v,
                _ => panic!(),
            }
        } else {
            super::auth::VerifyBlockResp::new()
        }
    }

    pub fn get_VerifyBlockResp(&self) -> &super::auth::VerifyBlockResp {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockResp(ref v)) => v,
            _ => super::auth::VerifyBlockResp::default_instance(),
        }
    }

    // .ExecutedResult ExecutedResult = 19;

    pub fn clear_ExecutedResult(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_ExecutedResult(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::ExecutedResult(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ExecutedResult(&mut self, v: super::executor::ExecutedResult) {
        self.content = ::std::option::Option::Some(InnerMessage_oneof_content::ExecutedResult(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ExecutedResult(&mut self) -> &mut super::executor::ExecutedResult {
        if let ::std::option::Option::Some(InnerMessage_oneof_content::ExecutedResult(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(InnerMessage_oneof_content::ExecutedResult(super::executor::ExecutedResult::new()));
        }
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::ExecutedResult(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ExecutedResult(&mut self) -> super::executor::ExecutedResult {
        if self.has_ExecutedResult() {
            match self.content.take() {
                ::std::option::Option::Some(InnerMessage_oneof_content::ExecutedResult(v)) => v,
                _ => panic!(),
            }
        } else {
            super::executor::ExecutedResult::new()
        }
    }

    pub fn get_ExecutedResult(&self) -> &super::executor::ExecutedResult {
        match self.content {
            ::std::option::Option::Some(InnerMessage_oneof_content::ExecutedResult(ref v)) => v,
            _ => super::executor::ExecutedResult::default_instance(),
        }
    }
}

impl ::protobuf::Message for InnerMessage {
    fn is_initialized(&self) -> bool {
        if let Some(InnerMessage_oneof_content::Request(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::Response(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::SyncRequest(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::SyncResponse(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::Status(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::RichStatus(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::SignedProposal(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::Block(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::BlockWithProof(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::BlockHeader(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::BlockTxs(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::BlockTxHashes(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::BlockTxHashesReq(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::VerifyTxReq(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::VerifyTxResp(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::VerifyBlockReq(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::VerifyBlockResp(ref v)) = self.content {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(InnerMessage_oneof_content::ExecutedResult(ref v)) = self.content {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::RawBytes(is.read_bytes()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Request(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Response(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::SyncRequest(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::SyncResponse(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Status(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::RichStatus(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::SignedProposal(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::Block(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockWithProof(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockHeader(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxs(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashes(is.read_message()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::BlockTxHashesReq(is.read_message()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxReq(is.read_message()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyTxResp(is.read_message()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockReq(is.read_message()?));
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::VerifyBlockResp(is.read_message()?));
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.content = ::std::option::Option::Some(InnerMessage_oneof_content::ExecutedResult(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &InnerMessage_oneof_content::RawBytes(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &InnerMessage_oneof_content::Request(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::Response(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::SyncRequest(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::SyncResponse(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::Status(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::RichStatus(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::SignedProposal(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::Block(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::BlockWithProof(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::BlockHeader(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::BlockTxs(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::BlockTxHashes(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::BlockTxHashesReq(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::VerifyTxReq(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::VerifyTxResp(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::VerifyBlockReq(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::VerifyBlockResp(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InnerMessage_oneof_content::ExecutedResult(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &InnerMessage_oneof_content::RawBytes(ref v) => {
                    os.write_bytes(1, v)?;
                },
                &InnerMessage_oneof_content::Request(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::Response(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::SyncRequest(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::SyncResponse(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::Status(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::RichStatus(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::SignedProposal(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::Block(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::BlockWithProof(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::BlockHeader(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::BlockTxs(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::BlockTxHashes(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::BlockTxHashesReq(ref v) => {
                    os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::VerifyTxReq(ref v) => {
                    os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::VerifyTxResp(ref v) => {
                    os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::VerifyBlockReq(ref v) => {
                    os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::VerifyBlockResp(ref v) => {
                    os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InnerMessage_oneof_content::ExecutedResult(ref v) => {
                    os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for InnerMessage {
    fn new() -> InnerMessage {
        InnerMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<InnerMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "RawBytes",
                    InnerMessage::has_RawBytes,
                    InnerMessage::get_RawBytes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::request::Request>(
                    "Request",
                    InnerMessage::has_Request,
                    InnerMessage::get_Request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::response::Response>(
                    "Response",
                    InnerMessage::has_Response,
                    InnerMessage::get_Response,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::sync::SyncRequest>(
                    "SyncRequest",
                    InnerMessage::has_SyncRequest,
                    InnerMessage::get_SyncRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::sync::SyncResponse>(
                    "SyncResponse",
                    InnerMessage::has_SyncResponse,
                    InnerMessage::get_SyncResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::Status>(
                    "Status",
                    InnerMessage::has_Status,
                    InnerMessage::get_Status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::RichStatus>(
                    "RichStatus",
                    InnerMessage::has_RichStatus,
                    InnerMessage::get_RichStatus,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::consensus::SignedProposal>(
                    "SignedProposal",
                    InnerMessage::has_SignedProposal,
                    InnerMessage::get_SignedProposal,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::Block>(
                    "Block",
                    InnerMessage::has_Block,
                    InnerMessage::get_Block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::BlockWithProof>(
                    "BlockWithProof",
                    InnerMessage::has_BlockWithProof,
                    InnerMessage::get_BlockWithProof,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::BlockHeader>(
                    "BlockHeader",
                    InnerMessage::has_BlockHeader,
                    InnerMessage::get_BlockHeader,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::blockchain::BlockTxs>(
                    "BlockTxs",
                    InnerMessage::has_BlockTxs,
                    InnerMessage::get_BlockTxs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::BlockTxHashes>(
                    "BlockTxHashes",
                    InnerMessage::has_BlockTxHashes,
                    InnerMessage::get_BlockTxHashes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::BlockTxHashesReq>(
                    "BlockTxHashesReq",
                    InnerMessage::has_BlockTxHashesReq,
                    InnerMessage::get_BlockTxHashesReq,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::VerifyTxReq>(
                    "VerifyTxReq",
                    InnerMessage::has_VerifyTxReq,
                    InnerMessage::get_VerifyTxReq,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::VerifyTxResp>(
                    "VerifyTxResp",
                    InnerMessage::has_VerifyTxResp,
                    InnerMessage::get_VerifyTxResp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::VerifyBlockReq>(
                    "VerifyBlockReq",
                    InnerMessage::has_VerifyBlockReq,
                    InnerMessage::get_VerifyBlockReq,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::auth::VerifyBlockResp>(
                    "VerifyBlockResp",
                    InnerMessage::has_VerifyBlockResp,
                    InnerMessage::get_VerifyBlockResp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::executor::ExecutedResult>(
                    "ExecutedResult",
                    InnerMessage::has_ExecutedResult,
                    InnerMessage::get_ExecutedResult,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InnerMessage>(
                    "InnerMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InnerMessage {
    fn clear(&mut self) {
        self.clear_RawBytes();
        self.clear_Request();
        self.clear_Response();
        self.clear_SyncRequest();
        self.clear_SyncResponse();
        self.clear_Status();
        self.clear_RichStatus();
        self.clear_SignedProposal();
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
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InnerMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InnerMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13communication.proto\x1a\rrequest.proto\x1a\x0eresponse.proto\x1a\n\
    sync.proto\x1a\x10blockchain.proto\x1a\x0fconsensus.proto\x1a\nauth.prot\
    o\x1a\x0eexecutor.proto\"\xc4\x07\n\x0cInnerMessage\x12\x1c\n\x08RawByte\
    s\x18\x01\x20\x01(\x0cH\0R\x08RawBytes\x12$\n\x07Request\x18\x02\x20\x01\
    (\x0b2\x08.RequestH\0R\x07Request\x12'\n\x08Response\x18\x03\x20\x01(\
    \x0b2\t.ResponseH\0R\x08Response\x120\n\x0bSyncRequest\x18\x04\x20\x01(\
    \x0b2\x0c.SyncRequestH\0R\x0bSyncRequest\x123\n\x0cSyncResponse\x18\x05\
    \x20\x01(\x0b2\r.SyncResponseH\0R\x0cSyncResponse\x12!\n\x06Status\x18\
    \x06\x20\x01(\x0b2\x07.StatusH\0R\x06Status\x12-\n\nRichStatus\x18\x07\
    \x20\x01(\x0b2\x0b.RichStatusH\0R\nRichStatus\x129\n\x0eSignedProposal\
    \x18\x08\x20\x01(\x0b2\x0f.SignedProposalH\0R\x0eSignedProposal\x12\x1e\
    \n\x05Block\x18\t\x20\x01(\x0b2\x06.BlockH\0R\x05Block\x129\n\x0eBlockWi\
    thProof\x18\n\x20\x01(\x0b2\x0f.BlockWithProofH\0R\x0eBlockWithProof\x12\
    0\n\x0bBlockHeader\x18\x0b\x20\x01(\x0b2\x0c.BlockHeaderH\0R\x0bBlockHea\
    der\x12'\n\x08BlockTxs\x18\x0c\x20\x01(\x0b2\t.BlockTxsH\0R\x08BlockTxs\
    \x126\n\rBlockTxHashes\x18\r\x20\x01(\x0b2\x0e.BlockTxHashesH\0R\rBlockT\
    xHashes\x12?\n\x10BlockTxHashesReq\x18\x0e\x20\x01(\x0b2\x11.BlockTxHash\
    esReqH\0R\x10BlockTxHashesReq\x120\n\x0bVerifyTxReq\x18\x0f\x20\x01(\x0b\
    2\x0c.VerifyTxReqH\0R\x0bVerifyTxReq\x123\n\x0cVerifyTxResp\x18\x10\x20\
    \x01(\x0b2\r.VerifyTxRespH\0R\x0cVerifyTxResp\x129\n\x0eVerifyBlockReq\
    \x18\x11\x20\x01(\x0b2\x0f.VerifyBlockReqH\0R\x0eVerifyBlockReq\x12<\n\
    \x0fVerifyBlockResp\x18\x12\x20\x01(\x0b2\x10.VerifyBlockRespH\0R\x0fVer\
    ifyBlockResp\x129\n\x0eExecutedResult\x18\x13\x20\x01(\x0b2\x0f.Executed\
    ResultH\0R\x0eExecutedResultB\t\n\x07contentJ\xa8\t\n\x06\x12\x04\0\0*\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07\x16\
    \n\t\n\x02\x03\x01\x12\x03\x03\x07\x17\n\t\n\x02\x03\x02\x12\x03\x04\x07\
    \x13\n\t\n\x02\x03\x03\x12\x03\x05\x07\x19\n\t\n\x02\x03\x04\x12\x03\x06\
    \x07\x18\n\t\n\x02\x03\x05\x12\x03\x07\x07\x13\n\t\n\x02\x03\x06\x12\x03\
    \x08\x07\x17\n\n\n\x02\x04\0\x12\x04\n\0*\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\n\x08\x14\n\x0c\n\x04\x04\0\x08\0\x12\x04\x0c\x04)\x05\n\x0c\n\x05\
    \x04\0\x08\0\x01\x12\x03\x0c\n\x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0e\
    \x08\x1b\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0e\x08\r\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x0e\x0e\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0e\
    \x19\x1a\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x10\x08\x1c\n\x0c\n\x05\x04\0\
    \x02\x01\x06\x12\x03\x10\x08\x0f\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x10\x10\x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x10\x1a\x1b\n\x0b\n\
    \x04\x04\0\x02\x02\x12\x03\x11\x08\x1e\n\x0c\n\x05\x04\0\x02\x02\x06\x12\
    \x03\x11\x08\x10\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x11\x11\x19\n\x0c\
    \n\x05\x04\0\x02\x02\x03\x12\x03\x11\x1c\x1d\n\x0b\n\x04\x04\0\x02\x03\
    \x12\x03\x13\x08$\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\x13\x08\x13\n\
    \x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x13\x14\x1f\n\x0c\n\x05\x04\0\x02\
    \x03\x03\x12\x03\x13\"#\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x14\x08&\n\x0c\
    \n\x05\x04\0\x02\x04\x06\x12\x03\x14\x08\x14\n\x0c\n\x05\x04\0\x02\x04\
    \x01\x12\x03\x14\x15!\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x14$%\n\x0b\
    \n\x04\x04\0\x02\x05\x12\x03\x16\x08\x1a\n\x0c\n\x05\x04\0\x02\x05\x06\
    \x12\x03\x16\x08\x0e\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x16\x0f\x15\n\
    \x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x16\x18\x19\n\x0b\n\x04\x04\0\x02\
    \x06\x12\x03\x17\x08\"\n\x0c\n\x05\x04\0\x02\x06\x06\x12\x03\x17\x08\x12\
    \n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x17\x13\x1d\n\x0c\n\x05\x04\0\x02\
    \x06\x03\x12\x03\x17\x20!\n\x0b\n\x04\x04\0\x02\x07\x12\x03\x19\x08*\n\
    \x0c\n\x05\x04\0\x02\x07\x06\x12\x03\x19\x08\x16\n\x0c\n\x05\x04\0\x02\
    \x07\x01\x12\x03\x19\x17%\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x19()\n\
    \x0b\n\x04\x04\0\x02\x08\x12\x03\x1b\x08\x18\n\x0c\n\x05\x04\0\x02\x08\
    \x06\x12\x03\x1b\x08\r\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x1b\x0e\x13\
    \n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03\x1b\x16\x17\n\x0b\n\x04\x04\0\x02\
    \t\x12\x03\x1c\x08+\n\x0c\n\x05\x04\0\x02\t\x06\x12\x03\x1c\x08\x16\n\
    \x0c\n\x05\x04\0\x02\t\x01\x12\x03\x1c\x17%\n\x0c\n\x05\x04\0\x02\t\x03\
    \x12\x03\x1c(*\n\x0b\n\x04\x04\0\x02\n\x12\x03\x1d\x08%\n\x0c\n\x05\x04\
    \0\x02\n\x06\x12\x03\x1d\x08\x13\n\x0c\n\x05\x04\0\x02\n\x01\x12\x03\x1d\
    \x14\x1f\n\x0c\n\x05\x04\0\x02\n\x03\x12\x03\x1d\"$\n\x0b\n\x04\x04\0\
    \x02\x0b\x12\x03\x1e\x08\x1f\n\x0c\n\x05\x04\0\x02\x0b\x06\x12\x03\x1e\
    \x08\x10\n\x0c\n\x05\x04\0\x02\x0b\x01\x12\x03\x1e\x11\x19\n\x0c\n\x05\
    \x04\0\x02\x0b\x03\x12\x03\x1e\x1c\x1e\n\x0b\n\x04\x04\0\x02\x0c\x12\x03\
    \x20\x08)\n\x0c\n\x05\x04\0\x02\x0c\x06\x12\x03\x20\x08\x15\n\x0c\n\x05\
    \x04\0\x02\x0c\x01\x12\x03\x20\x16#\n\x0c\n\x05\x04\0\x02\x0c\x03\x12\
    \x03\x20&(\n\x0b\n\x04\x04\0\x02\r\x12\x03!\x08/\n\x0c\n\x05\x04\0\x02\r\
    \x06\x12\x03!\x08\x18\n\x0c\n\x05\x04\0\x02\r\x01\x12\x03!\x19)\n\x0c\n\
    \x05\x04\0\x02\r\x03\x12\x03!,.\n\x0b\n\x04\x04\0\x02\x0e\x12\x03#\x08%\
    \n\x0c\n\x05\x04\0\x02\x0e\x06\x12\x03#\x08\x13\n\x0c\n\x05\x04\0\x02\
    \x0e\x01\x12\x03#\x14\x1f\n\x0c\n\x05\x04\0\x02\x0e\x03\x12\x03#\"$\n\
    \x0b\n\x04\x04\0\x02\x0f\x12\x03$\x08'\n\x0c\n\x05\x04\0\x02\x0f\x06\x12\
    \x03$\x08\x14\n\x0c\n\x05\x04\0\x02\x0f\x01\x12\x03$\x15!\n\x0c\n\x05\
    \x04\0\x02\x0f\x03\x12\x03$$&\n\x0b\n\x04\x04\0\x02\x10\x12\x03%\x08+\n\
    \x0c\n\x05\x04\0\x02\x10\x06\x12\x03%\x08\x16\n\x0c\n\x05\x04\0\x02\x10\
    \x01\x12\x03%\x17%\n\x0c\n\x05\x04\0\x02\x10\x03\x12\x03%(*\n\x0b\n\x04\
    \x04\0\x02\x11\x12\x03&\x08-\n\x0c\n\x05\x04\0\x02\x11\x06\x12\x03&\x08\
    \x17\n\x0c\n\x05\x04\0\x02\x11\x01\x12\x03&\x18'\n\x0c\n\x05\x04\0\x02\
    \x11\x03\x12\x03&*,\n\x0b\n\x04\x04\0\x02\x12\x12\x03(\x08+\n\x0c\n\x05\
    \x04\0\x02\x12\x06\x12\x03(\x08\x16\n\x0c\n\x05\x04\0\x02\x12\x01\x12\
    \x03(\x17%\n\x0c\n\x05\x04\0\x02\x12\x03\x12\x03((*b\x06proto3\
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
