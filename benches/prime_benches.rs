use const_primes::{are_prime, is_prime, primes};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn benchmarks(c: &mut Criterion) {
    c.bench_function("generate 10000 primes", |b| {
        b.iter(|| black_box(primes::<10_000>()))
    });
    c.bench_function("is_prime(1000000)", |b| {
        b.iter(|| black_box(is_prime(1_000_000)))
    });
    c.bench_function("sieve 10000 integers", |b| {
        b.iter(|| black_box(are_prime::<10_000>()))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
