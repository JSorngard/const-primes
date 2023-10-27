use const_primes::{is_prime, primes, sieve};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::prelude::*;
use std::hint::black_box;

fn benchmarks(c: &mut Criterion) {
    c.bench_function("generate 10000 primes", |b| {
        b.iter(|| black_box(primes::<10_000>()))
    });

    c.bench_function("is_prime on random numbers", |b| {
        b.iter_batched(
            || (0..10_000).map(|_| random()).collect::<Vec<u64>>(),
            |data| {
                for number in data.iter() {
                    black_box(is_prime(*number));
                }
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("sieve 10000 integers", |b| {
        b.iter(|| black_box(sieve::<10_000>()))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
