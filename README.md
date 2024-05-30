[![Latest Version](https://img.shields.io/crates/v/const-primes.svg)](https://crates.io/crates/const-primes)
[![Build Status](https://github.com/JSorngard/const-primes/actions/workflows/rust.yml/badge.svg)](https://github.com/JSorngard/const-primes/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/JSorngard/const-primes/graph/badge.svg?token=KXBSRZ71Q0)](https://codecov.io/gh/JSorngard/const-primes)

# const-primes

A crate for generating and working with prime numbers in const contexts.  
This lets you for example pre-compute prime numbers at compile time and store them in the binary,
or check whether a number is prime in a const function.

`#![no_std]` compatible, and currently supports Rust versions 1.67.1 and newer.

## Example: generate primes at compile time

Generate arrays of prime numbers at compile time with the function `primes` which uses a [segmented sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve):
```rust
use const_primes::primes;

const PRIMES: [u32; 10] = primes();

assert_eq!(PRIMES[5], 13);
assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
```

## Example: use a cache of generated primes for related computations

The struct `Primes` is a wrapper around an array of primes:
```rust
use const_primes::Primes;

const PRIMES: Primes<10> = Primes::new();

assert_eq!(PRIMES[5], 13);
assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
```
and it can be used as a cache of primes for related computations:
```rust
const CACHE: Primes<100> = Primes::new();

// For primality testing
const CHECK_42: Option<bool> = CACHE.is_prime(42);
const CHECK_541: Option<bool> = CACHE.is_prime(541);
assert_eq!(CHECK_42, Some(false));
assert_eq!(CHECK_541, Some(true));

// Or for prime counting
const PRIMES_LEQ_100: Option<usize> = CACHE.count_primes_leq(100);
assert_eq!(PRIMES_LEQ_100, Some(25));

// Or for prime factorization:
assert_eq!(CACHE.prime_factorization(3072).collect(), &[(2, 10), (3, 1)])

// If questions are asked about numbers
// outside the cache it returns None
assert!(CACHE.is_prime(1000).is_none());
assert!(CACHE.count_primes_leq(1000).is_none());
```

## Example: primality checking

Use `is_prime` to test whether a given number is prime:
```rust
use const_primes::is_prime;

const CHECK: bool = is_prime(18_446_744_073_709_551_557);

assert!(CHECK);
```

## Example: sieving

Sieve a range of numbers for their prime status with `sieve`:
```rust
use const_primes::sieve;

const PRIME_STATUS: [bool; 10] = sieve();

//                        0      1      2     3     4      5     6      7     8      9
assert_eq!(PRIME_STATUS, [false, false, true, true, false, true, false, true, false, false]);
```  

## Example: generate the three primes after 5000000031

The crate also provides prime generation and sieving functions that can be used to work with ranges that don't start at zero, e.g. `primes_geq` and `sieve_lt`. These functions can use large sieves to compute large primes, but don't need to return the entire sieve, just the requested numbers.
They are most conveniently used through the macros `primes_segment!` and `sieve_segment!` that automatically compute the size of the sieve that's needed for a certain computation.

Compute 3 primes greater than or equal to 5000000031:
```rust
use const_primes::{primes_segment, GenerationError};

const N: usize = 3;
const PRIMES_GEQ: Result<[u64; N], GenerationError> = primes_segment!(N; >= 5_000_000_031);

assert_eq!(PRIMES_GEQ, Ok([5_000_000_039, 5_000_000_059, 5_000_000_063]));
```

## Example: determine the prime status of the three largest numbers less than 100005

```rust
use const_primes::{sieve_segment, SieveError};

const N: usize = 3;
const PRIME_STATUS_LT: Result<[bool; N], SieveError> = sieve_segment!(N; < 100_005);

//                              100_102  100_103  100_104
assert_eq!(PRIME_STATUS_LT, Ok([false,   true,    false]));
```

## Example: find the next or previous prime numbers

Find the next or previous prime numbers with `next_prime` and `previous_prime` if they exist and can be represented in a `u64`:
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

`std`: implements the `Error` trait from the standard library for the error types.

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
