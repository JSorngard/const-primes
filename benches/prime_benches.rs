use criterion::{criterion_group, criterion_main, Criterion};
use const_primes::{primes, trial, sieve};
use std::hint::black_box;

fn sieve_primes(c: &mut Criterion) {
    c.bench_function("10 000 primes", |b| b.iter(|| black_box(primes::<10_000>())));
}

fn compare_is_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_prime");
    group.bench_function("sieve", |b| b.iter(|| black_box(sieve::is_prime::<1_000_000>())));
    group.bench_function("trial", |b| b.iter(|| black_box(trial::is_prime(1_000_000))));
}

criterion_group!(benches, sieve_primes, compare_is_primes);
criterion_main!(benches);