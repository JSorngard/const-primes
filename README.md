[![Latest Version](https://img.shields.io/crates/v/const-primes.svg)](https://crates.io/crates/const-primes)

# const-primes

A crate for generating and working with prime numbers in const contexts.

`#![no_std]` compatible.

## Examples
Generate arrays of prime numbers with the function `primes` which uses a [segmented sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve).
```rust
const PRIMES: [u32; 10] = primes();
assert_eq!(PRIMES[5], 13);
assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
```
or with the type `Primes` which ensures that a non-zero number of primes are generated:
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
Creating a `Primes<0>` is a compile fail in const contexts and a panic otherwise.  

### Other functionality
Use `is_prime` to test whether a given number is prime
```rust
const CHECK: bool = is_prime(18_446_744_073_709_551_557);
assert!(CHECK);
```
or `are_prime` to compute the prime status of the `N` first integers,
```rust
const N: usize = 10;
const PRIME_STATUS: [bool; N] = are_prime();
//                        0      1      2     3     4      5     6      7     8      9
assert_eq!(PRIME_STATUS, [false, false, true, true, false, true, false, true, false, false]);
```
or `are_prime_below` to compute the prime status of the `N` largest integers below a given value,
```rust
const N: usize = 1001;
const BIG_PRIME_STATUS: [bool; N] = are_prime_below(1_000_005);
//                                   1_000_002       1_000_003      1_000_004
assert_eq!(BIG_PRIME_STATUS[998..], [false,          true,          false]);
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
