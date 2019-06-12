#[macro_use]
extern crate criterion;

use cita_common_benches::bench_tools;
use criterion::Criterion;
use std::sync::Arc;

const BLAKE2BKEY: &str = "CryptapeCryptape";
const KIB_UNIT: usize = 1024;

fn bench_keccak256(data: &[u8]) {
    let mut result = [0u8; 32];
    tiny_keccak::Keccak::keccak256(data, &mut result);
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
