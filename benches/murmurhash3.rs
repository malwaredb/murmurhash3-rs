use criterion::{criterion_group, criterion_main, Criterion};

use rand::Rng;
use std::hint::black_box;
use std::iter::FromIterator;

use malwaredb_murmurhash3::{murmurhash3_x64_128, murmurhash3_x86_32};

fn run_bench(c: &mut Criterion, size: u64) {
    c.bench_function("murmurhash 256k", |b| {
        let mut data: Vec<u8> = FromIterator::from_iter((0..size).map(|_| 0u8));
        rand::rng().fill_bytes(&mut data);

        b.iter(|| {
            black_box(murmurhash3_x86_32(&data, 0));
        });
    });
}

fn run_bench128(c: &mut Criterion, size: u64) {
    c.bench_function("murmurhash128 256k", |b| {
        let mut data: Vec<u8> = FromIterator::from_iter((0..size).map(|_| 0u8));
        rand::rng().fill_bytes(&mut data);

        b.iter(|| {
            black_box(murmurhash3_x64_128(&data, 0));
        });
    });
}

fn bench_random_256k(c: &mut Criterion) {
    run_bench(c, 256 * 1024);
}

fn bench_random_16b(c: &mut Criterion) {
    run_bench(c, 16);
}

fn bench_random128_256k(c: &mut Criterion) {
    run_bench128(c, 256 * 1024);
}

fn bench_random128_16b(c: &mut Criterion) {
    run_bench128(c, 16);
}

criterion_group!(benches, bench_random_16b, bench_random_256k);
criterion_group!(benches128, bench_random128_16b, bench_random128_256k);
criterion_main!(benches, benches128);
