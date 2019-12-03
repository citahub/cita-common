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

#[macro_use]
extern crate criterion;

use cita_common_benches::bench_tools;
use criterion::Criterion;
use std::sync::Arc;

const BLAKE2BKEY: &str = "RivtowerRivtower";
const KIB_UNIT: usize = 1024;

fn bench_keccak256(data: &[u8]) {
    let mut result = [0u8; 32];
    let mut hasher = tiny_keccak::Keccak::v256();
    hasher.update(&data);
    hasher.finalize(&mut result);
}

fn bench_blake2b(data: &[u8]) {
    let mut result = [0u8; 32];
    unsafe {
        blake2b::blake2b(
            result.as_mut_ptr(),
            result.len(),
            data.as_ptr(),
            data.len(),
            BLAKE2BKEY.as_bytes().as_ptr(),
            BLAKE2BKEY.len(),
        );
    }
}

fn bench_sm3(data: &[u8]) {
    let mut result = [0u8; 32];
    result.copy_from_slice(libsm::sm3::hash::Sm3Hash::new(data).get_hash().as_ref());
}

fn benchmark_hash(c: &mut Criterion) {
    for kib in [1, 64, 1024].into_iter() {
        let data = Arc::new(bench_tools::random_bytes(kib * KIB_UNIT));
        let tests: Vec<(_, fn(&[u8]))> = vec![
            ("keccak256", bench_keccak256),
            ("blake2b", bench_blake2b),
            ("sm3", bench_sm3),
        ];
        for (name, func) in tests.into_iter() {
            let data = Arc::clone(&data);
            c.bench_function(format!("{} {} KiB", name, kib).as_ref(), move |b| {
                b.iter(|| func(&data))
            });
        }
    }
}

criterion_group!(benches, benchmark_hash);
criterion_main!(benches);
