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
pub struct SnapshotReq {
    // message fields
    pub cmd: Cmd,
    pub start_height: u64,
    pub end_height: u64,
    pub file: ::std::string::String,
    pub proof: ::protobuf::SingularPtrField<super::blockchain::Proof>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotReq {}

impl SnapshotReq {
    pub fn new() -> SnapshotReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotReq {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotReq,
        };
        unsafe {
            instance.get(SnapshotReq::new)
        }
    }

    // .Cmd cmd = 1;

    pub fn clear_cmd(&mut self) {
        self.cmd = Cmd::Begin;
    }

    // Param is passed by value, moved
    pub fn set_cmd(&mut self, v: Cmd) {
        self.cmd = v;
    }

    pub fn get_cmd(&self) -> Cmd {
        self.cmd
    }

    fn get_cmd_for_reflect(&self) -> &Cmd {
        &self.cmd
    }

    fn mut_cmd_for_reflect(&mut self) -> &mut Cmd {
        &mut self.cmd
    }

    // uint64 start_height = 2;

    pub fn clear_start_height(&mut self) {
        self.start_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_height(&mut self, v: u64) {
        self.start_height = v;
    }

    pub fn get_start_height(&self) -> u64 {
        self.start_height
    }

    fn get_start_height_for_reflect(&self) -> &u64 {
        &self.start_height
    }

    fn mut_start_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_height
    }

    // uint64 end_height = 3;

    pub fn clear_end_height(&mut self) {
        self.end_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_end_height(&mut self, v: u64) {
        self.end_height = v;
    }

    pub fn get_end_height(&self) -> u64 {
        self.end_height
    }

    fn get_end_height_for_reflect(&self) -> &u64 {
        &self.end_height
    }

    fn mut_end_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.end_height
    }

    // string file = 4;

    pub fn clear_file(&mut self) {
        self.file.clear();
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: ::std::string::String) {
        self.file = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file(&mut self) -> &mut ::std::string::String {
        &mut self.file
    }

    // Take field
    pub fn take_file(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.file, ::std::string::String::new())
    }

    pub fn get_file(&self) -> &str {
        &self.file
    }

    fn get_file_for_reflect(&self) -> &::std::string::String {
        &self.file
    }

    fn mut_file_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.file
    }

    // .Proof proof = 5;

    pub fn clear_proof(&mut self) {
        self.proof.clear();
    }

    pub fn has_proof(&self) -> bool {
        self.proof.is_some()
    }

    // Param is passed by value, moved
    pub fn set_proof(&mut self, v: super::blockchain::Proof) {
        self.proof = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proof(&mut self) -> &mut super::blockchain::Proof {
        if self.proof.is_none() {
            self.proof.set_default();
        }
        self.proof.as_mut().unwrap()
    }

    // Take field
    pub fn take_proof(&mut self) -> super::blockchain::Proof {
        self.proof.take().unwrap_or_else(|| super::blockchain::Proof::new())
    }

    pub fn get_proof(&self) -> &super::blockchain::Proof {
        self.proof.as_ref().unwrap_or_else(|| super::blockchain::Proof::default_instance())
    }

    fn get_proof_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain::Proof> {
        &self.proof
    }

    fn mut_proof_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain::Proof> {
        &mut self.proof
    }
}

impl ::protobuf::Message for SnapshotReq {
    fn is_initialized(&self) -> bool {
        for v in &self.proof {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    let tmp = is.read_enum()?;
                    self.cmd = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_height = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.end_height = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.file)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.proof)?;
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
        if self.cmd != Cmd::Begin {
            my_size += ::protobuf::rt::enum_size(1, self.cmd);
        }
        if self.start_height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.start_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.end_height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.end_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.file.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.file);
        }
        if let Some(ref v) = self.proof.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.cmd != Cmd::Begin {
            os.write_enum(1, self.cmd.value())?;
        }
        if self.start_height != 0 {
            os.write_uint64(2, self.start_height)?;
        }
        if self.end_height != 0 {
            os.write_uint64(3, self.end_height)?;
        }
        if !self.file.is_empty() {
            os.write_string(4, &self.file)?;
        }
        if let Some(ref v) = self.proof.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for SnapshotReq {
    fn new() -> SnapshotReq {
        SnapshotReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Cmd>>(
                    "cmd",
                    SnapshotReq::get_cmd_for_reflect,
                    SnapshotReq::mut_cmd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_height",
                    SnapshotReq::get_start_height_for_reflect,
                    SnapshotReq::mut_start_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "end_height",
                    SnapshotReq::get_end_height_for_reflect,
                    SnapshotReq::mut_end_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "file",
                    SnapshotReq::get_file_for_reflect,
                    SnapshotReq::mut_file_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::Proof>>(
                    "proof",
                    SnapshotReq::get_proof_for_reflect,
                    SnapshotReq::mut_proof_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotReq>(
                    "SnapshotReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotReq {
    fn clear(&mut self) {
        self.clear_cmd();
        self.clear_start_height();
        self.clear_end_height();
        self.clear_file();
        self.clear_proof();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotResp {
    // message fields
    pub resp: Resp,
    pub proof: ::protobuf::SingularPtrField<super::blockchain::Proof>,
    pub height: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotResp {}

impl SnapshotResp {
    pub fn new() -> SnapshotResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotResp {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotResp,
        };
        unsafe {
            instance.get(SnapshotResp::new)
        }
    }

    // .Resp resp = 1;

    pub fn clear_resp(&mut self) {
        self.resp = Resp::BeginAck;
    }

    // Param is passed by value, moved
    pub fn set_resp(&mut self, v: Resp) {
        self.resp = v;
    }

    pub fn get_resp(&self) -> Resp {
        self.resp
    }

    fn get_resp_for_reflect(&self) -> &Resp {
        &self.resp
    }

    fn mut_resp_for_reflect(&mut self) -> &mut Resp {
        &mut self.resp
    }

    // .Proof proof = 2;

    pub fn clear_proof(&mut self) {
        self.proof.clear();
    }

    pub fn has_proof(&self) -> bool {
        self.proof.is_some()
    }

    // Param is passed by value, moved
    pub fn set_proof(&mut self, v: super::blockchain::Proof) {
        self.proof = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proof(&mut self) -> &mut super::blockchain::Proof {
        if self.proof.is_none() {
            self.proof.set_default();
        }
        self.proof.as_mut().unwrap()
    }

    // Take field
    pub fn take_proof(&mut self) -> super::blockchain::Proof {
        self.proof.take().unwrap_or_else(|| super::blockchain::Proof::new())
    }

    pub fn get_proof(&self) -> &super::blockchain::Proof {
        self.proof.as_ref().unwrap_or_else(|| super::blockchain::Proof::default_instance())
    }

    fn get_proof_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain::Proof> {
        &self.proof
    }

    fn mut_proof_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain::Proof> {
        &mut self.proof
    }

    // uint64 height = 3;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &u64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.height
    }
}

impl ::protobuf::Message for SnapshotResp {
    fn is_initialized(&self) -> bool {
        for v in &self.proof {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    let tmp = is.read_enum()?;
                    self.resp = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.proof)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
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
        if self.resp != Resp::BeginAck {
            my_size += ::protobuf::rt::enum_size(1, self.resp);
        }
        if let Some(ref v) = self.proof.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.resp != Resp::BeginAck {
            os.write_enum(1, self.resp.value())?;
        }
        if let Some(ref v) = self.proof.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.height != 0 {
            os.write_uint64(3, self.height)?;
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

impl ::protobuf::MessageStatic for SnapshotResp {
    fn new() -> SnapshotResp {
        SnapshotResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Resp>>(
                    "resp",
                    SnapshotResp::get_resp_for_reflect,
                    SnapshotResp::mut_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::Proof>>(
                    "proof",
                    SnapshotResp::get_proof_for_reflect,
                    SnapshotResp::mut_proof_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    SnapshotResp::get_height_for_reflect,
                    SnapshotResp::mut_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotResp>(
                    "SnapshotResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotResp {
    fn clear(&mut self) {
        self.clear_resp();
        self.clear_proof();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotResp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Cmd {
    Begin = 0,
    Clear = 1,
    Snapshot = 2,
    Restore = 3,
    End = 4,
}

impl ::protobuf::ProtobufEnum for Cmd {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Cmd> {
        match value {
            0 => ::std::option::Option::Some(Cmd::Begin),
            1 => ::std::option::Option::Some(Cmd::Clear),
            2 => ::std::option::Option::Some(Cmd::Snapshot),
            3 => ::std::option::Option::Some(Cmd::Restore),
            4 => ::std::option::Option::Some(Cmd::End),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Cmd] = &[
            Cmd::Begin,
            Cmd::Clear,
            Cmd::Snapshot,
            Cmd::Restore,
            Cmd::End,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Cmd>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Cmd", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Cmd {
}

impl ::std::default::Default for Cmd {
    fn default() -> Self {
        Cmd::Begin
    }
}

impl ::protobuf::reflect::ProtobufValue for Cmd {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Resp {
    BeginAck = 0,
    ClearAck = 1,
    SnapshotAck = 2,
    RestoreAck = 3,
    EndAck = 4,
}

impl ::protobuf::ProtobufEnum for Resp {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Resp> {
        match value {
            0 => ::std::option::Option::Some(Resp::BeginAck),
            1 => ::std::option::Option::Some(Resp::ClearAck),
            2 => ::std::option::Option::Some(Resp::SnapshotAck),
            3 => ::std::option::Option::Some(Resp::RestoreAck),
            4 => ::std::option::Option::Some(Resp::EndAck),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Resp] = &[
            Resp::BeginAck,
            Resp::ClearAck,
            Resp::SnapshotAck,
            Resp::RestoreAck,
            Resp::EndAck,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Resp>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Resp", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Resp {
}

impl ::std::default::Default for Resp {
    fn default() -> Self {
        Resp::BeginAck
    }
}

impl ::protobuf::reflect::ProtobufValue for Resp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0esnapshot.proto\x1a\x10blockchain.proto\"\x99\x01\n\x0bSnapshotReq\
    \x12\x16\n\x03cmd\x18\x01\x20\x01(\x0e2\x04.CmdR\x03cmd\x12!\n\x0cstart_\
    height\x18\x02\x20\x01(\x04R\x0bstartHeight\x12\x1d\n\nend_height\x18\
    \x03\x20\x01(\x04R\tendHeight\x12\x12\n\x04file\x18\x04\x20\x01(\tR\x04f\
    ile\x12\x1c\n\x05proof\x18\x05\x20\x01(\x0b2\x06.ProofR\x05proof\"_\n\
    \x0cSnapshotResp\x12\x19\n\x04resp\x18\x01\x20\x01(\x0e2\x05.RespR\x04re\
    sp\x12\x1c\n\x05proof\x18\x02\x20\x01(\x0b2\x06.ProofR\x05proof\x12\x16\
    \n\x06height\x18\x03\x20\x01(\x04R\x06height*?\n\x03Cmd\x12\t\n\x05Begin\
    \x10\0\x12\t\n\x05Clear\x10\x01\x12\x0c\n\x08Snapshot\x10\x02\x12\x0b\n\
    \x07Restore\x10\x03\x12\x07\n\x03End\x10\x04*O\n\x04Resp\x12\x0c\n\x08Be\
    ginAck\x10\0\x12\x0c\n\x08ClearAck\x10\x01\x12\x0f\n\x0bSnapshotAck\x10\
    \x02\x12\x0e\n\nRestoreAck\x10\x03\x12\n\n\x06EndAck\x10\x04J\xc7\x08\n\
    \x06\x12\x04\0\0!\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\
    \x12\x03\x02\x07\x19\n\n\n\x02\x05\0\x12\x04\x04\0\n\x01\n\n\n\x03\x05\0\
    \x01\x12\x03\x04\x05\x08\n\x0b\n\x04\x05\0\x02\0\x12\x03\x05\x04\x0e\n\
    \x0c\n\x05\x05\0\x02\0\x01\x12\x03\x05\x04\t\n\x0c\n\x05\x05\0\x02\0\x02\
    \x12\x03\x05\x0c\r\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x06\x04\x0e\n\x0c\n\
    \x05\x05\0\x02\x01\x01\x12\x03\x06\x04\t\n\x0c\n\x05\x05\0\x02\x01\x02\
    \x12\x03\x06\x0c\r\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x07\x04\x11\n\x0c\n\
    \x05\x05\0\x02\x02\x01\x12\x03\x07\x04\x0c\n\x0c\n\x05\x05\0\x02\x02\x02\
    \x12\x03\x07\x0f\x10\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x08\x04\x10\n\x0c\
    \n\x05\x05\0\x02\x03\x01\x12\x03\x08\x04\x0b\n\x0c\n\x05\x05\0\x02\x03\
    \x02\x12\x03\x08\x0e\x0f\n\x0b\n\x04\x05\0\x02\x04\x12\x03\t\x04\x0c\n\
    \x0c\n\x05\x05\0\x02\x04\x01\x12\x03\t\x04\x07\n\x0c\n\x05\x05\0\x02\x04\
    \x02\x12\x03\t\n\x0b\n\n\n\x02\x05\x01\x12\x04\x0c\0\x12\x01\n\n\n\x03\
    \x05\x01\x01\x12\x03\x0c\x05\t\n\x0b\n\x04\x05\x01\x02\0\x12\x03\r\x04\
    \x11\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03\r\x04\x0c\n\x0c\n\x05\x05\x01\
    \x02\0\x02\x12\x03\r\x0f\x10\n\x0b\n\x04\x05\x01\x02\x01\x12\x03\x0e\x04\
    \x11\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03\x0e\x04\x0c\n\x0c\n\x05\x05\
    \x01\x02\x01\x02\x12\x03\x0e\x0f\x10\n\x0b\n\x04\x05\x01\x02\x02\x12\x03\
    \x0f\x04\x14\n\x0c\n\x05\x05\x01\x02\x02\x01\x12\x03\x0f\x04\x0f\n\x0c\n\
    \x05\x05\x01\x02\x02\x02\x12\x03\x0f\x12\x13\n\x0b\n\x04\x05\x01\x02\x03\
    \x12\x03\x10\x04\x13\n\x0c\n\x05\x05\x01\x02\x03\x01\x12\x03\x10\x04\x0e\
    \n\x0c\n\x05\x05\x01\x02\x03\x02\x12\x03\x10\x11\x12\n\x0b\n\x04\x05\x01\
    \x02\x04\x12\x03\x11\x04\x0f\n\x0c\n\x05\x05\x01\x02\x04\x01\x12\x03\x11\
    \x04\n\n\x0c\n\x05\x05\x01\x02\x04\x02\x12\x03\x11\r\x0e\n\n\n\x02\x04\0\
    \x12\x04\x14\0\x1a\x01\n\n\n\x03\x04\0\x01\x12\x03\x14\x08\x13\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x15\x04\x10\n\r\n\x05\x04\0\x02\0\x04\x12\x04\
    \x15\x04\x14\x15\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x15\x04\x07\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x15\x08\x0b\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x15\x0e\x0f\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x16\x04\x1c\n\r\n\x05\
    \x04\0\x02\x01\x04\x12\x04\x16\x04\x15\x10\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\x16\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x16\x0b\x17\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x16\x1a\x1b\n\x0b\n\x04\x04\0\x02\
    \x02\x12\x03\x17\x04\x1a\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x17\x04\x16\
    \x1c\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x17\x04\n\n\x0c\n\x05\x04\0\
    \x02\x02\x01\x12\x03\x17\x0b\x15\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\
    \x17\x18\x19\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x18\x04\x14\n\r\n\x05\x04\
    \0\x02\x03\x04\x12\x04\x18\x04\x17\x1a\n\x0c\n\x05\x04\0\x02\x03\x05\x12\
    \x03\x18\x04\n\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x18\x0b\x0f\n\x0c\n\
    \x05\x04\0\x02\x03\x03\x12\x03\x18\x12\x13\n\x0b\n\x04\x04\0\x02\x04\x12\
    \x03\x19\x04\x14\n\r\n\x05\x04\0\x02\x04\x04\x12\x04\x19\x04\x18\x14\n\
    \x0c\n\x05\x04\0\x02\x04\x06\x12\x03\x19\x04\t\n\x0c\n\x05\x04\0\x02\x04\
    \x01\x12\x03\x19\n\x0f\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x19\x12\x13\
    \n\n\n\x02\x04\x01\x12\x04\x1d\0!\x01\n\n\n\x03\x04\x01\x01\x12\x03\x1d\
    \x08\x14\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x1e\x04\x12\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04\x1e\x04\x1d\x16\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\
    \x1e\x04\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1e\t\r\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03\x1e\x10\x11\n\x0b\n\x04\x04\x01\x02\x01\x12\
    \x03\x1f\x04\x14\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x1f\x04\x1e\x12\n\
    \x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x1f\x04\t\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x03\x1f\n\x0f\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x1f\
    \x12\x13\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x20\x04\x16\n\r\n\x05\x04\
    \x01\x02\x02\x04\x12\x04\x20\x04\x1f\x14\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03\x20\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x20\x0b\x11\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x20\x14\x15b\x06proto3\
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
