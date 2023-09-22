# const-primes

A crate for generating arrays of prime numbers at compile time.

`#![no_std]` compatible.

The crate is currently split into two modules, `trial` and `sieve`, which contain implementations of
prime related `const` functions that use [trial division](https://en.wikipedia.org/wiki/Trial_division)
or the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) in their implementations respectively.

The sieve needs `O(n)` memory, which means that the functions in the `sieve` module
need a const generic to be specified in order to compile.

The implementations in `trial` are slower, but do not need const generics unless they return an array.

## Examples
### Prime generation
Generate arrays of prime numbers with the function `trial::primes`
```rust
const PRIMES: [u32; 10] = primes();
assert_eq!(PRIMES[5], 13);
assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
```
or with the type `wrapper::Primes` which ensures that a non-zero number of primes are generated
```rust
const PRIMES: Primes<10> = Primes::new();
assert_eq!(PRIMES[5], 13);
assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
```
Creating a `Primes<0>` is a compile fail in const contexts and a panic otherwise.

### Primality testing
There is one implementation of `is_prime` in `trial` and one in `sieve`
```rust
use const_primes::{trial, sieve};
const CHECK_5: bool = trial::is_prime(5);
const CHECK_1009: bool = sieve::is_prime::<1009>();
assert!(CHECK_5);
assert!(CHECK_1009);
```
The `Primes` type also lets you reuse an array of already computed primes for primality testing.
```rust
use const_primes::wrapper::Primes;
const CACHE: Primes<100> = Primes::new();
const CHECK_42: Option<bool> = CACHE.is_prime(42);
const CHECK_541: Option<bool> = CACHE.is_prime(541);
const CHECK_1000: Option<bool> = CACHE.is_prime(1000);
assert_eq!(CHECK_42, Some(false));
assert_eq!(CHECK_541, Some(true));
assert_eq!(CHECK_1000, None);
```
The function `sieve::primalities` lets you compute the prime status of many integers
```rust
use const_primes::sieve::primalities;
const PRIME_STATUS: [bool; 10] = primalities();
assert_eq!(PRIME_STATUS, [false, false, true, true, false, true, false, true, false, false]);
```

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
