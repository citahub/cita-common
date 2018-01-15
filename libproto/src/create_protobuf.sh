#!/usr/bin/env bash

find . -name "*.proto" | while read protofile; do
    protoc ${protofile} --rust_out .
done

function add_pub_to_oneof_in_generated_code () {
    local update_file="$1"
    local dataname="$2"
    local datatype="$3"
    local update_part="${dataname}: ::std::option::Option<${datatype}_oneof_${dataname}>,"
    local sed_opts=
    case "$OSTYPE" in
        darwin*)
            sed_opts="-g"
            ;;
        *)
            sed_opts=
            ;;
    esac
    sed -i ${sed_opts} "s/\(\s\)\(${update_part}\)/\1pub \2/g" "${update_file}"
}

add_pub_to_oneof_in_generated_code response.rs      data    Response
add_pub_to_oneof_in_generated_code request.rs       req     Request
add_pub_to_oneof_in_generated_code communication.rs content Message

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
