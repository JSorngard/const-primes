# const-primes

[![Static Badge](https://img.shields.io/badge/github-JSorngard%2Fconst--primes-8da0cb?logo=github)](https://github.com/JSorngard/const-primes)
[![Crates.io Version](https://img.shields.io/crates/v/const_primes?logo=rust)](https://crates.io/crates/const-primes)
[![docs.rs (with version)](https://img.shields.io/docsrs/const-primes/latest?logo=docs.rs&label=docs.rs)](https://docs.rs/const-primes/latest/const_primes/)
[![Build Status](https://github.com/JSorngard/const-primes/actions/workflows/rust.yml/badge.svg)](https://github.com/JSorngard/const-primes/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/JSorngard/const-primes/graph/badge.svg?token=KXBSRZ71Q0)](https://codecov.io/gh/JSorngard/const-primes)

Generate and work with prime numbers in const contexts.

This crate lets you for example pre-compute prime numbers at compile time, store
them in the binary, and use them later for related computations,
or check whether a number is prime in a const function.

`no_std` compatible when the `serde` feature is disabled.

This version supports Rust versions 1.81.0 and up, while previous versions
support Rust versions 1.67.1 and up.

## Example: generate primes at compile time and use them for related computations

The struct `Primes` is a wrapper around an array of primes generated by a
[segmented sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve)
and can be used as a cache of prime numbers for related computations:

```rust
// The first 100 primes
const CACHE: Primes<100> = Primes::new();

// Primality testing
const CHECK_42: Option<bool> = CACHE.is_prime(42);
const CHECK_541: Option<bool> = CACHE.is_prime(541);
assert_eq!(CHECK_42, Some(false));
assert_eq!(CHECK_541, Some(true));

// Prime counting
const PRIMES_LEQ_100: Option<usize> = CACHE.count_primes_leq(100);
assert_eq!(PRIMES_LEQ_100, Some(25));

// Prime factorization:
assert_eq!(CACHE.prime_factorization(3072).collect(), &[(2, 10), (3, 1)])
// and more!

// If questions are asked about numbers
// outside the cache it returns None
assert!(CACHE.is_prime(1000).is_none());
assert!(CACHE.count_primes_leq(1000).is_none());
```

Want only the numbers? Use the `primes` function, or convert the cache into an array:

```rust
use const_primes::{primes, Primes};

const CACHE: Primes<10> = Primes::new();

const PRIMES_ARRAY1: [u32; 10] = primes();
const PRIMES_ARRAY2: [i32; 10] = PRIMES.into_array();

assert_eq!(PRIMES_ARRAY1, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
assert_eq!(PRIMES_ARRAY1, PRIMES_ARRAY2);
```

## Example: primality checking

Use `is_prime` to test whether a given number is prime:

```rust
use const_primes::is_prime;

const CHECK: bool = is_prime(18_446_744_073_709_551_557);

assert!(CHECK);
```

## Example: generate the three primes after 5000000031

The crate also provides prime generation and sieving functions that can be used
to work with ranges of large numbers that don't start at zero, e.g.
`primes_geq` and `sieve_lt`. These functions can use large sieves to compute
large primes, but don't need to return the entire sieve, just the requested numbers.
They are most conveniently used through the macros `primes_segment!` and
`sieve_segment!` that automatically compute the size of the sieve that's needed
for a certain computation.

Compute 3 primes greater than or equal to 5000000031:

```rust
use const_primes::{primes_segment, GenerationError};

const N: usize = 3;
const PRIMES_GEQ: Result<[u64; N], GenerationError> = primes_segment!(N; >= 5_000_000_031);

assert_eq!(PRIMES_GEQ, Ok([5_000_000_039, 5_000_000_059, 5_000_000_063]));
```

## Example: find the next or previous prime numbers

Find the next or previous prime numbers with `next_prime` and `previous_prime`
if they exist and can be represented in a `u64`:

```rust
use const_primes::{previous_prime, next_prime};

const NEXT: Option<u64> = next_prime(25);
const PREV: Option<u64> = previous_prime(25);
const NO_SUCH: Option<u64> = previous_prime(2);
const TOO_BIG: Option<u64> = next_prime(u64::MAX);

assert_eq!(NEXT, Some(29));
assert_eq!(PREV, Some(23));
assert_eq!(NO_SUCH, None);
assert_eq!(TOO_BIG, None);
```

and more!

## Features

`serde`: derives the `Serialize` and `Deserialize` traits from [`serde`](https://crates.io/crates/serde)
for the `Primes` struct, as well as a few others.
Uses the [`serde_arrays`](https://crates.io/crates/serde_arrays)
crate to do this, and that crate uses the standard library.

`zerocopy`: derives the `AsBytes` trait from [`zerocopy`](https://crates.io/crates/zerocopy)
for the `Primes` struct.

`rkyv`: derives the `Serialize`, `Deserialize`, and `Archive` traits from the
[`rkyv`](https://crates.io/crates/rkyv) crate for the [`Primes`] struct.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/JSorngard/const-primes/blob/main/LICENSE-APACHE)
 or <http://www.apache.org/licenses/LICENSE-2.0>).
- MIT license ([LICENSE-MIT](https://github.com/JSorngard/const-primes/blob/main/LICENSE-MIT)
 or <http://opensource.org/licenses/MIT>).

at your option.

## Contribution

Contributions are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
