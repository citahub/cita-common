#!/usr/bin/env bash

rootdir=$(readlink -f "$(dirname $0)")
currdir=$(pwd)

function remove_all_rs () {
    find . -name "*.rs" -exec rm -v {} \;
}

function gen_rs_for_protos () {
    find . -name "*.proto" | while read protofile; do
        protoc ${protofile} --rust_out .
    done
}

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

function add_license () {
    for i in `find . -name "*.rs"`
    do
        if grep -q -e "Copyright 2015-20.. Parity Technologies" -e "Copyright 2016-20.. Cryptape Technologies" $i
        then
            echo "Ignoring the " $i
        else
            echo "Starting modify" $i
            (cat ../../../LICENSE_HEADER | cat - $i > file1) && mv file1 $i
        fi
    done
}

function generate_readme () {
    cat <<EOF
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

EOF
}

function gen_modrs_for_protos () {
    local modrs="mod.rs"
    generate_readme > "${modrs}"
    find . -maxdepth 1 -name "*.proto" \
            -exec basename {} \; \
            | sort \
            | cut -d"." -f 1 | while read name; do
        echo "pub mod ${name};" >> "${modrs}"
    done
    echo >> "${modrs}"
    find . -maxdepth 1 -name "*.proto" \
            -exec basename {} \; \
            | sort \
            | cut -d"." -f 1 | while read name; do
        items=$(grep "^pub [se].* {$" "${name}.rs" | sort | awk '{ printf $3", " }')
        echo "pub use self::${name}::{${items/%, }};" >> "${modrs}"
    done
}

function generate_impls_for_all () {
    local indentation="            "
    local replace_begin="${indentation}\\/\\/ Generate ALL-PROTOS automatically begin:"
    local replace_end="${indentation}\\/\\/ Generate ALL-PROTOS automatically end."
    local rsfile="../lib.rs"
    sed -i "/^${replace_begin}$/,/^${replace_end}$/{//!d}" "${rsfile}"
    grep "^pub struct .* {$" *.rs | sort \
            | awk '{ print $3 }' | uniq \
            | while read struct; do
        sed -i -e "/^${replace_end}$/i\\${indentation}${struct}," "${rsfile}"
    done
}

function generate_impls_for_msg () {
    local indentation="            "
    local replace_begin_1="${indentation}\\/\\/ Generate MSG-PROTOS automatically begin:"
    local replace_end_1="${indentation}\\/\\/ Generate MSG-PROTOS automatically end."
    local replace_begin_2="${indentation}\\/\\/ Generate MSG-PROTOS set_content automatically begin:"
    local replace_end_2="${indentation}\\/\\/ Generate MSG-PROTOS set_content automatically end."
    local replace_begin_3="${indentation}\\/\\/ Generate MSG-PROTOS from_content automatically begin:"
    local replace_end_3="${indentation}\\/\\/ Generate MSG-PROTOS from_content automatically end."
    local rsfile="../lib.rs"
    sed -i "/^${replace_begin_1}$/,/^${replace_end_1}$/{//!d}" "${rsfile}"
    sed -i "/^${replace_begin_2}$/,/^${replace_end_2}$/{//!d}" "${rsfile}"
    sed -i "/^${replace_begin_3}$/,/^${replace_end_3}$/{//!d}" "${rsfile}"
    sed -n '/^    oneof content {$/,/^    }$/p' "communication.proto" \
            | grep "^\s\{8\}[A-Z].*;$" | awk '{ print $2 }' \
            | while read struct; do
        sed -i -e "/^${replace_end_1}$/i\\${indentation}${struct}," "${rsfile}"
        sed -i -e "/^${replace_end_2}$/i\\${indentation}MsgClass::${struct}(data) => self.set_${struct}(data)," "${rsfile}"
        sed -i -e "/^${replace_end_3}$/i\\${indentation}Message_oneof_content::${struct}(data) => data.into()," "${rsfile}"
    done
}

function main () {
    cd "${rootdir}"
    remove_all_rs
    gen_rs_for_protos
    add_pub_to_oneof_in_generated_code response.rs      data    Response
    add_pub_to_oneof_in_generated_code request.rs       req     Request
    add_pub_to_oneof_in_generated_code communication.rs content Message
    generate_impls_for_all
    generate_impls_for_msg
    gen_modrs_for_protos
    add_license
    cd "${currdir}"
}

main
