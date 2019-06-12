#[macro_use]
extern crate criterion;

use cita_common_benches::bench_tools;
use criterion::Criterion;
use std::sync::Arc;

fn benchmark_sign(c: &mut Criterion) {
    let data = Arc::new(bench_tools::random_bytes(32));
    {
        let data = Arc::clone(&data);
        let context = secp256k1::Secp256k1::new();
        c.bench_function("secp256k1 sign", move |b| {
            let (sk, _) = context.generate_keypair(&mut rand::thread_rng());
            b.iter(|| {
                context
                    .sign_recoverable(&secp256k1::Message::from_slice(&data).unwrap(), &sk)
                    .serialize_compact()
            })
        });
    }
    {
        let data = Arc::clone(&data);
        c.bench_function("ed25519 sign", move |b| {
            let (_, sk) = sodiumoxide::crypto::sign::gen_keypair();
            b.iter(|| sodiumoxide::crypto::sign::sign_detached(&data, &sk))
        });
    }
    {
        let data = Arc::clone(&data);
        c.bench_function("sm2 sign", move |b| {
            let ctx = libsm::sm2::signature::SigCtx::new();
            let (pk, sk) = ctx.new_keypair();
            b.iter(|| ctx.sign(&data, &sk, &pk))
        });
    }
}

fn benchmark_verify(c: &mut Criterion) {
    let data = Arc::new(bench_tools::random_bytes(32));
    {
        let data = Arc::clone(&data);
        let context = secp256k1::Secp256k1::new();
        let (sk, pk) = context.generate_keypair(&mut rand::thread_rng());
        let sig_msg = secp256k1::Message::from_slice(&data).unwrap();
        let (rec_id, sig_data) = context.sign_recoverable(&sig_msg, &sk).serialize_compact();
        let sig = secp256k1::RecoverableSignature::from_compact(&sig_data, rec_id)
            .unwrap()
            .to_standard();
        c.bench_function("secp256k1 verify", move |b| {
            b.iter(|| context.verify(&sig_msg, &sig, &pk).unwrap())
        });
    }
    {
        let data = Arc::clone(&data);
        let (pk, sk) = sodiumoxide::crypto::sign::gen_keypair();
        let sig = sodiumoxide::crypto::sign::sign_detached(&data, &sk);
        c.bench_function("ed25519 verify", move |b| {
            b.iter(|| {
                if !sodiumoxide::crypto::sign::verify_detached(&sig, &data, &pk) {
                    panic!("ed25519 verify error");
                }
            })
        });
    }
    {
        let data = Arc::clone(&data);
        let ctx = libsm::sm2::signature::SigCtx::new();
        let (pk, sk) = ctx.new_keypair();
        let sig = ctx.sign(&data, &sk, &pk);
        c.bench_function("sm2 verify", move |b| {
            b.iter(|| {
                if !ctx.verify(&data, &pk, &sig) {
                    panic!("sm2 verify error");
                }
            })
        });
    }
}

criterion_group!(benches, benchmark_sign, benchmark_verify);
criterion_main!(benches);
