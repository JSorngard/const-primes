use std::hint::black_box;

use const_primes::{is_prime, sieve_geq};
use criterion::{criterion_group, criterion_main, Criterion};

fn find_crossover(c: &mut Criterion) {
    const N_FAST: usize = 10;

    // A MEM larger than this risks a stack overflow in sieve_geq
    // sieve_geq always takes the same ammount of time for a given MEM.
    const MEM: usize = 250000;

    // As large as possible for the given MEM.
    const N_SLOW: usize = MEM;

    // The idea is now to do a binary search over N until we find the point where the fastest method switches for various values of MEM.
    
    // Start as high up as possible
    let lower_limit = (MEM * MEM - MEM) as u64;

    let mut fast_group = c.benchmark_group("fast");

    fast_group.bench_function(format!("sieve_geq({lower_limit})"), |b| {
        b.iter(|| {
            black_box(sieve_geq::<N_FAST, MEM>(lower_limit)).unwrap();
        })
    });

    fast_group.bench_function(format!("map_is_prime({lower_limit})"), |b| {
        b.iter(|| {
            black_box(
                (lower_limit..lower_limit + N_FAST as u64)
                    .map(is_prime)
                    .collect::<Vec<_>>(),
            )
        });
    });

    drop(fast_group);

    let mut slow_group = c.benchmark_group("slow");

    slow_group.bench_function(format!("sieve_geq({lower_limit})"), |b| {
        b.iter(|| {
            black_box(sieve_geq::<N_SLOW, MEM>(lower_limit)).unwrap();
        });
    });

    slow_group.bench_function(format!("map_is_prime({lower_limit})"), |b| {
        b.iter(|| {
            black_box(
                (lower_limit..lower_limit + N_SLOW as u64)
                    .map(is_prime)
                    .collect::<Vec<_>>(),
            )
        });
    });
}

criterion_group!(crossover, find_crossover);
criterion_main!(crossover);
