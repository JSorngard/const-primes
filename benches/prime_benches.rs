use const_primes::{is_prime, primes, primes_greater_than_or_equal_to, primes_less_than, sieve, sieve_less_than, sieve_greater_than_or_equal_to};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::prelude::*;
use std::hint::black_box;

fn benchmarks(c: &mut Criterion) {
    {
        let mut prime_generation = c.benchmark_group("prime generation");
        prime_generation.bench_function("first 10000 primes", |b| {
            b.iter(|| black_box(primes::<10_000>()))
        });
        prime_generation.bench_function("10000 primes < 100000000", |b| {
            b.iter(|| black_box(primes_less_than::<10000>(100000000)))
        });
        prime_generation.bench_function("10000 primes >= 99990000", |b| {
            b.iter(|| black_box(primes_greater_than_or_equal_to::<10000>(99990000)))
        });
    }

    c.bench_function("is_prime on 10000 random numbers", |b| {
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

    {
        let mut sieving = c.benchmark_group("prime sieving");
        sieving.bench_function("first 10000 integers", |b| {
            b.iter(|| black_box(sieve::<10_000>()))
        });
        sieving.bench_function("10000 integers < 100000000", |b| {
            b.iter(|| black_box(sieve_less_than::<10_000>(100000000)))
        });
        sieving.bench_function("10000 integers >= 99990000", |b| {
            b.iter(|| black_box(sieve_greater_than_or_equal_to::<10_000>(99990000)))
        });
    }
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
