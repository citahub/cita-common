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
pub struct TransAddr {
    // message fields
    pub block_hash: ::std::vec::Vec<u8>,
    pub index: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransAddr {}

impl TransAddr {
    pub fn new() -> TransAddr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransAddr {
        static mut instance: ::protobuf::lazy::Lazy<TransAddr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransAddr,
        };
        unsafe {
            instance.get(TransAddr::new)
        }
    }

    // bytes block_hash = 1;

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

    // uint64 index = 2;

    pub fn clear_index(&mut self) {
        self.index = 0;
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = v;
    }

    pub fn get_index(&self) -> u64 {
        self.index
    }

    fn get_index_for_reflect(&self) -> &u64 {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut u64 {
        &mut self.index
    }
}

impl ::protobuf::Message for TransAddr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.block_hash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.index = tmp;
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
        if !self.block_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.block_hash);
        }
        if self.index != 0 {
            my_size += ::protobuf::rt::value_size(2, self.index, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.block_hash.is_empty() {
            os.write_bytes(1, &self.block_hash)?;
        }
        if self.index != 0 {
            os.write_uint64(2, self.index)?;
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

impl ::protobuf::MessageStatic for TransAddr {
    fn new() -> TransAddr {
        TransAddr::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransAddr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "block_hash",
                    TransAddr::get_block_hash_for_reflect,
                    TransAddr::mut_block_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    TransAddr::get_index_for_reflect,
                    TransAddr::mut_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransAddr>(
                    "TransAddr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransAddr {
    fn clear(&mut self) {
        self.clear_block_hash();
        self.clear_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransAddr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransAddr {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExecutedInfo {
    // message fields
    pub header: ::protobuf::SingularPtrField<ExecutedHeader>,
    pub transactions: ::std::collections::HashMap<::std::string::String, TransAddr>,
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

    // repeated .ExecutedInfo.TransactionsEntry transactions = 2;

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions(&mut self, v: ::std::collections::HashMap<::std::string::String, TransAddr>) {
        self.transactions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transactions(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, TransAddr> {
        &mut self.transactions
    }

    // Take field
    pub fn take_transactions(&mut self) -> ::std::collections::HashMap<::std::string::String, TransAddr> {
        ::std::mem::replace(&mut self.transactions, ::std::collections::HashMap::new())
    }

    pub fn get_transactions(&self) -> &::std::collections::HashMap<::std::string::String, TransAddr> {
        &self.transactions
    }

    fn get_transactions_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, TransAddr> {
        &self.transactions
    }

    fn mut_transactions_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, TransAddr> {
        &mut self.transactions
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
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TransAddr>>(wire_type, is, &mut self.transactions)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TransAddr>>(2, &self.transactions);
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
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TransAddr>>(2, &self.transactions, os)?;
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
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TransAddr>>(
                    "transactions",
                    ExecutedInfo::get_transactions_for_reflect,
                    ExecutedInfo::mut_transactions_for_reflect,
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
        self.clear_transactions();
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

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReceiptError {
    NoTransactionPermission = 0,
    NoContractPermission = 1,
    NotEnoughBaseGas = 2,
    BlockGasLimitReached = 3,
    AccountGasLimitReached = 4,
    OutOfGas = 5,
    BadJumpDestination = 6,
    BadInstruction = 7,
    StackUnderflow = 8,
    OutOfStack = 9,
    Internal = 10,
}

impl ::protobuf::ProtobufEnum for ReceiptError {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReceiptError> {
        match value {
            0 => ::std::option::Option::Some(ReceiptError::NoTransactionPermission),
            1 => ::std::option::Option::Some(ReceiptError::NoContractPermission),
            2 => ::std::option::Option::Some(ReceiptError::NotEnoughBaseGas),
            3 => ::std::option::Option::Some(ReceiptError::BlockGasLimitReached),
            4 => ::std::option::Option::Some(ReceiptError::AccountGasLimitReached),
            5 => ::std::option::Option::Some(ReceiptError::OutOfGas),
            6 => ::std::option::Option::Some(ReceiptError::BadJumpDestination),
            7 => ::std::option::Option::Some(ReceiptError::BadInstruction),
            8 => ::std::option::Option::Some(ReceiptError::StackUnderflow),
            9 => ::std::option::Option::Some(ReceiptError::OutOfStack),
            10 => ::std::option::Option::Some(ReceiptError::Internal),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReceiptError] = &[
            ReceiptError::NoTransactionPermission,
            ReceiptError::NoContractPermission,
            ReceiptError::NotEnoughBaseGas,
            ReceiptError::BlockGasLimitReached,
            ReceiptError::AccountGasLimitReached,
            ReceiptError::OutOfGas,
            ReceiptError::BadJumpDestination,
            ReceiptError::BadInstruction,
            ReceiptError::StackUnderflow,
            ReceiptError::OutOfStack,
            ReceiptError::Internal,
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
    \n\x0eexecuter.proto\x1a\x10blockchain.proto\"\xa8\x02\n\x0eExecutedHead\
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
    ateRoot\x12\x1d\n\nstate_root\x18\x01\x20\x01(\x0cR\tstateRoot\"\xba\x01\
    \n\x07Receipt\x12)\n\nstate_root\x18\x01\x20\x01(\x0b2\n.StateRootR\tsta\
    teRoot\x12\x19\n\x08gas_used\x18\x02\x20\x01(\tR\x07gasUsed\x12\x1b\n\tl\
    og_bloom\x18\x03\x20\x01(\x0cR\x08logBloom\x12\x1d\n\x04logs\x18\x04\x20\
    \x03(\x0b2\t.LogEntryR\x04logs\x12-\n\x05error\x18\x05\x20\x01(\x0b2\x17\
    .ReceiptErrorWithOptionR\x05error\"7\n\x11ReceiptWithOption\x12\"\n\x07r\
    eceipt\x18\x01\x20\x01(\x0b2\x08.ReceiptR\x07receipt\"@\n\tTransAddr\x12\
    \x1d\n\nblock_hash\x18\x01\x20\x01(\x0cR\tblockHash\x12\x14\n\x05index\
    \x18\x02\x20\x01(\x04R\x05index\"\xf9\x01\n\x0cExecutedInfo\x12'\n\x06he\
    ader\x18\x01\x20\x01(\x0b2\x0f.ExecutedHeaderR\x06header\x12C\n\x0ctrans\
    actions\x18\x02\x20\x03(\x0b2\x1f.ExecutedInfo.TransactionsEntryR\x0ctra\
    nsactions\x12.\n\x08receipts\x18\x03\x20\x03(\x0b2\x12.ReceiptWithOption\
    R\x08receipts\x1aK\n\x11TransactionsEntry\x12\x10\n\x03key\x18\x01\x20\
    \x01(\tR\x03key\x12\x20\n\x05value\x18\x02\x20\x01(\x0b2\n.TransAddrR\
    \x05value:\x028\x01\"\x8d\x01\n\x0fConsensusConfig\x12&\n\x0fblock_gas_l\
    imit\x18\x01\x20\x01(\x04R\rblockGasLimit\x12<\n\x11account_gas_limit\
    \x18\x02\x20\x01(\x0b2\x10.AccountGasLimitR\x0faccountGasLimit\x12\x14\n\
    \x05nodes\x18\x03\x20\x03(\x0cR\x05nodes\"n\n\x0eExecutedResult\x122\n\r\
    executed_info\x18\x01\x20\x01(\x0b2\r.ExecutedInfoR\x0cexecutedInfo\x12(\
    \n\x06config\x18\x02\x20\x01(\x0b2\x10.ConsensusConfigR\x06config*\xfd\
    \x01\n\x0cReceiptError\x12\x1b\n\x17NoTransactionPermission\x10\0\x12\
    \x18\n\x14NoContractPermission\x10\x01\x12\x14\n\x10NotEnoughBaseGas\x10\
    \x02\x12\x18\n\x14BlockGasLimitReached\x10\x03\x12\x1a\n\x16AccountGasLi\
    mitReached\x10\x04\x12\x0c\n\x08OutOfGas\x10\x05\x12\x16\n\x12BadJumpDes\
    tination\x10\x06\x12\x12\n\x0eBadInstruction\x10\x07\x12\x12\n\x0eStackU\
    nderflow\x10\x08\x12\x0e\n\nOutOfStack\x10\t\x12\x0c\n\x08Internal\x10\n\
    J\xa9\x16\n\x06\x12\x04\0\0M\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\
    \x02\x03\0\x12\x03\x02\x07\x19\n\n\n\x02\x04\0\x12\x04\x04\0\x0e\x01\n\n\
    \n\x03\x04\0\x01\x12\x03\x04\x08\x16\n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\
    \x04\x17\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x05\x04\x04\x18\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x05\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x05\n\x12\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x05\x15\x16\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\x06\x04\x19\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\
    \x06\x04\x05\x17\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x06\x04\n\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\x06\x0b\x14\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x06\x17\x18\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x07\x04\x16\n\r\n\
    \x05\x04\0\x02\x02\x04\x12\x04\x07\x04\x06\x19\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x07\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x07\x0b\x11\
    \n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x07\x14\x15\n\x0b\n\x04\x04\0\x02\
    \x03\x12\x03\x08\x04\x19\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x08\x04\x07\
    \x16\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x08\x04\t\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x03\x08\n\x14\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x08\
    \x17\x18\n\x0b\n\x04\x04\0\x02\x04\x12\x03\t\x04\x20\n\r\n\x05\x04\0\x02\
    \x04\x04\x12\x04\t\x04\x08\x19\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\t\
    \x04\t\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\t\n\x1b\n\x0c\n\x05\x04\0\
    \x02\x04\x03\x12\x03\t\x1e\x1f\n\x0b\n\x04\x04\0\x02\x05\x12\x03\n\x04\
    \x1c\n\r\n\x05\x04\0\x02\x05\x04\x12\x04\n\x04\t\x20\n\x0c\n\x05\x04\0\
    \x02\x05\x05\x12\x03\n\x04\t\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\n\n\
    \x17\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\n\x1a\x1b\n\x0b\n\x04\x04\0\
    \x02\x06\x12\x03\x0b\x04\x18\n\r\n\x05\x04\0\x02\x06\x04\x12\x04\x0b\x04\
    \n\x1c\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\x0b\x04\t\n\x0c\n\x05\x04\0\
    \x02\x06\x01\x12\x03\x0b\n\x13\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x0b\
    \x16\x17\n\x0b\n\x04\x04\0\x02\x07\x12\x03\x0c\x04\x18\n\r\n\x05\x04\0\
    \x02\x07\x04\x12\x04\x0c\x04\x0b\x18\n\x0c\n\x05\x04\0\x02\x07\x05\x12\
    \x03\x0c\x04\n\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\x0c\x0b\x13\n\x0c\n\
    \x05\x04\0\x02\x07\x03\x12\x03\x0c\x16\x17\n\x0b\n\x04\x04\0\x02\x08\x12\
    \x03\r\x04\x19\n\r\n\x05\x04\0\x02\x08\x04\x12\x04\r\x04\x0c\x18\n\x0c\n\
    \x05\x04\0\x02\x08\x05\x12\x03\r\x04\n\n\x0c\n\x05\x04\0\x02\x08\x01\x12\
    \x03\r\x0b\x14\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03\r\x17\x18\n\n\n\x02\
    \x05\0\x12\x04\x10\0\x1d\x01\n\n\n\x03\x05\0\x01\x12\x03\x10\x05\x11\n\
    \x1c\n\x04\x05\0\x02\0\x12\x03\x12\x04\x20\x1a\x0fExecutionError\n\n\x0c\
    \n\x05\x05\0\x02\0\x01\x12\x03\x12\x04\x1b\n\x0c\n\x05\x05\0\x02\0\x02\
    \x12\x03\x12\x1e\x1f\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x13\x04\x1d\n\x0c\
    \n\x05\x05\0\x02\x01\x01\x12\x03\x13\x04\x18\n\x0c\n\x05\x05\0\x02\x01\
    \x02\x12\x03\x13\x1b\x1c\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x14\x04\x19\n\
    \x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x14\x04\x14\n\x0c\n\x05\x05\0\x02\
    \x02\x02\x12\x03\x14\x17\x18\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x15\x04\
    \x1e\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\x15\x04\x18\n\x0c\n\x05\x05\0\
    \x02\x03\x02\x12\x03\x15\x1c\x1d\n\x0b\n\x04\x05\0\x02\x04\x12\x03\x16\
    \x04\x20\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x16\x04\x1a\n\x0c\n\x05\
    \x05\0\x02\x04\x02\x12\x03\x16\x1e\x1f\n\x0b\n\x04\x05\0\x02\x05\x12\x03\
    \x17\x04\x11\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03\x17\x04\x0c\n\x0c\n\
    \x05\x05\0\x02\x05\x02\x12\x03\x17\x0f\x10\n\x0b\n\x04\x05\0\x02\x06\x12\
    \x03\x18\x04\x1b\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03\x18\x04\x16\n\x0c\
    \n\x05\x05\0\x02\x06\x02\x12\x03\x18\x19\x1a\n\x0b\n\x04\x05\0\x02\x07\
    \x12\x03\x19\x04\x17\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03\x19\x04\x12\n\
    \x0c\n\x05\x05\0\x02\x07\x02\x12\x03\x19\x15\x16\n\x0b\n\x04\x05\0\x02\
    \x08\x12\x03\x1a\x04\x17\n\x0c\n\x05\x05\0\x02\x08\x01\x12\x03\x1a\x04\
    \x12\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x03\x1a\x15\x16\n\x0b\n\x04\x05\0\
    \x02\t\x12\x03\x1b\x04\x13\n\x0c\n\x05\x05\0\x02\t\x01\x12\x03\x1b\x04\
    \x0e\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03\x1b\x11\x12\n\x0b\n\x04\x05\0\
    \x02\n\x12\x03\x1c\x04\x12\n\x0c\n\x05\x05\0\x02\n\x01\x12\x03\x1c\x04\
    \x0c\n\x0c\n\x05\x05\0\x02\n\x02\x12\x03\x1c\x0f\x11\n\n\n\x02\x04\x01\
    \x12\x04\x1f\0#\x01\n\n\n\x03\x04\x01\x01\x12\x03\x1f\x08\x10\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03\x20\x04\x16\n\r\n\x05\x04\x01\x02\0\x04\x12\
    \x04\x20\x04\x1f\x12\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x20\x04\t\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x20\n\x11\n\x0c\n\x05\x04\x01\x02\0\
    \x03\x12\x03\x20\x14\x15\n\x0b\n\x04\x04\x01\x02\x01\x12\x03!\x04\x1e\n\
    \x0c\n\x05\x04\x01\x02\x01\x04\x12\x03!\x04\x0c\n\x0c\n\x05\x04\x01\x02\
    \x01\x05\x12\x03!\r\x12\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03!\x13\x19\
    \n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03!\x1c\x1d\n\x0b\n\x04\x04\x01\
    \x02\x02\x12\x03\"\x04\x13\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\"\x04!\
    \x1e\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\"\x04\t\n\x0c\n\x05\x04\x01\
    \x02\x02\x01\x12\x03\"\n\x0e\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\"\
    \x11\x12\n\n\n\x02\x04\x02\x12\x04%\0'\x01\n\n\n\x03\x04\x02\x01\x12\x03\
    %\x08\x1e\n\x0b\n\x04\x04\x02\x02\0\x12\x03&\x04\x1b\n\r\n\x05\x04\x02\
    \x02\0\x04\x12\x04&\x04%\x20\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03&\x04\
    \x10\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03&\x11\x16\n\x0c\n\x05\x04\x02\
    \x02\0\x03\x12\x03&\x19\x1a\n\n\n\x02\x04\x03\x12\x04)\0+\x01\n\n\n\x03\
    \x04\x03\x01\x12\x03)\x08\x11\n\x0b\n\x04\x04\x03\x02\0\x12\x03*\x04\x19\
    \n\r\n\x05\x04\x03\x02\0\x04\x12\x04*\x04)\x13\n\x0c\n\x05\x04\x03\x02\0\
    \x05\x12\x03*\x04\t\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03*\n\x14\n\x0c\n\
    \x05\x04\x03\x02\0\x03\x12\x03*\x17\x18\n\n\n\x02\x04\x04\x12\x04-\03\
    \x01\n\n\n\x03\x04\x04\x01\x12\x03-\x08\x0f\n\x0b\n\x04\x04\x04\x02\0\
    \x12\x03.\x04\x1d\n\r\n\x05\x04\x04\x02\0\x04\x12\x04.\x04-\x11\n\x0c\n\
    \x05\x04\x04\x02\0\x06\x12\x03.\x04\r\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x03.\x0e\x18\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03.\x1b\x1c\n\x0b\n\x04\
    \x04\x04\x02\x01\x12\x03/\x04\x18\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04/\
    \x04.\x1d\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03/\x04\n\n\x0c\n\x05\x04\
    \x04\x02\x01\x01\x12\x03/\x0b\x13\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\
    \x03/\x16\x17\n\x0b\n\x04\x04\x04\x02\x02\x12\x030\x04\x18\n\r\n\x05\x04\
    \x04\x02\x02\x04\x12\x040\x04/\x18\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\
    \x030\x04\t\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x030\n\x13\n\x0c\n\x05\
    \x04\x04\x02\x02\x03\x12\x030\x16\x17\n\x0b\n\x04\x04\x04\x02\x03\x12\
    \x031\x04\x1f\n\x0c\n\x05\x04\x04\x02\x03\x04\x12\x031\x04\x0c\n\x0c\n\
    \x05\x04\x04\x02\x03\x06\x12\x031\r\x15\n\x0c\n\x05\x04\x04\x02\x03\x01\
    \x12\x031\x16\x1a\n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x031\x1d\x1e\n\x0b\
    \n\x04\x04\x04\x02\x04\x12\x032\x04%\n\r\n\x05\x04\x04\x02\x04\x04\x12\
    \x042\x041\x1f\n\x0c\n\x05\x04\x04\x02\x04\x06\x12\x032\x04\x1a\n\x0c\n\
    \x05\x04\x04\x02\x04\x01\x12\x032\x1b\x20\n\x0c\n\x05\x04\x04\x02\x04\
    \x03\x12\x032#$\n\n\n\x02\x04\x05\x12\x045\07\x01\n\n\n\x03\x04\x05\x01\
    \x12\x035\x08\x19\n\x0b\n\x04\x04\x05\x02\0\x12\x036\x04\x18\n\r\n\x05\
    \x04\x05\x02\0\x04\x12\x046\x045\x1b\n\x0c\n\x05\x04\x05\x02\0\x06\x12\
    \x036\x04\x0b\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x036\x0c\x13\n\x0c\n\x05\
    \x04\x05\x02\0\x03\x12\x036\x16\x17\n\n\n\x02\x04\x06\x12\x049\0<\x01\n\
    \n\n\x03\x04\x06\x01\x12\x039\x08\x11\n\x0b\n\x04\x04\x06\x02\0\x12\x03:\
    \x04\x19\n\r\n\x05\x04\x06\x02\0\x04\x12\x04:\x049\x13\n\x0c\n\x05\x04\
    \x06\x02\0\x05\x12\x03:\x04\t\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03:\n\
    \x14\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03:\x17\x18\n\x0b\n\x04\x04\x06\
    \x02\x01\x12\x03;\x04\x15\n\r\n\x05\x04\x06\x02\x01\x04\x12\x04;\x04:\
    \x19\n\x0c\n\x05\x04\x06\x02\x01\x05\x12\x03;\x04\n\n\x0c\n\x05\x04\x06\
    \x02\x01\x01\x12\x03;\x0b\x10\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03;\
    \x13\x14\n\n\n\x02\x04\x07\x12\x04>\0B\x01\n\n\n\x03\x04\x07\x01\x12\x03\
    >\x08\x14\n\x0b\n\x04\x04\x07\x02\0\x12\x03?\x04\x1e\n\r\n\x05\x04\x07\
    \x02\0\x04\x12\x04?\x04>\x16\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x03?\x04\
    \x12\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03?\x13\x19\n\x0c\n\x05\x04\x07\
    \x02\0\x03\x12\x03?\x1c\x1d\n\x0b\n\x04\x04\x07\x02\x01\x12\x03@\x04,\n\
    \r\n\x05\x04\x07\x02\x01\x04\x12\x04@\x04?\x1e\n\x0c\n\x05\x04\x07\x02\
    \x01\x06\x12\x03@\x04\x1a\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x03@\x1b'\
    \n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03@*+\n\x0b\n\x04\x04\x07\x02\x02\
    \x12\x03A\x04,\n\x0c\n\x05\x04\x07\x02\x02\x04\x12\x03A\x04\x0c\n\x0c\n\
    \x05\x04\x07\x02\x02\x06\x12\x03A\r\x1e\n\x0c\n\x05\x04\x07\x02\x02\x01\
    \x12\x03A\x1f'\n\x0c\n\x05\x04\x07\x02\x02\x03\x12\x03A*+\n\n\n\x02\x04\
    \x08\x12\x04D\0H\x01\n\n\n\x03\x04\x08\x01\x12\x03D\x08\x17\n\x0b\n\x04\
    \x04\x08\x02\0\x12\x03E\x04\x1f\n\r\n\x05\x04\x08\x02\0\x04\x12\x04E\x04\
    D\x19\n\x0c\n\x05\x04\x08\x02\0\x05\x12\x03E\x04\n\n\x0c\n\x05\x04\x08\
    \x02\0\x01\x12\x03E\x0b\x1a\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03E\x1d\
    \x1e\n\x0b\n\x04\x04\x08\x02\x01\x12\x03F\x04*\n\r\n\x05\x04\x08\x02\x01\
    \x04\x12\x04F\x04E\x1f\n\x0c\n\x05\x04\x08\x02\x01\x06\x12\x03F\x04\x13\
    \n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03F\x14%\n\x0c\n\x05\x04\x08\x02\
    \x01\x03\x12\x03F()\n\x0b\n\x04\x04\x08\x02\x02\x12\x03G\x04\x1d\n\x0c\n\
    \x05\x04\x08\x02\x02\x04\x12\x03G\x04\x0c\n\x0c\n\x05\x04\x08\x02\x02\
    \x05\x12\x03G\r\x12\n\x0c\n\x05\x04\x08\x02\x02\x01\x12\x03G\x13\x18\n\
    \x0c\n\x05\x04\x08\x02\x02\x03\x12\x03G\x1b\x1c\n\n\n\x02\x04\t\x12\x04J\
    \0M\x01\n\n\n\x03\x04\t\x01\x12\x03J\x08\x16\n\x0b\n\x04\x04\t\x02\0\x12\
    \x03K\x04#\n\r\n\x05\x04\t\x02\0\x04\x12\x04K\x04J\x18\n\x0c\n\x05\x04\t\
    \x02\0\x06\x12\x03K\x04\x10\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03K\x11\x1e\
    \n\x0c\n\x05\x04\t\x02\0\x03\x12\x03K!\"\n\x0b\n\x04\x04\t\x02\x01\x12\
    \x03L\x04\x1f\n\r\n\x05\x04\t\x02\x01\x04\x12\x04L\x04K#\n\x0c\n\x05\x04\
    \t\x02\x01\x06\x12\x03L\x04\x13\n\x0c\n\x05\x04\t\x02\x01\x01\x12\x03L\
    \x14\x1a\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03L\x1d\x1eb\x06proto3\
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
