// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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


// interface

pub trait CitacodeService {
    fn init(&self, o: ::grpc::RequestOptions, p: super::citacode::InvokeRequest) -> ::grpc::SingleResponse<super::citacode::InvokeResponse>;

    fn invoke(&self, o: ::grpc::RequestOptions, p: super::citacode::InvokeRequest) -> ::grpc::SingleResponse<super::citacode::InvokeResponse>;
}

// client

pub struct CitacodeServiceClient {
    grpc_client: ::grpc::Client,
    method_Init: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::citacode::InvokeRequest, super::citacode::InvokeResponse>>,
    method_Invoke: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::citacode::InvokeRequest, super::citacode::InvokeResponse>>,
}

impl CitacodeServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        CitacodeServiceClient {
            grpc_client: grpc_client,
            method_Init: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/citacode.CitacodeService/Init".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Invoke: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/citacode.CitacodeService/Invoke".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            CitacodeServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            CitacodeServiceClient::with_client(c)
        })
    }
}

impl CitacodeService for CitacodeServiceClient {
    fn init(&self, o: ::grpc::RequestOptions, p: super::citacode::InvokeRequest) -> ::grpc::SingleResponse<super::citacode::InvokeResponse> {
        self.grpc_client.call_unary(o, p, self.method_Init.clone())
    }

    fn invoke(&self, o: ::grpc::RequestOptions, p: super::citacode::InvokeRequest) -> ::grpc::SingleResponse<super::citacode::InvokeResponse> {
        self.grpc_client.call_unary(o, p, self.method_Invoke.clone())
    }
}

// server

pub struct CitacodeServiceServer;


impl CitacodeServiceServer {
    pub fn new_service_def<H : CitacodeService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/citacode.CitacodeService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/citacode.CitacodeService/Init".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.init(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/citacode.CitacodeService/Invoke".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.invoke(o, p))
                    },
                ),
            ],
        )
    }
}
