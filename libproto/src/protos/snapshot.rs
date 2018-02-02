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
pub struct Manifest {
    // message fields
    pub state_hashes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub block_hashes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub state_root: ::std::vec::Vec<u8>,
    pub block_number: u64,
    pub block_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Manifest {}

impl Manifest {
    pub fn new() -> Manifest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Manifest {
        static mut instance: ::protobuf::lazy::Lazy<Manifest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Manifest,
        };
        unsafe {
            instance.get(Manifest::new)
        }
    }

    // repeated bytes state_hashes = 1;

    pub fn clear_state_hashes(&mut self) {
        self.state_hashes.clear();
    }

    // Param is passed by value, moved
    pub fn set_state_hashes(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.state_hashes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_state_hashes(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.state_hashes
    }

    // Take field
    pub fn take_state_hashes(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.state_hashes, ::protobuf::RepeatedField::new())
    }

    pub fn get_state_hashes(&self) -> &[::std::vec::Vec<u8>] {
        &self.state_hashes
    }

    fn get_state_hashes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.state_hashes
    }

    fn mut_state_hashes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.state_hashes
    }

    // repeated bytes block_hashes = 2;

    pub fn clear_block_hashes(&mut self) {
        self.block_hashes.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_hashes(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.block_hashes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_block_hashes(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.block_hashes
    }

    // Take field
    pub fn take_block_hashes(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.block_hashes, ::protobuf::RepeatedField::new())
    }

    pub fn get_block_hashes(&self) -> &[::std::vec::Vec<u8>] {
        &self.block_hashes
    }

    fn get_block_hashes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.block_hashes
    }

    fn mut_block_hashes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.block_hashes
    }

    // bytes state_root = 3;

    pub fn clear_state_root(&mut self) {
        self.state_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_state_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.state_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.state_root
    }

    // Take field
    pub fn take_state_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.state_root, ::std::vec::Vec::new())
    }

    pub fn get_state_root(&self) -> &[u8] {
        &self.state_root
    }

    fn get_state_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.state_root
    }

    fn mut_state_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.state_root
    }

    // uint64 block_number = 4;

    pub fn clear_block_number(&mut self) {
        self.block_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_block_number(&mut self, v: u64) {
        self.block_number = v;
    }

    pub fn get_block_number(&self) -> u64 {
        self.block_number
    }

    fn get_block_number_for_reflect(&self) -> &u64 {
        &self.block_number
    }

    fn mut_block_number_for_reflect(&mut self) -> &mut u64 {
        &mut self.block_number
    }

    // bytes block_hash = 5;

    pub fn clear_block_hash(&mut self) {
        self.block_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.block_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_hash
    }

    // Take field
    pub fn take_block_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.block_hash, ::std::vec::Vec::new())
    }

    pub fn get_block_hash(&self) -> &[u8] {
        &self.block_hash
    }

    fn get_block_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.block_hash
    }

    fn mut_block_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_hash
    }
}

impl ::protobuf::Message for Manifest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.state_hashes)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.block_hashes)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.state_root)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.block_number = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.block_hash)?;
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
        for value in &self.state_hashes {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.block_hashes {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if !self.state_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.state_root);
        }
        if self.block_number != 0 {
            my_size += ::protobuf::rt::value_size(4, self.block_number, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.block_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.block_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.state_hashes {
            os.write_bytes(1, &v)?;
        };
        for v in &self.block_hashes {
            os.write_bytes(2, &v)?;
        };
        if !self.state_root.is_empty() {
            os.write_bytes(3, &self.state_root)?;
        }
        if self.block_number != 0 {
            os.write_uint64(4, self.block_number)?;
        }
        if !self.block_hash.is_empty() {
            os.write_bytes(5, &self.block_hash)?;
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

impl ::protobuf::MessageStatic for Manifest {
    fn new() -> Manifest {
        Manifest::new()
    }

    fn descriptor_static(_: ::std::option::Option<Manifest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "state_hashes",
                    Manifest::get_state_hashes_for_reflect,
                    Manifest::mut_state_hashes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "block_hashes",
                    Manifest::get_block_hashes_for_reflect,
                    Manifest::mut_block_hashes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "state_root",
                    Manifest::get_state_root_for_reflect,
                    Manifest::mut_state_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "block_number",
                    Manifest::get_block_number_for_reflect,
                    Manifest::mut_block_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "block_hash",
                    Manifest::get_block_hash_for_reflect,
                    Manifest::mut_block_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Manifest>(
                    "Manifest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Manifest {
    fn clear(&mut self) {
        self.clear_state_hashes();
        self.clear_block_hashes();
        self.clear_state_root();
        self.clear_block_number();
        self.clear_block_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Manifest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Manifest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotReq {
    // message fields
    pub cmd: Cmd,
    pub start_height: u64,
    pub end_height: u64,
    pub manifest: ::protobuf::SingularPtrField<Manifest>,
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
        self.cmd = Cmd::BEGIN;
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

    // .Manifest manifest = 4;

    pub fn clear_manifest(&mut self) {
        self.manifest.clear();
    }

    pub fn has_manifest(&self) -> bool {
        self.manifest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manifest(&mut self, v: Manifest) {
        self.manifest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_manifest(&mut self) -> &mut Manifest {
        if self.manifest.is_none() {
            self.manifest.set_default();
        }
        self.manifest.as_mut().unwrap()
    }

    // Take field
    pub fn take_manifest(&mut self) -> Manifest {
        self.manifest.take().unwrap_or_else(|| Manifest::new())
    }

    pub fn get_manifest(&self) -> &Manifest {
        self.manifest.as_ref().unwrap_or_else(|| Manifest::default_instance())
    }

    fn get_manifest_for_reflect(&self) -> &::protobuf::SingularPtrField<Manifest> {
        &self.manifest
    }

    fn mut_manifest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Manifest> {
        &mut self.manifest
    }
}

impl ::protobuf::Message for SnapshotReq {
    fn is_initialized(&self) -> bool {
        for v in &self.manifest {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.manifest)?;
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
        if self.cmd != Cmd::BEGIN {
            my_size += ::protobuf::rt::enum_size(1, self.cmd);
        }
        if self.start_height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.start_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.end_height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.end_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.manifest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.cmd != Cmd::BEGIN {
            os.write_enum(1, self.cmd.value())?;
        }
        if self.start_height != 0 {
            os.write_uint64(2, self.start_height)?;
        }
        if self.end_height != 0 {
            os.write_uint64(3, self.end_height)?;
        }
        if let Some(ref v) = self.manifest.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Manifest>>(
                    "manifest",
                    SnapshotReq::get_manifest_for_reflect,
                    SnapshotReq::mut_manifest_for_reflect,
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
        self.clear_manifest();
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
    pub manifest: ::protobuf::SingularPtrField<Manifest>,
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
        self.resp = Resp::BEGIN_RESP;
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

    // .Manifest manifest = 2;

    pub fn clear_manifest(&mut self) {
        self.manifest.clear();
    }

    pub fn has_manifest(&self) -> bool {
        self.manifest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manifest(&mut self, v: Manifest) {
        self.manifest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_manifest(&mut self) -> &mut Manifest {
        if self.manifest.is_none() {
            self.manifest.set_default();
        }
        self.manifest.as_mut().unwrap()
    }

    // Take field
    pub fn take_manifest(&mut self) -> Manifest {
        self.manifest.take().unwrap_or_else(|| Manifest::new())
    }

    pub fn get_manifest(&self) -> &Manifest {
        self.manifest.as_ref().unwrap_or_else(|| Manifest::default_instance())
    }

    fn get_manifest_for_reflect(&self) -> &::protobuf::SingularPtrField<Manifest> {
        &self.manifest
    }

    fn mut_manifest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Manifest> {
        &mut self.manifest
    }
}

impl ::protobuf::Message for SnapshotResp {
    fn is_initialized(&self) -> bool {
        for v in &self.manifest {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.manifest)?;
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
        if self.resp != Resp::BEGIN_RESP {
            my_size += ::protobuf::rt::enum_size(1, self.resp);
        }
        if let Some(ref v) = self.manifest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.resp != Resp::BEGIN_RESP {
            os.write_enum(1, self.resp.value())?;
        }
        if let Some(ref v) = self.manifest.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Manifest>>(
                    "manifest",
                    SnapshotResp::get_manifest_for_reflect,
                    SnapshotResp::mut_manifest_for_reflect,
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
        self.clear_manifest();
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
    BEGIN = 0,
    CLEAR = 1,
    SNAPSHOT = 2,
    RESTORE = 3,
    END = 4,
}

impl ::protobuf::ProtobufEnum for Cmd {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Cmd> {
        match value {
            0 => ::std::option::Option::Some(Cmd::BEGIN),
            1 => ::std::option::Option::Some(Cmd::CLEAR),
            2 => ::std::option::Option::Some(Cmd::SNAPSHOT),
            3 => ::std::option::Option::Some(Cmd::RESTORE),
            4 => ::std::option::Option::Some(Cmd::END),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Cmd] = &[
            Cmd::BEGIN,
            Cmd::CLEAR,
            Cmd::SNAPSHOT,
            Cmd::RESTORE,
            Cmd::END,
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
        Cmd::BEGIN
    }
}

impl ::protobuf::reflect::ProtobufValue for Cmd {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Resp {
    BEGIN_RESP = 0,
    CLEAR_RESP = 1,
    SNAPSHOT_RESP = 2,
    RESTORE_RESP = 3,
    END_RESP = 4,
}

impl ::protobuf::ProtobufEnum for Resp {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Resp> {
        match value {
            0 => ::std::option::Option::Some(Resp::BEGIN_RESP),
            1 => ::std::option::Option::Some(Resp::CLEAR_RESP),
            2 => ::std::option::Option::Some(Resp::SNAPSHOT_RESP),
            3 => ::std::option::Option::Some(Resp::RESTORE_RESP),
            4 => ::std::option::Option::Some(Resp::END_RESP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Resp] = &[
            Resp::BEGIN_RESP,
            Resp::CLEAR_RESP,
            Resp::SNAPSHOT_RESP,
            Resp::RESTORE_RESP,
            Resp::END_RESP,
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
        Resp::BEGIN_RESP
    }
}

impl ::protobuf::reflect::ProtobufValue for Resp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0esnapshot.proto\"\xb1\x01\n\x08Manifest\x12!\n\x0cstate_hashes\x18\
    \x01\x20\x03(\x0cR\x0bstateHashes\x12!\n\x0cblock_hashes\x18\x02\x20\x03\
    (\x0cR\x0bblockHashes\x12\x1d\n\nstate_root\x18\x03\x20\x01(\x0cR\tstate\
    Root\x12!\n\x0cblock_number\x18\x04\x20\x01(\x04R\x0bblockNumber\x12\x1d\
    \n\nblock_hash\x18\x05\x20\x01(\x0cR\tblockHash\"\x8e\x01\n\x0bSnapshotR\
    eq\x12\x16\n\x03cmd\x18\x01\x20\x01(\x0e2\x04.CmdR\x03cmd\x12!\n\x0cstar\
    t_height\x18\x02\x20\x01(\x04R\x0bstartHeight\x12\x1d\n\nend_height\x18\
    \x03\x20\x01(\x04R\tendHeight\x12%\n\x08manifest\x18\x04\x20\x01(\x0b2\t\
    .ManifestR\x08manifest\"P\n\x0cSnapshotResp\x12\x19\n\x04resp\x18\x01\
    \x20\x01(\x0e2\x05.RespR\x04resp\x12%\n\x08manifest\x18\x02\x20\x01(\x0b\
    2\t.ManifestR\x08manifest*?\n\x03Cmd\x12\t\n\x05BEGIN\x10\0\x12\t\n\x05C\
    LEAR\x10\x01\x12\x0c\n\x08SNAPSHOT\x10\x02\x12\x0b\n\x07RESTORE\x10\x03\
    \x12\x07\n\x03END\x10\x04*Y\n\x04Resp\x12\x0e\n\nBEGIN_RESP\x10\0\x12\
    \x0e\n\nCLEAR_RESP\x10\x01\x12\x11\n\rSNAPSHOT_RESP\x10\x02\x12\x10\n\
    \x0cRESTORE_RESP\x10\x03\x12\x0c\n\x08END_RESP\x10\x04J\xa4\n\n\x06\x12\
    \x04\0\0%\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x05\0\x12\x04\
    \x02\0\x08\x01\n\n\n\x03\x05\0\x01\x12\x03\x02\x05\x08\n\x0b\n\x04\x05\0\
    \x02\0\x12\x03\x03\x04\x0e\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x03\x04\t\
    \n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x03\x0c\r\n\x0b\n\x04\x05\0\x02\x01\
    \x12\x03\x04\x04\x0e\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x04\x04\t\n\
    \x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x04\x0c\r\n\x0b\n\x04\x05\0\x02\x02\
    \x12\x03\x05\x04\x11\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x05\x04\x0c\n\
    \x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x05\x0f\x10\n\x0b\n\x04\x05\0\x02\
    \x03\x12\x03\x06\x04\x10\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\x06\x04\
    \x0b\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x06\x0e\x0f\n\x0b\n\x04\x05\0\
    \x02\x04\x12\x03\x07\x04\x0c\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x07\
    \x04\x07\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x07\n\x0b\n\n\n\x02\x05\
    \x01\x12\x04\n\0\x10\x01\n\n\n\x03\x05\x01\x01\x12\x03\n\x05\t\n\x0b\n\
    \x04\x05\x01\x02\0\x12\x03\x0b\x04\x13\n\x0c\n\x05\x05\x01\x02\0\x01\x12\
    \x03\x0b\x04\x0e\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03\x0b\x11\x12\n\x0b\
    \n\x04\x05\x01\x02\x01\x12\x03\x0c\x04\x13\n\x0c\n\x05\x05\x01\x02\x01\
    \x01\x12\x03\x0c\x04\x0e\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03\x0c\x11\
    \x12\n\x0b\n\x04\x05\x01\x02\x02\x12\x03\r\x04\x16\n\x0c\n\x05\x05\x01\
    \x02\x02\x01\x12\x03\r\x04\x11\n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x03\r\
    \x14\x15\n\x0b\n\x04\x05\x01\x02\x03\x12\x03\x0e\x04\x15\n\x0c\n\x05\x05\
    \x01\x02\x03\x01\x12\x03\x0e\x04\x10\n\x0c\n\x05\x05\x01\x02\x03\x02\x12\
    \x03\x0e\x13\x14\n\x0b\n\x04\x05\x01\x02\x04\x12\x03\x0f\x04\x11\n\x0c\n\
    \x05\x05\x01\x02\x04\x01\x12\x03\x0f\x04\x0c\n\x0c\n\x05\x05\x01\x02\x04\
    \x02\x12\x03\x0f\x0f\x10\n\n\n\x02\x04\0\x12\x04\x12\0\x18\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x12\x08\x10\n\x0b\n\x04\x04\0\x02\0\x12\x03\x13\x04$\
    \n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x13\x04\x0c\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x13\r\x12\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x13\x13\x1f\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x13\"#\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\x14\x04$\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x14\x04\x0c\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03\x14\r\x12\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x14\x13\x1f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x14\"#\n\x0b\
    \n\x04\x04\0\x02\x02\x12\x03\x15\x04\x19\n\r\n\x05\x04\0\x02\x02\x04\x12\
    \x04\x15\x04\x14$\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x15\x04\t\n\x0c\
    \n\x05\x04\0\x02\x02\x01\x12\x03\x15\n\x14\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x15\x17\x18\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x16\x04\x1c\n\r\n\
    \x05\x04\0\x02\x03\x04\x12\x04\x16\x04\x15\x19\n\x0c\n\x05\x04\0\x02\x03\
    \x05\x12\x03\x16\x04\n\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x16\x0b\x17\
    \n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x16\x1a\x1b\n\x0b\n\x04\x04\0\x02\
    \x04\x12\x03\x17\x04\x19\n\r\n\x05\x04\0\x02\x04\x04\x12\x04\x17\x04\x16\
    \x1c\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x17\x04\t\n\x0c\n\x05\x04\0\
    \x02\x04\x01\x12\x03\x17\n\x14\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x17\
    \x17\x18\n\n\n\x02\x04\x01\x12\x04\x1a\0\x1f\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\x1a\x08\x13\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x1b\x04\x10\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04\x1b\x04\x1a\x15\n\x0c\n\x05\x04\x01\x02\0\
    \x06\x12\x03\x1b\x04\x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1b\x08\
    \x0b\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1b\x0e\x0f\n\x0b\n\x04\x04\
    \x01\x02\x01\x12\x03\x1c\x04\x1c\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\
    \x1c\x04\x1b\x10\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x1c\x04\n\n\x0c\
    \n\x05\x04\x01\x02\x01\x01\x12\x03\x1c\x0b\x17\n\x0c\n\x05\x04\x01\x02\
    \x01\x03\x12\x03\x1c\x1a\x1b\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x1d\x04\
    \x1a\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x1d\x04\x1c\x1c\n\x0c\n\x05\
    \x04\x01\x02\x02\x05\x12\x03\x1d\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\
    \x12\x03\x1d\x0b\x15\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x1d\x18\x19\
    \n\x0b\n\x04\x04\x01\x02\x03\x12\x03\x1e\x04\x1a\n\r\n\x05\x04\x01\x02\
    \x03\x04\x12\x04\x1e\x04\x1d\x1a\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x03\
    \x1e\x04\x0c\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x1e\r\x15\n\x0c\n\
    \x05\x04\x01\x02\x03\x03\x12\x03\x1e\x18\x19\n\n\n\x02\x04\x02\x12\x04\"\
    \0%\x01\n\n\n\x03\x04\x02\x01\x12\x03\"\x08\x14\n\x0b\n\x04\x04\x02\x02\
    \0\x12\x03#\x04\x12\n\r\n\x05\x04\x02\x02\0\x04\x12\x04#\x04\"\x16\n\x0c\
    \n\x05\x04\x02\x02\0\x06\x12\x03#\x04\x08\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03#\t\r\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03#\x10\x11\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03$\x04\x1a\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04$\
    \x04#\x12\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03$\x04\x0c\n\x0c\n\x05\
    \x04\x02\x02\x01\x01\x12\x03$\r\x15\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03$\x18\x19b\x06proto3\
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
