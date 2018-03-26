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
pub struct ExecutedHeader {
    // message fields
    pub prevhash: ::std::vec::Vec<u8>,
    pub timestamp: u64,
    pub height: u64,
    pub state_root: ::std::vec::Vec<u8>,
    pub transactions_root: ::std::vec::Vec<u8>,
    pub receipts_root: ::std::vec::Vec<u8>,
    pub log_bloom: ::std::vec::Vec<u8>,
    pub gas_used: u64,
    pub gas_limit: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExecutedHeader {}

impl ExecutedHeader {
    pub fn new() -> ExecutedHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutedHeader {
        static mut instance: ::protobuf::lazy::Lazy<ExecutedHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutedHeader,
        };
        unsafe {
            instance.get(ExecutedHeader::new)
        }
    }

    // bytes prevhash = 1;

    pub fn clear_prevhash(&mut self) {
        self.prevhash.clear();
    }

    // Param is passed by value, moved
    pub fn set_prevhash(&mut self, v: ::std::vec::Vec<u8>) {
        self.prevhash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prevhash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.prevhash
    }

    // Take field
    pub fn take_prevhash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.prevhash, ::std::vec::Vec::new())
    }

    pub fn get_prevhash(&self) -> &[u8] {
        &self.prevhash
    }

    fn get_prevhash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.prevhash
    }

    fn mut_prevhash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.prevhash
    }

    // uint64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &u64 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut u64 {
        &mut self.timestamp
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

    // bytes state_root = 4;

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

    // bytes transactions_root = 5;

    pub fn clear_transactions_root(&mut self) {
        self.transactions_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.transactions_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transactions_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transactions_root
    }

    // Take field
    pub fn take_transactions_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.transactions_root, ::std::vec::Vec::new())
    }

    pub fn get_transactions_root(&self) -> &[u8] {
        &self.transactions_root
    }

    fn get_transactions_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.transactions_root
    }

    fn mut_transactions_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transactions_root
    }

    // bytes receipts_root = 6;

    pub fn clear_receipts_root(&mut self) {
        self.receipts_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_receipts_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.receipts_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_receipts_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.receipts_root
    }

    // Take field
    pub fn take_receipts_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.receipts_root, ::std::vec::Vec::new())
    }

    pub fn get_receipts_root(&self) -> &[u8] {
        &self.receipts_root
    }

    fn get_receipts_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.receipts_root
    }

    fn mut_receipts_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.receipts_root
    }

    // bytes log_bloom = 7;

    pub fn clear_log_bloom(&mut self) {
        self.log_bloom.clear();
    }

    // Param is passed by value, moved
    pub fn set_log_bloom(&mut self, v: ::std::vec::Vec<u8>) {
        self.log_bloom = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log_bloom(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.log_bloom
    }

    // Take field
    pub fn take_log_bloom(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.log_bloom, ::std::vec::Vec::new())
    }

    pub fn get_log_bloom(&self) -> &[u8] {
        &self.log_bloom
    }

    fn get_log_bloom_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.log_bloom
    }

    fn mut_log_bloom_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.log_bloom
    }

    // uint64 gas_used = 8;

    pub fn clear_gas_used(&mut self) {
        self.gas_used = 0;
    }

    // Param is passed by value, moved
    pub fn set_gas_used(&mut self, v: u64) {
        self.gas_used = v;
    }

    pub fn get_gas_used(&self) -> u64 {
        self.gas_used
    }

    fn get_gas_used_for_reflect(&self) -> &u64 {
        &self.gas_used
    }

    fn mut_gas_used_for_reflect(&mut self) -> &mut u64 {
        &mut self.gas_used
    }

    // uint64 gas_limit = 9;

    pub fn clear_gas_limit(&mut self) {
        self.gas_limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_gas_limit(&mut self, v: u64) {
        self.gas_limit = v;
    }

    pub fn get_gas_limit(&self) -> u64 {
        self.gas_limit
    }

    fn get_gas_limit_for_reflect(&self) -> &u64 {
        &self.gas_limit
    }

    fn mut_gas_limit_for_reflect(&mut self) -> &mut u64 {
        &mut self.gas_limit
    }
}

impl ::protobuf::Message for ExecutedHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.prevhash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.state_root)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.transactions_root)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.receipts_root)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.log_bloom)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.gas_used = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.gas_limit = tmp;
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
        if !self.prevhash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.prevhash);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.state_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.state_root);
        }
        if !self.transactions_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.transactions_root);
        }
        if !self.receipts_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.receipts_root);
        }
        if !self.log_bloom.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.log_bloom);
        }
        if self.gas_used != 0 {
            my_size += ::protobuf::rt::value_size(8, self.gas_used, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.gas_limit != 0 {
            my_size += ::protobuf::rt::value_size(9, self.gas_limit, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.prevhash.is_empty() {
            os.write_bytes(1, &self.prevhash)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(2, self.timestamp)?;
        }
        if self.height != 0 {
            os.write_uint64(3, self.height)?;
        }
        if !self.state_root.is_empty() {
            os.write_bytes(4, &self.state_root)?;
        }
        if !self.transactions_root.is_empty() {
            os.write_bytes(5, &self.transactions_root)?;
        }
        if !self.receipts_root.is_empty() {
            os.write_bytes(6, &self.receipts_root)?;
        }
        if !self.log_bloom.is_empty() {
            os.write_bytes(7, &self.log_bloom)?;
        }
        if self.gas_used != 0 {
            os.write_uint64(8, self.gas_used)?;
        }
        if self.gas_limit != 0 {
            os.write_uint64(9, self.gas_limit)?;
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

impl ::protobuf::MessageStatic for ExecutedHeader {
    fn new() -> ExecutedHeader {
        ExecutedHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecutedHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "prevhash",
                    ExecutedHeader::get_prevhash_for_reflect,
                    ExecutedHeader::mut_prevhash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    ExecutedHeader::get_timestamp_for_reflect,
                    ExecutedHeader::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    ExecutedHeader::get_height_for_reflect,
                    ExecutedHeader::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "state_root",
                    ExecutedHeader::get_state_root_for_reflect,
                    ExecutedHeader::mut_state_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "transactions_root",
                    ExecutedHeader::get_transactions_root_for_reflect,
                    ExecutedHeader::mut_transactions_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "receipts_root",
                    ExecutedHeader::get_receipts_root_for_reflect,
                    ExecutedHeader::mut_receipts_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "log_bloom",
                    ExecutedHeader::get_log_bloom_for_reflect,
                    ExecutedHeader::mut_log_bloom_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "gas_used",
                    ExecutedHeader::get_gas_used_for_reflect,
                    ExecutedHeader::mut_gas_used_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "gas_limit",
                    ExecutedHeader::get_gas_limit_for_reflect,
                    ExecutedHeader::mut_gas_limit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecutedHeader>(
                    "ExecutedHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecutedHeader {
    fn clear(&mut self) {
        self.clear_prevhash();
        self.clear_timestamp();
        self.clear_height();
        self.clear_state_root();
        self.clear_transactions_root();
        self.clear_receipts_root();
        self.clear_log_bloom();
        self.clear_gas_used();
        self.clear_gas_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExecutedHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExecutedHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LogEntry {
    // message fields
    pub address: ::std::vec::Vec<u8>,
    pub topics: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogEntry {}

impl LogEntry {
    pub fn new() -> LogEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogEntry {
        static mut instance: ::protobuf::lazy::Lazy<LogEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogEntry,
        };
        unsafe {
            instance.get(LogEntry::new)
        }
    }

    // bytes address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::vec::Vec<u8>) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.address, ::std::vec::Vec::new())
    }

    pub fn get_address(&self) -> &[u8] {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // repeated bytes topics = 2;

    pub fn clear_topics(&mut self) {
        self.topics.clear();
    }

    // Param is passed by value, moved
    pub fn set_topics(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.topics = v;
    }

    // Mutable pointer to the field.
    pub fn mut_topics(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.topics
    }

    // Take field
    pub fn take_topics(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.topics, ::protobuf::RepeatedField::new())
    }

    pub fn get_topics(&self) -> &[::std::vec::Vec<u8>] {
        &self.topics
    }

    fn get_topics_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.topics
    }

    fn mut_topics_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.topics
    }

    // bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for LogEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.topics)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.address);
        }
        for value in &self.topics {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.address.is_empty() {
            os.write_bytes(1, &self.address)?;
        }
        for v in &self.topics {
            os.write_bytes(2, &v)?;
        };
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
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

impl ::protobuf::MessageStatic for LogEntry {
    fn new() -> LogEntry {
        LogEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "address",
                    LogEntry::get_address_for_reflect,
                    LogEntry::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "topics",
                    LogEntry::get_topics_for_reflect,
                    LogEntry::mut_topics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    LogEntry::get_data_for_reflect,
                    LogEntry::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogEntry>(
                    "LogEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogEntry {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_topics();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LogEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReceiptErrorWithOption {
    // message fields
    pub error: ReceiptError,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReceiptErrorWithOption {}

impl ReceiptErrorWithOption {
    pub fn new() -> ReceiptErrorWithOption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReceiptErrorWithOption {
        static mut instance: ::protobuf::lazy::Lazy<ReceiptErrorWithOption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReceiptErrorWithOption,
        };
        unsafe {
            instance.get(ReceiptErrorWithOption::new)
        }
    }

    // .ReceiptError error = 1;

    pub fn clear_error(&mut self) {
        self.error = ReceiptError::NoTransactionPermission;
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ReceiptError) {
        self.error = v;
    }

    pub fn get_error(&self) -> ReceiptError {
        self.error
    }

    fn get_error_for_reflect(&self) -> &ReceiptError {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ReceiptError {
        &mut self.error
    }
}

impl ::protobuf::Message for ReceiptErrorWithOption {
    fn is_initialized(&self) -> bool {
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
                    self.error = tmp;
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
        if self.error != ReceiptError::NoTransactionPermission {
            my_size += ::protobuf::rt::enum_size(1, self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.error != ReceiptError::NoTransactionPermission {
            os.write_enum(1, self.error.value())?;
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

impl ::protobuf::MessageStatic for ReceiptErrorWithOption {
    fn new() -> ReceiptErrorWithOption {
        ReceiptErrorWithOption::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReceiptErrorWithOption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ReceiptError>>(
                    "error",
                    ReceiptErrorWithOption::get_error_for_reflect,
                    ReceiptErrorWithOption::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReceiptErrorWithOption>(
                    "ReceiptErrorWithOption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReceiptErrorWithOption {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReceiptErrorWithOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReceiptErrorWithOption {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StateRoot {
    // message fields
    pub state_root: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StateRoot {}

impl StateRoot {
    pub fn new() -> StateRoot {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StateRoot {
        static mut instance: ::protobuf::lazy::Lazy<StateRoot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StateRoot,
        };
        unsafe {
            instance.get(StateRoot::new)
        }
    }

    // bytes state_root = 1;

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
}

impl ::protobuf::Message for StateRoot {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.state_root)?;
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
        if !self.state_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.state_root);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.state_root.is_empty() {
            os.write_bytes(1, &self.state_root)?;
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

impl ::protobuf::MessageStatic for StateRoot {
    fn new() -> StateRoot {
        StateRoot::new()
    }

    fn descriptor_static(_: ::std::option::Option<StateRoot>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "state_root",
                    StateRoot::get_state_root_for_reflect,
                    StateRoot::mut_state_root_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StateRoot>(
                    "StateRoot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StateRoot {
    fn clear(&mut self) {
        self.clear_state_root();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StateRoot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StateRoot {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Receipt {
    // message fields
    pub state_root: ::protobuf::SingularPtrField<StateRoot>,
    pub gas_used: ::std::string::String,
    pub log_bloom: ::std::vec::Vec<u8>,
    pub logs: ::protobuf::RepeatedField<LogEntry>,
    pub error: ::protobuf::SingularPtrField<ReceiptErrorWithOption>,
    pub account_nonce: u64,
    pub transaction_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Receipt {}

impl Receipt {
    pub fn new() -> Receipt {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Receipt {
        static mut instance: ::protobuf::lazy::Lazy<Receipt> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Receipt,
        };
        unsafe {
            instance.get(Receipt::new)
        }
    }

    // .StateRoot state_root = 1;

    pub fn clear_state_root(&mut self) {
        self.state_root.clear();
    }

    pub fn has_state_root(&self) -> bool {
        self.state_root.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state_root(&mut self, v: StateRoot) {
        self.state_root = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state_root(&mut self) -> &mut StateRoot {
        if self.state_root.is_none() {
            self.state_root.set_default();
        }
        self.state_root.as_mut().unwrap()
    }

    // Take field
    pub fn take_state_root(&mut self) -> StateRoot {
        self.state_root.take().unwrap_or_else(|| StateRoot::new())
    }

    pub fn get_state_root(&self) -> &StateRoot {
        self.state_root.as_ref().unwrap_or_else(|| StateRoot::default_instance())
    }

    fn get_state_root_for_reflect(&self) -> &::protobuf::SingularPtrField<StateRoot> {
        &self.state_root
    }

    fn mut_state_root_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StateRoot> {
        &mut self.state_root
    }

    // string gas_used = 2;

    pub fn clear_gas_used(&mut self) {
        self.gas_used.clear();
    }

    // Param is passed by value, moved
    pub fn set_gas_used(&mut self, v: ::std::string::String) {
        self.gas_used = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gas_used(&mut self) -> &mut ::std::string::String {
        &mut self.gas_used
    }

    // Take field
    pub fn take_gas_used(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gas_used, ::std::string::String::new())
    }

    pub fn get_gas_used(&self) -> &str {
        &self.gas_used
    }

    fn get_gas_used_for_reflect(&self) -> &::std::string::String {
        &self.gas_used
    }

    fn mut_gas_used_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.gas_used
    }

    // bytes log_bloom = 3;

    pub fn clear_log_bloom(&mut self) {
        self.log_bloom.clear();
    }

    // Param is passed by value, moved
    pub fn set_log_bloom(&mut self, v: ::std::vec::Vec<u8>) {
        self.log_bloom = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log_bloom(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.log_bloom
    }

    // Take field
    pub fn take_log_bloom(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.log_bloom, ::std::vec::Vec::new())
    }

    pub fn get_log_bloom(&self) -> &[u8] {
        &self.log_bloom
    }

    fn get_log_bloom_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.log_bloom
    }

    fn mut_log_bloom_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.log_bloom
    }

    // repeated .LogEntry logs = 4;

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    // Param is passed by value, moved
    pub fn set_logs(&mut self, v: ::protobuf::RepeatedField<LogEntry>) {
        self.logs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_logs(&mut self) -> &mut ::protobuf::RepeatedField<LogEntry> {
        &mut self.logs
    }

    // Take field
    pub fn take_logs(&mut self) -> ::protobuf::RepeatedField<LogEntry> {
        ::std::mem::replace(&mut self.logs, ::protobuf::RepeatedField::new())
    }

    pub fn get_logs(&self) -> &[LogEntry] {
        &self.logs
    }

    fn get_logs_for_reflect(&self) -> &::protobuf::RepeatedField<LogEntry> {
        &self.logs
    }

    fn mut_logs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LogEntry> {
        &mut self.logs
    }

    // .ReceiptErrorWithOption error = 5;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ReceiptErrorWithOption) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ReceiptErrorWithOption {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ReceiptErrorWithOption {
        self.error.take().unwrap_or_else(|| ReceiptErrorWithOption::new())
    }

    pub fn get_error(&self) -> &ReceiptErrorWithOption {
        self.error.as_ref().unwrap_or_else(|| ReceiptErrorWithOption::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<ReceiptErrorWithOption> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ReceiptErrorWithOption> {
        &mut self.error
    }

    // uint64 account_nonce = 6;

    pub fn clear_account_nonce(&mut self) {
        self.account_nonce = 0;
    }

    // Param is passed by value, moved
    pub fn set_account_nonce(&mut self, v: u64) {
        self.account_nonce = v;
    }

    pub fn get_account_nonce(&self) -> u64 {
        self.account_nonce
    }

    fn get_account_nonce_for_reflect(&self) -> &u64 {
        &self.account_nonce
    }

    fn mut_account_nonce_for_reflect(&mut self) -> &mut u64 {
        &mut self.account_nonce
    }

    // bytes transaction_hash = 7;

    pub fn clear_transaction_hash(&mut self) {
        self.transaction_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_transaction_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.transaction_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transaction_hash
    }

    // Take field
    pub fn take_transaction_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.transaction_hash, ::std::vec::Vec::new())
    }

    pub fn get_transaction_hash(&self) -> &[u8] {
        &self.transaction_hash
    }

    fn get_transaction_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.transaction_hash
    }

    fn mut_transaction_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transaction_hash
    }
}

impl ::protobuf::Message for Receipt {
    fn is_initialized(&self) -> bool {
        for v in &self.state_root {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.logs {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.state_root)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gas_used)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.log_bloom)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.logs)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.account_nonce = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.transaction_hash)?;
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
        if let Some(ref v) = self.state_root.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.gas_used.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.gas_used);
        }
        if !self.log_bloom.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.log_bloom);
        }
        for value in &self.logs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.account_nonce != 0 {
            my_size += ::protobuf::rt::value_size(6, self.account_nonce, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.transaction_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.transaction_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.state_root.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.gas_used.is_empty() {
            os.write_string(2, &self.gas_used)?;
        }
        if !self.log_bloom.is_empty() {
            os.write_bytes(3, &self.log_bloom)?;
        }
        for v in &self.logs {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.account_nonce != 0 {
            os.write_uint64(6, self.account_nonce)?;
        }
        if !self.transaction_hash.is_empty() {
            os.write_bytes(7, &self.transaction_hash)?;
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

impl ::protobuf::MessageStatic for Receipt {
    fn new() -> Receipt {
        Receipt::new()
    }

    fn descriptor_static(_: ::std::option::Option<Receipt>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StateRoot>>(
                    "state_root",
                    Receipt::get_state_root_for_reflect,
                    Receipt::mut_state_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gas_used",
                    Receipt::get_gas_used_for_reflect,
                    Receipt::mut_gas_used_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "log_bloom",
                    Receipt::get_log_bloom_for_reflect,
                    Receipt::mut_log_bloom_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LogEntry>>(
                    "logs",
                    Receipt::get_logs_for_reflect,
                    Receipt::mut_logs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReceiptErrorWithOption>>(
                    "error",
                    Receipt::get_error_for_reflect,
                    Receipt::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "account_nonce",
                    Receipt::get_account_nonce_for_reflect,
                    Receipt::mut_account_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "transaction_hash",
                    Receipt::get_transaction_hash_for_reflect,
                    Receipt::mut_transaction_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Receipt>(
                    "Receipt",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Receipt {
    fn clear(&mut self) {
        self.clear_state_root();
        self.clear_gas_used();
        self.clear_log_bloom();
        self.clear_logs();
        self.clear_error();
        self.clear_account_nonce();
        self.clear_transaction_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Receipt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Receipt {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReceiptWithOption {
    // message fields
    pub receipt: ::protobuf::SingularPtrField<Receipt>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReceiptWithOption {}

impl ReceiptWithOption {
    pub fn new() -> ReceiptWithOption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReceiptWithOption {
        static mut instance: ::protobuf::lazy::Lazy<ReceiptWithOption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReceiptWithOption,
        };
        unsafe {
            instance.get(ReceiptWithOption::new)
        }
    }

    // .Receipt receipt = 1;

    pub fn clear_receipt(&mut self) {
        self.receipt.clear();
    }

    pub fn has_receipt(&self) -> bool {
        self.receipt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_receipt(&mut self, v: Receipt) {
        self.receipt = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_receipt(&mut self) -> &mut Receipt {
        if self.receipt.is_none() {
            self.receipt.set_default();
        }
        self.receipt.as_mut().unwrap()
    }

    // Take field
    pub fn take_receipt(&mut self) -> Receipt {
        self.receipt.take().unwrap_or_else(|| Receipt::new())
    }

    pub fn get_receipt(&self) -> &Receipt {
        self.receipt.as_ref().unwrap_or_else(|| Receipt::default_instance())
    }

    fn get_receipt_for_reflect(&self) -> &::protobuf::SingularPtrField<Receipt> {
        &self.receipt
    }

    fn mut_receipt_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Receipt> {
        &mut self.receipt
    }
}

impl ::protobuf::Message for ReceiptWithOption {
    fn is_initialized(&self) -> bool {
        for v in &self.receipt {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.receipt)?;
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
        if let Some(ref v) = self.receipt.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.receipt.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ReceiptWithOption {
    fn new() -> ReceiptWithOption {
        ReceiptWithOption::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReceiptWithOption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Receipt>>(
                    "receipt",
                    ReceiptWithOption::get_receipt_for_reflect,
                    ReceiptWithOption::mut_receipt_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReceiptWithOption>(
                    "ReceiptWithOption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReceiptWithOption {
    fn clear(&mut self) {
        self.clear_receipt();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReceiptWithOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReceiptWithOption {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExecutedInfo {
    // message fields
    pub header: ::protobuf::SingularPtrField<ExecutedHeader>,
    pub receipts: ::protobuf::RepeatedField<ReceiptWithOption>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExecutedInfo {}

impl ExecutedInfo {
    pub fn new() -> ExecutedInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutedInfo {
        static mut instance: ::protobuf::lazy::Lazy<ExecutedInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutedInfo,
        };
        unsafe {
            instance.get(ExecutedInfo::new)
        }
    }

    // .ExecutedHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ExecutedHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ExecutedHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ExecutedHeader {
        self.header.take().unwrap_or_else(|| ExecutedHeader::new())
    }

    pub fn get_header(&self) -> &ExecutedHeader {
        self.header.as_ref().unwrap_or_else(|| ExecutedHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ExecutedHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ExecutedHeader> {
        &mut self.header
    }

    // repeated .ReceiptWithOption receipts = 3;

    pub fn clear_receipts(&mut self) {
        self.receipts.clear();
    }

    // Param is passed by value, moved
    pub fn set_receipts(&mut self, v: ::protobuf::RepeatedField<ReceiptWithOption>) {
        self.receipts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_receipts(&mut self) -> &mut ::protobuf::RepeatedField<ReceiptWithOption> {
        &mut self.receipts
    }

    // Take field
    pub fn take_receipts(&mut self) -> ::protobuf::RepeatedField<ReceiptWithOption> {
        ::std::mem::replace(&mut self.receipts, ::protobuf::RepeatedField::new())
    }

    pub fn get_receipts(&self) -> &[ReceiptWithOption] {
        &self.receipts
    }

    fn get_receipts_for_reflect(&self) -> &::protobuf::RepeatedField<ReceiptWithOption> {
        &self.receipts
    }

    fn mut_receipts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ReceiptWithOption> {
        &mut self.receipts
    }
}

impl ::protobuf::Message for ExecutedInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.receipts {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.receipts)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.receipts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.receipts {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for ExecutedInfo {
    fn new() -> ExecutedInfo {
        ExecutedInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecutedInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ExecutedHeader>>(
                    "header",
                    ExecutedInfo::get_header_for_reflect,
                    ExecutedInfo::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReceiptWithOption>>(
                    "receipts",
                    ExecutedInfo::get_receipts_for_reflect,
                    ExecutedInfo::mut_receipts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecutedInfo>(
                    "ExecutedInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecutedInfo {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_receipts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExecutedInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExecutedInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConsensusConfig {
    // message fields
    pub block_gas_limit: u64,
    pub account_gas_limit: ::protobuf::SingularPtrField<super::blockchain::AccountGasLimit>,
    pub nodes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConsensusConfig {}

impl ConsensusConfig {
    pub fn new() -> ConsensusConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConsensusConfig {
        static mut instance: ::protobuf::lazy::Lazy<ConsensusConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConsensusConfig,
        };
        unsafe {
            instance.get(ConsensusConfig::new)
        }
    }

    // uint64 block_gas_limit = 1;

    pub fn clear_block_gas_limit(&mut self) {
        self.block_gas_limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_block_gas_limit(&mut self, v: u64) {
        self.block_gas_limit = v;
    }

    pub fn get_block_gas_limit(&self) -> u64 {
        self.block_gas_limit
    }

    fn get_block_gas_limit_for_reflect(&self) -> &u64 {
        &self.block_gas_limit
    }

    fn mut_block_gas_limit_for_reflect(&mut self) -> &mut u64 {
        &mut self.block_gas_limit
    }

    // .AccountGasLimit account_gas_limit = 2;

    pub fn clear_account_gas_limit(&mut self) {
        self.account_gas_limit.clear();
    }

    pub fn has_account_gas_limit(&self) -> bool {
        self.account_gas_limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_gas_limit(&mut self, v: super::blockchain::AccountGasLimit) {
        self.account_gas_limit = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_gas_limit(&mut self) -> &mut super::blockchain::AccountGasLimit {
        if self.account_gas_limit.is_none() {
            self.account_gas_limit.set_default();
        }
        self.account_gas_limit.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_gas_limit(&mut self) -> super::blockchain::AccountGasLimit {
        self.account_gas_limit.take().unwrap_or_else(|| super::blockchain::AccountGasLimit::new())
    }

    pub fn get_account_gas_limit(&self) -> &super::blockchain::AccountGasLimit {
        self.account_gas_limit.as_ref().unwrap_or_else(|| super::blockchain::AccountGasLimit::default_instance())
    }

    fn get_account_gas_limit_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain::AccountGasLimit> {
        &self.account_gas_limit
    }

    fn mut_account_gas_limit_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain::AccountGasLimit> {
        &mut self.account_gas_limit
    }

    // repeated bytes nodes = 3;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[::std::vec::Vec<u8>] {
        &self.nodes
    }

    fn get_nodes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.nodes
    }
}

impl ::protobuf::Message for ConsensusConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.account_gas_limit {
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
                    let tmp = is.read_uint64()?;
                    self.block_gas_limit = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.account_gas_limit)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.nodes)?;
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
        if self.block_gas_limit != 0 {
            my_size += ::protobuf::rt::value_size(1, self.block_gas_limit, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.account_gas_limit.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.nodes {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.block_gas_limit != 0 {
            os.write_uint64(1, self.block_gas_limit)?;
        }
        if let Some(ref v) = self.account_gas_limit.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.nodes {
            os.write_bytes(3, &v)?;
        };
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

impl ::protobuf::MessageStatic for ConsensusConfig {
    fn new() -> ConsensusConfig {
        ConsensusConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConsensusConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "block_gas_limit",
                    ConsensusConfig::get_block_gas_limit_for_reflect,
                    ConsensusConfig::mut_block_gas_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain::AccountGasLimit>>(
                    "account_gas_limit",
                    ConsensusConfig::get_account_gas_limit_for_reflect,
                    ConsensusConfig::mut_account_gas_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "nodes",
                    ConsensusConfig::get_nodes_for_reflect,
                    ConsensusConfig::mut_nodes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConsensusConfig>(
                    "ConsensusConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConsensusConfig {
    fn clear(&mut self) {
        self.clear_block_gas_limit();
        self.clear_account_gas_limit();
        self.clear_nodes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConsensusConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConsensusConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExecutedResult {
    // message fields
    pub executed_info: ::protobuf::SingularPtrField<ExecutedInfo>,
    pub config: ::protobuf::SingularPtrField<ConsensusConfig>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExecutedResult {}

impl ExecutedResult {
    pub fn new() -> ExecutedResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutedResult {
        static mut instance: ::protobuf::lazy::Lazy<ExecutedResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutedResult,
        };
        unsafe {
            instance.get(ExecutedResult::new)
        }
    }

    // .ExecutedInfo executed_info = 1;

    pub fn clear_executed_info(&mut self) {
        self.executed_info.clear();
    }

    pub fn has_executed_info(&self) -> bool {
        self.executed_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executed_info(&mut self, v: ExecutedInfo) {
        self.executed_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executed_info(&mut self) -> &mut ExecutedInfo {
        if self.executed_info.is_none() {
            self.executed_info.set_default();
        }
        self.executed_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_executed_info(&mut self) -> ExecutedInfo {
        self.executed_info.take().unwrap_or_else(|| ExecutedInfo::new())
    }

    pub fn get_executed_info(&self) -> &ExecutedInfo {
        self.executed_info.as_ref().unwrap_or_else(|| ExecutedInfo::default_instance())
    }

    fn get_executed_info_for_reflect(&self) -> &::protobuf::SingularPtrField<ExecutedInfo> {
        &self.executed_info
    }

    fn mut_executed_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ExecutedInfo> {
        &mut self.executed_info
    }

    // .ConsensusConfig config = 2;

    pub fn clear_config(&mut self) {
        self.config.clear();
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: ConsensusConfig) {
        self.config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config(&mut self) -> &mut ConsensusConfig {
        if self.config.is_none() {
            self.config.set_default();
        }
        self.config.as_mut().unwrap()
    }

    // Take field
    pub fn take_config(&mut self) -> ConsensusConfig {
        self.config.take().unwrap_or_else(|| ConsensusConfig::new())
    }

    pub fn get_config(&self) -> &ConsensusConfig {
        self.config.as_ref().unwrap_or_else(|| ConsensusConfig::default_instance())
    }

    fn get_config_for_reflect(&self) -> &::protobuf::SingularPtrField<ConsensusConfig> {
        &self.config
    }

    fn mut_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ConsensusConfig> {
        &mut self.config
    }
}

impl ::protobuf::Message for ExecutedResult {
    fn is_initialized(&self) -> bool {
        for v in &self.executed_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.config {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.executed_info)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.config)?;
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
        if let Some(ref v) = self.executed_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.executed_info.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.config.as_ref() {
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

impl ::protobuf::MessageStatic for ExecutedResult {
    fn new() -> ExecutedResult {
        ExecutedResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecutedResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ExecutedInfo>>(
                    "executed_info",
                    ExecutedResult::get_executed_info_for_reflect,
                    ExecutedResult::mut_executed_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ConsensusConfig>>(
                    "config",
                    ExecutedResult::get_config_for_reflect,
                    ExecutedResult::mut_config_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecutedResult>(
                    "ExecutedResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecutedResult {
    fn clear(&mut self) {
        self.clear_executed_info();
        self.clear_config();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExecutedResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExecutedResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegisterRequest {
    // message fields
    pub contract_address: ::std::string::String,
    pub ip: ::std::string::String,
    pub port: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegisterRequest {}

impl RegisterRequest {
    pub fn new() -> RegisterRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegisterRequest {
        static mut instance: ::protobuf::lazy::Lazy<RegisterRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterRequest,
        };
        unsafe {
            instance.get(RegisterRequest::new)
        }
    }

    // string contract_address = 1;

    pub fn clear_contract_address(&mut self) {
        self.contract_address.clear();
    }

    // Param is passed by value, moved
    pub fn set_contract_address(&mut self, v: ::std::string::String) {
        self.contract_address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contract_address(&mut self) -> &mut ::std::string::String {
        &mut self.contract_address
    }

    // Take field
    pub fn take_contract_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.contract_address, ::std::string::String::new())
    }

    pub fn get_contract_address(&self) -> &str {
        &self.contract_address
    }

    fn get_contract_address_for_reflect(&self) -> &::std::string::String {
        &self.contract_address
    }

    fn mut_contract_address_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.contract_address
    }

    // string ip = 2;

    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::string::String {
        &mut self.ip
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ip, ::std::string::String::new())
    }

    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    fn get_ip_for_reflect(&self) -> &::std::string::String {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.ip
    }

    // string port = 3;

    pub fn clear_port(&mut self) {
        self.port.clear();
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: ::std::string::String) {
        self.port = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_port(&mut self) -> &mut ::std::string::String {
        &mut self.port
    }

    // Take field
    pub fn take_port(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.port, ::std::string::String::new())
    }

    pub fn get_port(&self) -> &str {
        &self.port
    }

    fn get_port_for_reflect(&self) -> &::std::string::String {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.port
    }
}

impl ::protobuf::Message for RegisterRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.contract_address)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ip)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.port)?;
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
        if !self.contract_address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.contract_address);
        }
        if !self.ip.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.ip);
        }
        if !self.port.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.port);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.contract_address.is_empty() {
            os.write_string(1, &self.contract_address)?;
        }
        if !self.ip.is_empty() {
            os.write_string(2, &self.ip)?;
        }
        if !self.port.is_empty() {
            os.write_string(3, &self.port)?;
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

impl ::protobuf::MessageStatic for RegisterRequest {
    fn new() -> RegisterRequest {
        RegisterRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegisterRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "contract_address",
                    RegisterRequest::get_contract_address_for_reflect,
                    RegisterRequest::mut_contract_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ip",
                    RegisterRequest::get_ip_for_reflect,
                    RegisterRequest::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "port",
                    RegisterRequest::get_port_for_reflect,
                    RegisterRequest::mut_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegisterRequest>(
                    "RegisterRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegisterRequest {
    fn clear(&mut self) {
        self.clear_contract_address();
        self.clear_ip();
        self.clear_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegisterRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegisterRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegisterResponse {
    // message fields
    pub state: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegisterResponse {}

impl RegisterResponse {
    pub fn new() -> RegisterResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegisterResponse {
        static mut instance: ::protobuf::lazy::Lazy<RegisterResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterResponse,
        };
        unsafe {
            instance.get(RegisterResponse::new)
        }
    }

    // string state = 1;

    pub fn clear_state(&mut self) {
        self.state.clear();
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: ::std::string::String) {
        self.state = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state(&mut self) -> &mut ::std::string::String {
        &mut self.state
    }

    // Take field
    pub fn take_state(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.state, ::std::string::String::new())
    }

    pub fn get_state(&self) -> &str {
        &self.state
    }

    fn get_state_for_reflect(&self) -> &::std::string::String {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.state
    }
}

impl ::protobuf::Message for RegisterResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.state)?;
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
        if !self.state.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.state);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.state.is_empty() {
            os.write_string(1, &self.state)?;
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

impl ::protobuf::MessageStatic for RegisterResponse {
    fn new() -> RegisterResponse {
        RegisterResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegisterResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "state",
                    RegisterResponse::get_state_for_reflect,
                    RegisterResponse::mut_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegisterResponse>(
                    "RegisterResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegisterResponse {
    fn clear(&mut self) {
        self.clear_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegisterResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegisterResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoadRequest {
    // message fields
    pub height: u64,
    pub contract_address: ::std::string::String,
    pub key: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoadRequest {}

impl LoadRequest {
    pub fn new() -> LoadRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoadRequest {
        static mut instance: ::protobuf::lazy::Lazy<LoadRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoadRequest,
        };
        unsafe {
            instance.get(LoadRequest::new)
        }
    }

    // uint64 height = 1;

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

    // string contract_address = 2;

    pub fn clear_contract_address(&mut self) {
        self.contract_address.clear();
    }

    // Param is passed by value, moved
    pub fn set_contract_address(&mut self, v: ::std::string::String) {
        self.contract_address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contract_address(&mut self) -> &mut ::std::string::String {
        &mut self.contract_address
    }

    // Take field
    pub fn take_contract_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.contract_address, ::std::string::String::new())
    }

    pub fn get_contract_address(&self) -> &str {
        &self.contract_address
    }

    fn get_contract_address_for_reflect(&self) -> &::std::string::String {
        &self.contract_address
    }

    fn mut_contract_address_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.contract_address
    }

    // string key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::string::String {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }
}

impl ::protobuf::Message for LoadRequest {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.contract_address)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
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
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(1, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.contract_address.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.contract_address);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_uint64(1, self.height)?;
        }
        if !self.contract_address.is_empty() {
            os.write_string(2, &self.contract_address)?;
        }
        if !self.key.is_empty() {
            os.write_string(3, &self.key)?;
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

impl ::protobuf::MessageStatic for LoadRequest {
    fn new() -> LoadRequest {
        LoadRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoadRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    LoadRequest::get_height_for_reflect,
                    LoadRequest::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "contract_address",
                    LoadRequest::get_contract_address_for_reflect,
                    LoadRequest::mut_contract_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    LoadRequest::get_key_for_reflect,
                    LoadRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoadRequest>(
                    "LoadRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoadRequest {
    fn clear(&mut self) {
        self.clear_height();
        self.clear_contract_address();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoadRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoadRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoadResponse {
    // message fields
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoadResponse {}

impl LoadResponse {
    pub fn new() -> LoadResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoadResponse {
        static mut instance: ::protobuf::lazy::Lazy<LoadResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoadResponse,
        };
        unsafe {
            instance.get(LoadResponse::new)
        }
    }

    // string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::string::String {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}

impl ::protobuf::Message for LoadResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
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
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.value.is_empty() {
            os.write_string(1, &self.value)?;
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

impl ::protobuf::MessageStatic for LoadResponse {
    fn new() -> LoadResponse {
        LoadResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoadResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    LoadResponse::get_value_for_reflect,
                    LoadResponse::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoadResponse>(
                    "LoadResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoadResponse {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoadResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoadResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReceiptError {
    NoTransactionPermission = 0,
    NoContractPermission = 1,
    NoCallPermission = 2,
    NotEnoughBaseGas = 3,
    BlockGasLimitReached = 4,
    AccountGasLimitReached = 5,
    OutOfGas = 6,
    BadJumpDestination = 7,
    BadInstruction = 8,
    StackUnderflow = 9,
    OutOfStack = 10,
    Internal = 11,
    MutableCallInStaticContext = 12,
    OutOfBounds = 13,
    Reverted = 14,
}

impl ::protobuf::ProtobufEnum for ReceiptError {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReceiptError> {
        match value {
            0 => ::std::option::Option::Some(ReceiptError::NoTransactionPermission),
            1 => ::std::option::Option::Some(ReceiptError::NoContractPermission),
            2 => ::std::option::Option::Some(ReceiptError::NoCallPermission),
            3 => ::std::option::Option::Some(ReceiptError::NotEnoughBaseGas),
            4 => ::std::option::Option::Some(ReceiptError::BlockGasLimitReached),
            5 => ::std::option::Option::Some(ReceiptError::AccountGasLimitReached),
            6 => ::std::option::Option::Some(ReceiptError::OutOfGas),
            7 => ::std::option::Option::Some(ReceiptError::BadJumpDestination),
            8 => ::std::option::Option::Some(ReceiptError::BadInstruction),
            9 => ::std::option::Option::Some(ReceiptError::StackUnderflow),
            10 => ::std::option::Option::Some(ReceiptError::OutOfStack),
            11 => ::std::option::Option::Some(ReceiptError::Internal),
            12 => ::std::option::Option::Some(ReceiptError::MutableCallInStaticContext),
            13 => ::std::option::Option::Some(ReceiptError::OutOfBounds),
            14 => ::std::option::Option::Some(ReceiptError::Reverted),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReceiptError] = &[
            ReceiptError::NoTransactionPermission,
            ReceiptError::NoContractPermission,
            ReceiptError::NoCallPermission,
            ReceiptError::NotEnoughBaseGas,
            ReceiptError::BlockGasLimitReached,
            ReceiptError::AccountGasLimitReached,
            ReceiptError::OutOfGas,
            ReceiptError::BadJumpDestination,
            ReceiptError::BadInstruction,
            ReceiptError::StackUnderflow,
            ReceiptError::OutOfStack,
            ReceiptError::Internal,
            ReceiptError::MutableCallInStaticContext,
            ReceiptError::OutOfBounds,
            ReceiptError::Reverted,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ReceiptError>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReceiptError", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReceiptError {
}

impl ::std::default::Default for ReceiptError {
    fn default() -> Self {
        ReceiptError::NoTransactionPermission
    }
}

impl ::protobuf::reflect::ProtobufValue for ReceiptError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eexecutor.proto\x1a\x10blockchain.proto\"\xa8\x02\n\x0eExecutedHead\
    er\x12\x1a\n\x08prevhash\x18\x01\x20\x01(\x0cR\x08prevhash\x12\x1c\n\tti\
    mestamp\x18\x02\x20\x01(\x04R\ttimestamp\x12\x16\n\x06height\x18\x03\x20\
    \x01(\x04R\x06height\x12\x1d\n\nstate_root\x18\x04\x20\x01(\x0cR\tstateR\
    oot\x12+\n\x11transactions_root\x18\x05\x20\x01(\x0cR\x10transactionsRoo\
    t\x12#\n\rreceipts_root\x18\x06\x20\x01(\x0cR\x0creceiptsRoot\x12\x1b\n\
    \tlog_bloom\x18\x07\x20\x01(\x0cR\x08logBloom\x12\x19\n\x08gas_used\x18\
    \x08\x20\x01(\x04R\x07gasUsed\x12\x1b\n\tgas_limit\x18\t\x20\x01(\x04R\
    \x08gasLimit\"P\n\x08LogEntry\x12\x18\n\x07address\x18\x01\x20\x01(\x0cR\
    \x07address\x12\x16\n\x06topics\x18\x02\x20\x03(\x0cR\x06topics\x12\x12\
    \n\x04data\x18\x03\x20\x01(\x0cR\x04data\"=\n\x16ReceiptErrorWithOption\
    \x12#\n\x05error\x18\x01\x20\x01(\x0e2\r.ReceiptErrorR\x05error\"*\n\tSt\
    ateRoot\x12\x1d\n\nstate_root\x18\x01\x20\x01(\x0cR\tstateRoot\"\x8a\x02\
    \n\x07Receipt\x12)\n\nstate_root\x18\x01\x20\x01(\x0b2\n.StateRootR\tsta\
    teRoot\x12\x19\n\x08gas_used\x18\x02\x20\x01(\tR\x07gasUsed\x12\x1b\n\tl\
    og_bloom\x18\x03\x20\x01(\x0cR\x08logBloom\x12\x1d\n\x04logs\x18\x04\x20\
    \x03(\x0b2\t.LogEntryR\x04logs\x12-\n\x05error\x18\x05\x20\x01(\x0b2\x17\
    .ReceiptErrorWithOptionR\x05error\x12#\n\raccount_nonce\x18\x06\x20\x01(\
    \x04R\x0caccountNonce\x12)\n\x10transaction_hash\x18\x07\x20\x01(\x0cR\
    \x0ftransactionHash\"7\n\x11ReceiptWithOption\x12\"\n\x07receipt\x18\x01\
    \x20\x01(\x0b2\x08.ReceiptR\x07receipt\"g\n\x0cExecutedInfo\x12'\n\x06he\
    ader\x18\x01\x20\x01(\x0b2\x0f.ExecutedHeaderR\x06header\x12.\n\x08recei\
    pts\x18\x03\x20\x03(\x0b2\x12.ReceiptWithOptionR\x08receipts\"\x8d\x01\n\
    \x0fConsensusConfig\x12&\n\x0fblock_gas_limit\x18\x01\x20\x01(\x04R\rblo\
    ckGasLimit\x12<\n\x11account_gas_limit\x18\x02\x20\x01(\x0b2\x10.Account\
    GasLimitR\x0faccountGasLimit\x12\x14\n\x05nodes\x18\x03\x20\x03(\x0cR\
    \x05nodes\"n\n\x0eExecutedResult\x122\n\rexecuted_info\x18\x01\x20\x01(\
    \x0b2\r.ExecutedInfoR\x0cexecutedInfo\x12(\n\x06config\x18\x02\x20\x01(\
    \x0b2\x10.ConsensusConfigR\x06config\"`\n\x0fRegisterRequest\x12)\n\x10c\
    ontract_address\x18\x01\x20\x01(\tR\x0fcontractAddress\x12\x0e\n\x02ip\
    \x18\x02\x20\x01(\tR\x02ip\x12\x12\n\x04port\x18\x03\x20\x01(\tR\x04port\
    \"(\n\x10RegisterResponse\x12\x14\n\x05state\x18\x01\x20\x01(\tR\x05stat\
    e\"b\n\x0bLoadRequest\x12\x16\n\x06height\x18\x01\x20\x01(\x04R\x06heigh\
    t\x12)\n\x10contract_address\x18\x02\x20\x01(\tR\x0fcontractAddress\x12\
    \x10\n\x03key\x18\x03\x20\x01(\tR\x03key\"$\n\x0cLoadResponse\x12\x14\n\
    \x05value\x18\x01\x20\x01(\tR\x05value*\xd2\x02\n\x0cReceiptError\x12\
    \x1b\n\x17NoTransactionPermission\x10\0\x12\x18\n\x14NoContractPermissio\
    n\x10\x01\x12\x14\n\x10NoCallPermission\x10\x02\x12\x14\n\x10NotEnoughBa\
    seGas\x10\x03\x12\x18\n\x14BlockGasLimitReached\x10\x04\x12\x1a\n\x16Acc\
    ountGasLimitReached\x10\x05\x12\x0c\n\x08OutOfGas\x10\x06\x12\x16\n\x12B\
    adJumpDestination\x10\x07\x12\x12\n\x0eBadInstruction\x10\x08\x12\x12\n\
    \x0eStackUnderflow\x10\t\x12\x0e\n\nOutOfStack\x10\n\x12\x0c\n\x08Intern\
    al\x10\x0b\x12\x1e\n\x1aMutableCallInStaticContext\x10\x0c\x12\x0f\n\x0b\
    OutOfBounds\x10\r\x12\x0c\n\x08Reverted\x10\x0e2k\n\x0fExecutorService\
    \x121\n\x08Register\x12\x10.RegisterRequest\x1a\x11.RegisterResponse\"\0\
    \x12%\n\x04Load\x12\x0c.LoadRequest\x1a\r.LoadResponse\"\0J\x85\x1d\n\
    \x06\x12\x04\0\0f\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\
    \x12\x03\x02\x07\x19\n\n\n\x02\x04\0\x12\x04\x04\0\x0e\x01\n\n\n\x03\x04\
    \0\x01\x12\x03\x04\x08\x16\n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\x04\x17\n\
    \r\n\x05\x04\0\x02\0\x04\x12\x04\x05\x04\x04\x18\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x05\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\n\x12\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x05\x15\x16\n\x0b\n\x04\x04\0\x02\x01\
    \x12\x03\x06\x04\x19\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x06\x04\x05\x17\
    \n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x06\x04\n\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x06\x0b\x14\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x06\
    \x17\x18\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x07\x04\x16\n\r\n\x05\x04\0\
    \x02\x02\x04\x12\x04\x07\x04\x06\x19\n\x0c\n\x05\x04\0\x02\x02\x05\x12\
    \x03\x07\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x07\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03\x07\x14\x15\n\x0b\n\x04\x04\0\x02\x03\x12\
    \x03\x08\x04\x19\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x08\x04\x07\x16\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x08\x04\t\n\x0c\n\x05\x04\0\x02\x03\
    \x01\x12\x03\x08\n\x14\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x08\x17\x18\
    \n\x0b\n\x04\x04\0\x02\x04\x12\x03\t\x04\x20\n\r\n\x05\x04\0\x02\x04\x04\
    \x12\x04\t\x04\x08\x19\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\t\x04\t\n\
    \x0c\n\x05\x04\0\x02\x04\x01\x12\x03\t\n\x1b\n\x0c\n\x05\x04\0\x02\x04\
    \x03\x12\x03\t\x1e\x1f\n\x0b\n\x04\x04\0\x02\x05\x12\x03\n\x04\x1c\n\r\n\
    \x05\x04\0\x02\x05\x04\x12\x04\n\x04\t\x20\n\x0c\n\x05\x04\0\x02\x05\x05\
    \x12\x03\n\x04\t\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\n\n\x17\n\x0c\n\
    \x05\x04\0\x02\x05\x03\x12\x03\n\x1a\x1b\n\x0b\n\x04\x04\0\x02\x06\x12\
    \x03\x0b\x04\x18\n\r\n\x05\x04\0\x02\x06\x04\x12\x04\x0b\x04\n\x1c\n\x0c\
    \n\x05\x04\0\x02\x06\x05\x12\x03\x0b\x04\t\n\x0c\n\x05\x04\0\x02\x06\x01\
    \x12\x03\x0b\n\x13\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x0b\x16\x17\n\
    \x0b\n\x04\x04\0\x02\x07\x12\x03\x0c\x04\x18\n\r\n\x05\x04\0\x02\x07\x04\
    \x12\x04\x0c\x04\x0b\x18\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x03\x0c\x04\n\
    \n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\x0c\x0b\x13\n\x0c\n\x05\x04\0\x02\
    \x07\x03\x12\x03\x0c\x16\x17\n\x0b\n\x04\x04\0\x02\x08\x12\x03\r\x04\x19\
    \n\r\n\x05\x04\0\x02\x08\x04\x12\x04\r\x04\x0c\x18\n\x0c\n\x05\x04\0\x02\
    \x08\x05\x12\x03\r\x04\n\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\r\x0b\x14\
    \n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03\r\x17\x18\n\n\n\x02\x05\0\x12\x04\
    \x10\0!\x01\n\n\n\x03\x05\0\x01\x12\x03\x10\x05\x11\n\x1c\n\x04\x05\0\
    \x02\0\x12\x03\x12\x04\x20\x1a\x0fExecutionError\n\n\x0c\n\x05\x05\0\x02\
    \0\x01\x12\x03\x12\x04\x1b\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x12\x1e\
    \x1f\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x13\x04\x1d\n\x0c\n\x05\x05\0\x02\
    \x01\x01\x12\x03\x13\x04\x18\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x13\
    \x1b\x1c\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x14\x04\x19\n\x0c\n\x05\x05\0\
    \x02\x02\x01\x12\x03\x14\x04\x14\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\
    \x14\x17\x18\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x15\x04\x19\n\x0c\n\x05\
    \x05\0\x02\x03\x01\x12\x03\x15\x04\x14\n\x0c\n\x05\x05\0\x02\x03\x02\x12\
    \x03\x15\x17\x18\n\x0b\n\x04\x05\0\x02\x04\x12\x03\x16\x04\x1e\n\x0c\n\
    \x05\x05\0\x02\x04\x01\x12\x03\x16\x04\x18\n\x0c\n\x05\x05\0\x02\x04\x02\
    \x12\x03\x16\x1c\x1d\n\x0b\n\x04\x05\0\x02\x05\x12\x03\x17\x04\x20\n\x0c\
    \n\x05\x05\0\x02\x05\x01\x12\x03\x17\x04\x1a\n\x0c\n\x05\x05\0\x02\x05\
    \x02\x12\x03\x17\x1e\x1f\n\x0b\n\x04\x05\0\x02\x06\x12\x03\x18\x04\x11\n\
    \x0c\n\x05\x05\0\x02\x06\x01\x12\x03\x18\x04\x0c\n\x0c\n\x05\x05\0\x02\
    \x06\x02\x12\x03\x18\x0f\x10\n\x0b\n\x04\x05\0\x02\x07\x12\x03\x19\x04\
    \x1b\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03\x19\x04\x16\n\x0c\n\x05\x05\0\
    \x02\x07\x02\x12\x03\x19\x19\x1a\n\x0b\n\x04\x05\0\x02\x08\x12\x03\x1a\
    \x04\x17\n\x0c\n\x05\x05\0\x02\x08\x01\x12\x03\x1a\x04\x12\n\x0c\n\x05\
    \x05\0\x02\x08\x02\x12\x03\x1a\x15\x16\n\x0b\n\x04\x05\0\x02\t\x12\x03\
    \x1b\x04\x17\n\x0c\n\x05\x05\0\x02\t\x01\x12\x03\x1b\x04\x12\n\x0c\n\x05\
    \x05\0\x02\t\x02\x12\x03\x1b\x15\x16\n\x0b\n\x04\x05\0\x02\n\x12\x03\x1c\
    \x04\x14\n\x0c\n\x05\x05\0\x02\n\x01\x12\x03\x1c\x04\x0e\n\x0c\n\x05\x05\
    \0\x02\n\x02\x12\x03\x1c\x11\x13\n\x0b\n\x04\x05\0\x02\x0b\x12\x03\x1d\
    \x04\x12\n\x0c\n\x05\x05\0\x02\x0b\x01\x12\x03\x1d\x04\x0c\n\x0c\n\x05\
    \x05\0\x02\x0b\x02\x12\x03\x1d\x0f\x11\n\x0b\n\x04\x05\0\x02\x0c\x12\x03\
    \x1e\x04$\n\x0c\n\x05\x05\0\x02\x0c\x01\x12\x03\x1e\x04\x1e\n\x0c\n\x05\
    \x05\0\x02\x0c\x02\x12\x03\x1e!#\n\x0b\n\x04\x05\0\x02\r\x12\x03\x1f\x04\
    \x15\n\x0c\n\x05\x05\0\x02\r\x01\x12\x03\x1f\x04\x0f\n\x0c\n\x05\x05\0\
    \x02\r\x02\x12\x03\x1f\x12\x14\n\x0b\n\x04\x05\0\x02\x0e\x12\x03\x20\x04\
    \x12\n\x0c\n\x05\x05\0\x02\x0e\x01\x12\x03\x20\x04\x0c\n\x0c\n\x05\x05\0\
    \x02\x0e\x02\x12\x03\x20\x0f\x11\n\n\n\x02\x04\x01\x12\x04#\0'\x01\n\n\n\
    \x03\x04\x01\x01\x12\x03#\x08\x10\n\x0b\n\x04\x04\x01\x02\0\x12\x03$\x04\
    \x16\n\r\n\x05\x04\x01\x02\0\x04\x12\x04$\x04#\x12\n\x0c\n\x05\x04\x01\
    \x02\0\x05\x12\x03$\x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03$\n\x11\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03$\x14\x15\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x03%\x04\x1e\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03%\x04\x0c\n\
    \x0c\n\x05\x04\x01\x02\x01\x05\x12\x03%\r\x12\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x03%\x13\x19\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03%\x1c\
    \x1d\n\x0b\n\x04\x04\x01\x02\x02\x12\x03&\x04\x13\n\r\n\x05\x04\x01\x02\
    \x02\x04\x12\x04&\x04%\x1e\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03&\x04\
    \t\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03&\n\x0e\n\x0c\n\x05\x04\x01\
    \x02\x02\x03\x12\x03&\x11\x12\n\n\n\x02\x04\x02\x12\x04)\0+\x01\n\n\n\
    \x03\x04\x02\x01\x12\x03)\x08\x1e\n\x0b\n\x04\x04\x02\x02\0\x12\x03*\x04\
    \x1b\n\r\n\x05\x04\x02\x02\0\x04\x12\x04*\x04)\x20\n\x0c\n\x05\x04\x02\
    \x02\0\x06\x12\x03*\x04\x10\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03*\x11\
    \x16\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03*\x19\x1a\n\n\n\x02\x04\x03\
    \x12\x04-\0/\x01\n\n\n\x03\x04\x03\x01\x12\x03-\x08\x11\n\x0b\n\x04\x04\
    \x03\x02\0\x12\x03.\x04\x19\n\r\n\x05\x04\x03\x02\0\x04\x12\x04.\x04-\
    \x13\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03.\x04\t\n\x0c\n\x05\x04\x03\
    \x02\0\x01\x12\x03.\n\x14\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03.\x17\x18\
    \n\n\n\x02\x04\x04\x12\x041\09\x01\n\n\n\x03\x04\x04\x01\x12\x031\x08\
    \x0f\n\x0b\n\x04\x04\x04\x02\0\x12\x032\x04\x1d\n\r\n\x05\x04\x04\x02\0\
    \x04\x12\x042\x041\x11\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x032\x04\r\n\
    \x0c\n\x05\x04\x04\x02\0\x01\x12\x032\x0e\x18\n\x0c\n\x05\x04\x04\x02\0\
    \x03\x12\x032\x1b\x1c\n\x0b\n\x04\x04\x04\x02\x01\x12\x033\x04\x18\n\r\n\
    \x05\x04\x04\x02\x01\x04\x12\x043\x042\x1d\n\x0c\n\x05\x04\x04\x02\x01\
    \x05\x12\x033\x04\n\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x033\x0b\x13\n\
    \x0c\n\x05\x04\x04\x02\x01\x03\x12\x033\x16\x17\n\x0b\n\x04\x04\x04\x02\
    \x02\x12\x034\x04\x18\n\r\n\x05\x04\x04\x02\x02\x04\x12\x044\x043\x18\n\
    \x0c\n\x05\x04\x04\x02\x02\x05\x12\x034\x04\t\n\x0c\n\x05\x04\x04\x02\
    \x02\x01\x12\x034\n\x13\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x034\x16\x17\
    \n\x0b\n\x04\x04\x04\x02\x03\x12\x035\x04\x1f\n\x0c\n\x05\x04\x04\x02\
    \x03\x04\x12\x035\x04\x0c\n\x0c\n\x05\x04\x04\x02\x03\x06\x12\x035\r\x15\
    \n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x035\x16\x1a\n\x0c\n\x05\x04\x04\
    \x02\x03\x03\x12\x035\x1d\x1e\n\x0b\n\x04\x04\x04\x02\x04\x12\x036\x04%\
    \n\r\n\x05\x04\x04\x02\x04\x04\x12\x046\x045\x1f\n\x0c\n\x05\x04\x04\x02\
    \x04\x06\x12\x036\x04\x1a\n\x0c\n\x05\x04\x04\x02\x04\x01\x12\x036\x1b\
    \x20\n\x0c\n\x05\x04\x04\x02\x04\x03\x12\x036#$\n\x0b\n\x04\x04\x04\x02\
    \x05\x12\x037\x04\x1d\n\r\n\x05\x04\x04\x02\x05\x04\x12\x047\x046%\n\x0c\
    \n\x05\x04\x04\x02\x05\x05\x12\x037\x04\n\n\x0c\n\x05\x04\x04\x02\x05\
    \x01\x12\x037\x0b\x18\n\x0c\n\x05\x04\x04\x02\x05\x03\x12\x037\x1b\x1c\n\
    \x0b\n\x04\x04\x04\x02\x06\x12\x038\x04\x1f\n\r\n\x05\x04\x04\x02\x06\
    \x04\x12\x048\x047\x1d\n\x0c\n\x05\x04\x04\x02\x06\x05\x12\x038\x04\t\n\
    \x0c\n\x05\x04\x04\x02\x06\x01\x12\x038\n\x1a\n\x0c\n\x05\x04\x04\x02\
    \x06\x03\x12\x038\x1d\x1e\n\n\n\x02\x04\x05\x12\x04;\0=\x01\n\n\n\x03\
    \x04\x05\x01\x12\x03;\x08\x19\n\x0b\n\x04\x04\x05\x02\0\x12\x03<\x04\x18\
    \n\r\n\x05\x04\x05\x02\0\x04\x12\x04<\x04;\x1b\n\x0c\n\x05\x04\x05\x02\0\
    \x06\x12\x03<\x04\x0b\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03<\x0c\x13\n\
    \x0c\n\x05\x04\x05\x02\0\x03\x12\x03<\x16\x17\n\n\n\x02\x04\x06\x12\x04?\
    \0B\x01\n\n\n\x03\x04\x06\x01\x12\x03?\x08\x14\n\x0b\n\x04\x04\x06\x02\0\
    \x12\x03@\x04\x1e\n\r\n\x05\x04\x06\x02\0\x04\x12\x04@\x04?\x16\n\x0c\n\
    \x05\x04\x06\x02\0\x06\x12\x03@\x04\x12\n\x0c\n\x05\x04\x06\x02\0\x01\
    \x12\x03@\x13\x19\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03@\x1c\x1d\n\x0b\n\
    \x04\x04\x06\x02\x01\x12\x03A\x04,\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\
    \x03A\x04\x0c\n\x0c\n\x05\x04\x06\x02\x01\x06\x12\x03A\r\x1e\n\x0c\n\x05\
    \x04\x06\x02\x01\x01\x12\x03A\x1f'\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\
    \x03A*+\n\n\n\x02\x04\x07\x12\x04D\0H\x01\n\n\n\x03\x04\x07\x01\x12\x03D\
    \x08\x17\n\x0b\n\x04\x04\x07\x02\0\x12\x03E\x04\x1f\n\r\n\x05\x04\x07\
    \x02\0\x04\x12\x04E\x04D\x19\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03E\x04\
    \n\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03E\x0b\x1a\n\x0c\n\x05\x04\x07\
    \x02\0\x03\x12\x03E\x1d\x1e\n\x0b\n\x04\x04\x07\x02\x01\x12\x03F\x04*\n\
    \r\n\x05\x04\x07\x02\x01\x04\x12\x04F\x04E\x1f\n\x0c\n\x05\x04\x07\x02\
    \x01\x06\x12\x03F\x04\x13\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x03F\x14%\
    \n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03F()\n\x0b\n\x04\x04\x07\x02\x02\
    \x12\x03G\x04\x1d\n\x0c\n\x05\x04\x07\x02\x02\x04\x12\x03G\x04\x0c\n\x0c\
    \n\x05\x04\x07\x02\x02\x05\x12\x03G\r\x12\n\x0c\n\x05\x04\x07\x02\x02\
    \x01\x12\x03G\x13\x18\n\x0c\n\x05\x04\x07\x02\x02\x03\x12\x03G\x1b\x1c\n\
    \n\n\x02\x04\x08\x12\x04J\0M\x01\n\n\n\x03\x04\x08\x01\x12\x03J\x08\x16\
    \n\x0b\n\x04\x04\x08\x02\0\x12\x03K\x04#\n\r\n\x05\x04\x08\x02\0\x04\x12\
    \x04K\x04J\x18\n\x0c\n\x05\x04\x08\x02\0\x06\x12\x03K\x04\x10\n\x0c\n\
    \x05\x04\x08\x02\0\x01\x12\x03K\x11\x1e\n\x0c\n\x05\x04\x08\x02\0\x03\
    \x12\x03K!\"\n\x0b\n\x04\x04\x08\x02\x01\x12\x03L\x04\x1f\n\r\n\x05\x04\
    \x08\x02\x01\x04\x12\x04L\x04K#\n\x0c\n\x05\x04\x08\x02\x01\x06\x12\x03L\
    \x04\x13\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03L\x14\x1a\n\x0c\n\x05\
    \x04\x08\x02\x01\x03\x12\x03L\x1d\x1e\n\n\n\x02\x04\t\x12\x04O\0S\x01\n\
    \n\n\x03\x04\t\x01\x12\x03O\x08\x17\n\x0b\n\x04\x04\t\x02\0\x12\x03P\x04\
    \x20\n\r\n\x05\x04\t\x02\0\x04\x12\x04P\x04O\x19\n\x0c\n\x05\x04\t\x02\0\
    \x05\x12\x03P\x04\n\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03P\x0b\x1b\n\x0c\n\
    \x05\x04\t\x02\0\x03\x12\x03P\x1e\x1f\n\x0b\n\x04\x04\t\x02\x01\x12\x03Q\
    \x04\x12\n\r\n\x05\x04\t\x02\x01\x04\x12\x04Q\x04P\x20\n\x0c\n\x05\x04\t\
    \x02\x01\x05\x12\x03Q\x04\n\n\x0c\n\x05\x04\t\x02\x01\x01\x12\x03Q\x0b\r\
    \n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03Q\x10\x11\n\x0b\n\x04\x04\t\x02\
    \x02\x12\x03R\x04\x14\n\r\n\x05\x04\t\x02\x02\x04\x12\x04R\x04Q\x12\n\
    \x0c\n\x05\x04\t\x02\x02\x05\x12\x03R\x04\n\n\x0c\n\x05\x04\t\x02\x02\
    \x01\x12\x03R\x0b\x0f\n\x0c\n\x05\x04\t\x02\x02\x03\x12\x03R\x12\x13\n\n\
    \n\x02\x04\n\x12\x04U\0W\x01\n\n\n\x03\x04\n\x01\x12\x03U\x08\x18\n\x0b\
    \n\x04\x04\n\x02\0\x12\x03V\x04\x15\n\r\n\x05\x04\n\x02\0\x04\x12\x04V\
    \x04U\x1a\n\x0c\n\x05\x04\n\x02\0\x05\x12\x03V\x04\n\n\x0c\n\x05\x04\n\
    \x02\0\x01\x12\x03V\x0b\x10\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03V\x13\x14\
    \n\n\n\x02\x04\x0b\x12\x04Y\0]\x01\n\n\n\x03\x04\x0b\x01\x12\x03Y\x08\
    \x13\n\x0b\n\x04\x04\x0b\x02\0\x12\x03Z\x04\x16\n\r\n\x05\x04\x0b\x02\0\
    \x04\x12\x04Z\x04Y\x15\n\x0c\n\x05\x04\x0b\x02\0\x05\x12\x03Z\x04\n\n\
    \x0c\n\x05\x04\x0b\x02\0\x01\x12\x03Z\x0b\x11\n\x0c\n\x05\x04\x0b\x02\0\
    \x03\x12\x03Z\x14\x15\n\x0b\n\x04\x04\x0b\x02\x01\x12\x03[\x04\x20\n\r\n\
    \x05\x04\x0b\x02\x01\x04\x12\x04[\x04Z\x16\n\x0c\n\x05\x04\x0b\x02\x01\
    \x05\x12\x03[\x04\n\n\x0c\n\x05\x04\x0b\x02\x01\x01\x12\x03[\x0b\x1b\n\
    \x0c\n\x05\x04\x0b\x02\x01\x03\x12\x03[\x1e\x1f\n\x0b\n\x04\x04\x0b\x02\
    \x02\x12\x03\\\x04\x13\n\r\n\x05\x04\x0b\x02\x02\x04\x12\x04\\\x04[\x20\
    \n\x0c\n\x05\x04\x0b\x02\x02\x05\x12\x03\\\x04\n\n\x0c\n\x05\x04\x0b\x02\
    \x02\x01\x12\x03\\\x0b\x0e\n\x0c\n\x05\x04\x0b\x02\x02\x03\x12\x03\\\x11\
    \x12\n\n\n\x02\x04\x0c\x12\x04_\0a\x01\n\n\n\x03\x04\x0c\x01\x12\x03_\
    \x08\x14\n\x0b\n\x04\x04\x0c\x02\0\x12\x03`\x04\x15\n\r\n\x05\x04\x0c\
    \x02\0\x04\x12\x04`\x04_\x16\n\x0c\n\x05\x04\x0c\x02\0\x05\x12\x03`\x04\
    \n\n\x0c\n\x05\x04\x0c\x02\0\x01\x12\x03`\x0b\x10\n\x0c\n\x05\x04\x0c\
    \x02\0\x03\x12\x03`\x13\x14\n\n\n\x02\x06\0\x12\x04c\0f\x01\n\n\n\x03\
    \x06\0\x01\x12\x03c\x08\x17\n\x0b\n\x04\x06\0\x02\0\x12\x03d\x04>\n\x0c\
    \n\x05\x06\0\x02\0\x01\x12\x03d\x08\x10\n\x0c\n\x05\x06\0\x02\0\x02\x12\
    \x03d\x11\x20\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03d+;\n\x0b\n\x04\x06\0\
    \x02\x01\x12\x03e\x042\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03e\x08\x0c\n\
    \x0c\n\x05\x06\0\x02\x01\x02\x12\x03e\r\x18\n\x0c\n\x05\x06\0\x02\x01\
    \x03\x12\x03e#/b\x06proto3\
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
