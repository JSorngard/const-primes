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
or with the wrapping type [`Primes`]:
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
that can be used to work with ranges that don't start at zero.
```rust
const N: usize = 70722;
const PRIMES_GEQ: [u64; N] = primes_geq(5_000_000_031);
assert_eq!(PRIMES_GEQ[..3], [5_000_000_039, 5_000_000_059, 5_000_000_063]);
```
```rust
const N: usize = 70711;
const PRIME_STATUS_LT: [bool; N] = sieve_lt(5_000_000_031);
//                                    5_000_000_028  5_000_000_029  5_000_000_030
assert_eq!(PRIME_STATUS_LT[N - 3..], [false,         true,          false]);
```
Unfortunately the output array must be large enough to contain the prime sieve, which scales with
the square root of largest relavant number, which is why the examples use a size of over 70000 even though
they're only interested in three numbers.
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
