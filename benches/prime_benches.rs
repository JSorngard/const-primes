use const_primes::{are_prime, is_prime, moebius, primes};
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
        b.iter(|| black_box(are_prime::<10_000>()))
    });

    let ints: Vec<_> = (1..1_000_000).map(|n| n).collect();
    c.bench_function("möbius of first 1e6 integers", |b| {
        b.iter(|| {
            for &i in &ints {
                black_box(moebius(i));
            }
        })
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
