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


// interface

pub trait ExecutorService {
    fn register(&self, o: ::grpc::RequestOptions, p: super::executor::RegisterRequest) -> ::grpc::SingleResponse<super::executor::RegisterResponse>;

    fn load(&self, o: ::grpc::RequestOptions, p: super::executor::LoadRequest) -> ::grpc::SingleResponse<super::executor::LoadResponse>;
}

// client

pub struct ExecutorServiceClient {
    grpc_client: ::grpc::Client,
    method_Register: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::executor::RegisterRequest, super::executor::RegisterResponse>>,
    method_Load: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::executor::LoadRequest, super::executor::LoadResponse>>,
}

impl ExecutorServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ExecutorServiceClient {
            grpc_client: grpc_client,
            method_Register: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ExecutorService/Register".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Load: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ExecutorService/Load".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            ExecutorServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            ExecutorServiceClient::with_client(c)
        })
    }
}

impl ExecutorService for ExecutorServiceClient {
    fn register(&self, o: ::grpc::RequestOptions, p: super::executor::RegisterRequest) -> ::grpc::SingleResponse<super::executor::RegisterResponse> {
        self.grpc_client.call_unary(o, p, self.method_Register.clone())
    }

    fn load(&self, o: ::grpc::RequestOptions, p: super::executor::LoadRequest) -> ::grpc::SingleResponse<super::executor::LoadResponse> {
        self.grpc_client.call_unary(o, p, self.method_Load.clone())
    }
}

// server

pub struct ExecutorServiceServer;


impl ExecutorServiceServer {
    pub fn new_service_def<H : ExecutorService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/ExecutorService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ExecutorService/Register".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.register(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ExecutorService/Load".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.load(o, p))
                    },
                ),
            ],
        )
    }
}
