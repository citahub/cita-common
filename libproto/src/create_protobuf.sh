#!/bin/bash

find . -name "*.proto" | while read protofile; do
    protoc ${protofile} --rust_out .
done

case "$OSTYPE" in
    darwin*)
        sed -ig 's/    data: ::std::option::Option<Response_oneof_data>,/    pub data: ::std::option::Option<Response_oneof_data>,/g' response.rs
        sed -ig 's/    req: ::std::option::Option<Request_oneof_req>,/    pub req: ::std::option::Option<Request_oneof_req>,/g' request.rs
        ;;
    *)
        sed -i 's/    data: ::std::option::Option<Response_oneof_data>,/    pub data: ::std::option::Option<Response_oneof_data>,/g' response.rs
        sed -i 's/    req: ::std::option::Option<Request_oneof_req>,/    pub req: ::std::option::Option<Request_oneof_req>,/g' request.rs
        ;;
esac

for i in `find . -name "*.rs"`
do
    if grep -q -e "Copyright 2015-20.. Parity Technologies" -e "Copyright 2016-20.. Cryptape Technologies" $i
    then
        echo "Ignoring the " $i
    else
        echo "Starting modify" $i
        (cat ../../LICENSE_HEADER | cat - $i > file1) && mv file1 $i
    fi
done
