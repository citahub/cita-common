## Overview

Provide the type of interaction between the cita-node 
interaction type and the cita component, use protobuf 
to generate the corresponding **.rs file which include 
types and some set/get methods.

## Dependences

- [protobuf 3.5.1](https://github.com/google/protobuf/releases)
- [rust-protobuf v2.0.2](https://github.com/stepancheg/rust-protobuf)
- [grpc-rust 0.5.0](https://github.com/stepancheg/grpc-rust)

Currently only supports these versions. If there is a break version, 
it is temporarily not supported.

## Usage

1. Install google protoc

2. Install rust plugin

```
$ cargo install protobuf-codegen --vers 2.0.2
$ cargo install grpc-compiler --vers 0.5.0
```

3. You can start modifying `./src/proto/*.proto`
and use `./create_protobuf.sh` regenerate `*.rs`,
finally synchronized to the cita-proto library
