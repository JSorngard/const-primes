use const_primes::{is_prime, primes, primes_geq, primes_lt, sieve, sieve_geq, sieve_lt};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use rand::prelude::*;
use std::hint::black_box;

fn benchmarks(c: &mut Criterion) {
    {
        const N: usize = 10_000;
        let mut prime_generation = c.benchmark_group("prime generation");
        prime_generation.bench_function(format!("first {N} primes"), |b| {
            b.iter(|| black_box(primes::<N>()))
        });
        prime_generation.bench_function(format!("{N} primes < 100000000"), |b| {
            b.iter(|| black_box(primes_lt::<N>(100000000)))
        });
        prime_generation.bench_function(format!("{N} primes >= 99990000"), |b| {
            b.iter(|| black_box(primes_geq::<N>(99990000)))
        });
    }

    {
        const N: u64 = 10_000;
        let mut primality_testing = c.benchmark_group("primality testing");
        primality_testing.throughput(Throughput::Elements(N));
        primality_testing.bench_function(format!("is_prime on {N} random numbers"), |b| {
            b.iter_batched(
                || (0..N).map(|_| random()).collect::<Vec<u64>>(),
                |data| {
                    for number in data.iter() {
                        black_box(is_prime(*number));
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }

    {
        const N: usize = 10_000;
        let mut sieving = c.benchmark_group("prime sieving");
        sieving.bench_function(format!("first {N} integers"), |b| {
            b.iter(|| black_box(sieve::<N>()))
        });
        sieving.bench_function(format!("{N} integers < 100000000"), |b| {
            b.iter(|| black_box(sieve_lt::<N>(100000000)))
        });
        sieving.bench_function(format!("{N} integers >= 99990000"), |b| {
            b.iter(|| black_box(sieve_geq::<N>(99990000)))
        });
    }
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
