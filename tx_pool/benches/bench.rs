// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(test, feature(test))]
extern crate cita_crypto as crypto;
extern crate libproto;
extern crate test;
extern crate tx_pool;
extern crate util;

use crypto::{CreateKey, KeyPair};
use libproto::blockchain::AccountGasLimit;
use libproto::blockchain::Transaction;
use std::collections::HashMap;
use std::time::SystemTime;
use test::Bencher;
use tx_pool::pool::*;
#[bench]
fn bench_base(b: &mut Bencher) {
    let start = SystemTime::now();
    let mut tx = Transaction::new();
    for i in 0..10000 {
        tx.set_data(format!("{}", i).as_bytes().to_vec());
    }
    let sys_time = SystemTime::now();
    let diff = sys_time
        .duration_since(start)
        .expect("SystemTime::duration_since failed");
    println!("pass");
    println!(
        "test {:20} ... bench: {}.{} s/iter",
        "bench_base",
        diff.as_secs(),
        diff.subsec_nanos()
    );
    b.iter(|| {});
}

#[bench]
fn bench_enqueue(b: &mut Bencher) {
    let start = SystemTime::now();
    let mut p = Pool::new(1000);
    let mut tx = Transaction::new();
    let keypair = KeyPair::gen_keypair();
    let pv = keypair.privkey();
    for i in 0..10000 {
        tx.set_data(format!("{}", i).as_bytes().to_vec());
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(99);
        // 2000*10000 <= account_quota_limit <= block_quota_limit
        tx.set_quota(2000);
        p.enqueue(tx.sign(*pv));
    }
    let sys_time = SystemTime::now();
    let diff = sys_time
        .duration_since(start)
        .expect("SystemTime::duration_since failed");
    println!("pass");
    println!(
        "test {:20} ... bench: {}.{} s/iter",
        "bench_enqueue",
        diff.as_secs(),
        diff.subsec_nanos()
    );
    b.iter(|| {});
}

#[bench]
fn bench_package(b: &mut Bencher) {
    let start = SystemTime::now();
    let mut p = Pool::new(1000);
    let mut tx = Transaction::new();
    let keypair = KeyPair::gen_keypair();
    let pv = keypair.privkey();
    for i in 0..10000 {
        tx.set_data(format!("{}", i).as_bytes().to_vec());
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(99);
        // 6000*10000 <= account_quota_limit <= block_quota_limit
        tx.set_quota(6000);
        p.enqueue(tx.sign(*pv));
    }
    let mut account_quota_limit = AccountGasLimit::new();
    // set block_quota_limit default
    let block_quota_limit = 61415926;
    // height should less than valid_until_block
    let height = 0;
    // set account_quota_limit be equal as block_quota_limit
    account_quota_limit.set_common_quota_limit(block_quota_limit);
    account_quota_limit.set_specific_quota_limit(HashMap::new());

    p.package(height, block_quota_limit, account_quota_limit.clone());
    let sys_time = SystemTime::now();
    let diff = sys_time
        .duration_since(start)
        .expect("SystemTime::duration_since failed");
    println!("pass");
    println!(
        "test {:20} ... bench: {}.{} s/iter",
        "bench_package",
        diff.as_secs(),
        diff.subsec_nanos()
    );
    b.iter(|| {});
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let start = SystemTime::now();
    let mut p = Pool::new(1000);
    let mut tx = Transaction::new();
    let keypair = KeyPair::gen_keypair();
    let pv = keypair.privkey();

    for i in 0..10000 {
        tx.set_data(format!("{}", i).as_bytes().to_vec());
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(99);
        // 6000*10000 <= account_quota_limit <= block_quota_limit
        tx.set_quota(6000);
        p.enqueue(tx.sign(*pv));
    }
    let mut account_quota_limit = AccountGasLimit::new();
    // set block_quota_limit default
    let block_quota_limit = 61415926;
    // height should less than valid_until_block
    let height = 0;
    // set account_quota_limit be equal as block_quota_limit
    account_quota_limit.set_common_quota_limit(block_quota_limit);
    account_quota_limit.set_specific_quota_limit(HashMap::new());

    let txs = p.package(height, block_quota_limit, account_quota_limit.clone());
    p.update(&txs);
    let sys_time = SystemTime::now();
    let diff = sys_time
        .duration_since(start)
        .expect("SystemTime::duration_since failed");
    println!("pass");
    println!(
        "test {:20} ... bench: {}.{} s/iter",
        "bench_update",
        diff.as_secs(),
        diff.subsec_nanos()
    );
    b.iter(|| {});
}
