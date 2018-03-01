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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\radapter.proto\x12\x07adapter\"`\n\x0fRegisterRequest\x12)\n\x10contr\
    act_address\x18\x01\x20\x01(\tR\x0fcontractAddress\x12\x0e\n\x02ip\x18\
    \x02\x20\x01(\tR\x02ip\x12\x12\n\x04port\x18\x03\x20\x01(\tR\x04port\"(\
    \n\x10RegisterResponse\x12\x14\n\x05state\x18\x01\x20\x01(\tR\x05state2M\
    \n\x08Register\x12A\n\x08SayHello\x12\x18.adapter.RegisterRequest\x1a\
    \x19.adapter.RegisterResponse\"\0J\xc7\x03\n\x06\x12\x04\0\0\x10\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x0f\n\n\n\
    \x02\x06\0\x12\x04\x03\0\x06\x01\n\n\n\x03\x06\0\x01\x12\x03\x03\x08\x10\
    \n\x1f\n\x04\x06\0\x02\0\x12\x03\x05\x04@\x1a\x12\x20Sends\x20a\x20greet\
    ing\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x05\x08\x10\n\x0c\n\x05\x06\0\
    \x02\0\x02\x12\x03\x05\x12!\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x05,<\n\
    \n\n\x02\x04\0\x12\x04\x08\0\x0c\x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\
    \x17\n\x0b\n\x04\x04\0\x02\0\x12\x03\t\x04\x20\n\r\n\x05\x04\0\x02\0\x04\
    \x12\x04\t\x04\x08\x19\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\t\x04\n\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\t\x0b\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\t\x1e\x1f\n\x0b\n\x04\x04\0\x02\x01\x12\x03\n\x04\x12\n\r\n\x05\x04\
    \0\x02\x01\x04\x12\x04\n\x04\t\x20\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\
    \n\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\n\x0b\r\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\n\x10\x11\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0b\x04\
    \x14\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x0b\x04\n\x12\n\x0c\n\x05\x04\0\
    \x02\x02\x05\x12\x03\x0b\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0b\
    \x0b\x0f\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0b\x12\x13\n\n\n\x02\x04\
    \x01\x12\x04\x0e\0\x10\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0e\x08\x18\n\
    \x0b\n\x04\x04\x01\x02\0\x12\x03\x0f\x04\x15\n\r\n\x05\x04\x01\x02\0\x04\
    \x12\x04\x0f\x04\x0e\x1a\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0f\x04\n\
    \n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0f\x0b\x10\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03\x0f\x13\x14b\x06proto3\
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
