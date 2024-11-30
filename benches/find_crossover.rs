use std::hint::black_box;

use const_primes::{is_prime, sieve_geq};
use criterion::{criterion_group, criterion_main, Criterion};

fn find_crossover(c: &mut Criterion) {
    // An N larger than this results in a stack overflow in sieve_geq
    const N: usize = 253099;

    let mut group = c.benchmark_group("crossover");

    for lower_limit in [1000000, 100000000, (N * N - N) as u64] {
        // sieve_geq always takes the same ammount of time for a given MEM.
        group.bench_function(format!("sieve_geq({lower_limit})"), |b| {
            b.iter(|| {
                black_box(sieve_geq::<N, N>(lower_limit)).unwrap();
            })
        });
        
        group.bench_function(
            format!("map_is_prime({lower_limit})"),
            |b| {
                b.iter(|| {
                    black_box(
                        (lower_limit..lower_limit + N as u64)
                            .map(is_prime)
                            .collect::<Vec<_>>(),
                    )
                });
            },
        );
    }
}

criterion_group!(crossover, find_crossover);
criterion_main!(crossover);
