[![Latest Version](https://img.shields.io/crates/v/const-primes.svg)](https://crates.io/crates/const-primes)
[![Build Status](https://github.com/JSorngard/const-primes/actions/workflows/rust.yml/badge.svg)](https://github.com/JSorngard/const-primes/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/JSorngard/const-primes/graph/badge.svg?token=KXBSRZ71Q0)](https://codecov.io/gh/JSorngard/const-primes)

# const-primes

A crate for generating and working with prime numbers in const contexts.

`#![no_std]` compatible.

## Examples
Generate arrays of prime numbers with the function `primes` which uses a [segmented sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve):
```rust
const PRIMES: [u32; 10] = primes();
assert_eq!(PRIMES[5], 13);
assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
```
or with the wrapping type `Primes`:
```rust
const PRIMES: Primes<10> = Primes::new();
assert_eq!(PRIMES[5], 13);
assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
```
It also lets you reuse it as a cache of primes for related computations:
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

// If questions are asked about numbers
// outside the cache it returns None
assert!(CACHE.is_prime(1000).is_none());
assert!(CACHE.count_primes_leq(1000).is_none());
```
Sieve a range of numbers for their prime status with `sieve`:
```rust
const N: usize = 10;
const PRIME_STATUS: [bool; N] = sieve();
//                        0      1      2     3     4      5     6      7     8      9
assert_eq!(PRIME_STATUS, [false, false, true, true, false, true, false, true, false, false]);
```  

## Arbitrary ranges
The crate provides prime generation and sieving functions with suffixes, e.g. `primes_geq` and `sieve_lt`
that can be used to work with ranges that don't start at zero. They take two generics: the number of elements
to store in the binary and the size of the sieve used during evaluation. The sieve size must be the ceiling
of the square root of the largest encountered value:
```rust
//                              ceil(sqrt(5_000_000_063)) = 70_711
const PRIMES_GEQ: Result<[u64; 3], GenerationError> = primes_geq::<3, 70_711>(5_000_000_031);
assert_eq!(PRIMES_GEQ, Ok([5_000_000_039, 5_000_000_059, 5_000_000_063]));
```
```rust
const PRIME_STATUS_LT: Result<[bool; N], SieveError> = sieve_lt::<3, 70_711>(5_000_000_031);
//                              5_000_000_028  5_000_000_029  5_000_000_030
assert_eq!(PRIME_STATUS_LT, Ok([false,         true,          false]));
```
The sieve size can be computed by the crate by using the macro `primes_segment!` and `sieve_segment!`.
## Other functionality
Use `is_prime` to test whether a given number is prime:
```rust
const CHECK: bool = is_prime(18_446_744_073_709_551_557);
assert!(CHECK);
```
Find the next or previous prime numbers with `next_prime` and `previous_prime` if they exist:
```rust
const NEXT: Option<u64> = next_prime(25);
const PREV: Option<u64> = previous_prime(25);
const NOSUCH: Option<u64> = previous_prime(2);

assert_eq!(NEXT, Some(29));
assert_eq!(PREV, Some(23));
assert_eq!(NOSUCH, None);
```
and more!

## Features

`std`: derives the `Error` trait for the error types.  

## License

Licensed under either of

 * Apache License, Version 2.0
   [LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   [LICENSE-MIT](http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
