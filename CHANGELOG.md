# Changelog

This file contains the changes to the crate since version 0.4.8.

## 0.9.0

### Breaking changes

- Removed the `const_assert` feature, its functionality is now always enabled.
- Removed the `std` feature, the crate now uses the `Error` trait
 from `core`. The crate is thus always `no_std` compatible.
 If the `serde` feature is enabled the crate uses the [`serde_arrays`](https://crates.io/crates/serde_arrays)
 crate to serialize the type, and that crate in turn uses the standard library.
- Removed the implementations of `PartialEq`, `Eq`, `PartialOrd`, and `Ord` from
 `Primes`. To perform comparisons of the numbers in the struct with arrays or
 slices you can call `as_array` or `as_slice`. The reason for the removal is that
 every instance of `Primes<N>` for a given `N` is the same, which means that
 comparisons and orderings are pointless, since `N` is part of the type signature.

These changes mean that the MSRV of the crate is increased from 1.67.1 to 1.81.0.

### Other changes

- Added the `zerocopy` feature that derives the `AsBytes` trait from the [`zerocopy`](https://crates.io/crates/zerocopy)
 crate for the `Primes<N>` struct.
- Added the `rkyv` feature that derives the `Serialize`, `Deserialize`, and `Archive`
 traits from the [`rkyv`](https://crates.io/crates/rkyv) crate for the `Primes<N>`
 struct.

## 0.8.7

- Sped up `is_prime` by checking fewer witnesses in the Miller-Rabin test.

## 0.8.6

- Fixed a bug where the crate would try to sieve numbers below zero for some
 inputs to `sieve_lt` and `primes_lt` and panic.

## 0.8.5

- Added the `const_assert` feature that promotes all panics that involve only
 const generics into compile errors.

## 0.8.4

- License and docs.rs link improvements in README.
- Minor documentation improvements.

## 0.8.3

- Added the `totient` function to the `Primes` struct.
- Derived `Serialize` and `Deserialize` for the error types.
- Derived `Hash` for `SieveError`.

## 0.8.2

- Added the `serde` feature that derives the `Serialize` and `Deserialize`
 traits from `serde` for the `Primes` struct.

## 0.8.1

- Added a crate feature flag badge to the docs.
- Mention what can be done with the crate clearer in the description.

## 0.8.0

### Breaking changes

- Changed `Primes<N>::binary_search` to have the same API as `slice::binary_search`.

## 0.7.4

- Corrected wrong doclink to `remainder` in docstring of `Primes<N>::prime_factors`.

## 0.7.3

- Added `Primes<N>::prime_factors` function that returns an iterator over the
 prime factors of the given number (and not their multiplicities).

## 0.7.2

- Added `Primes<N>::prime_factorization` function that returns an iterator over
 the prime factors of the given number and their multiplicities.

## 0.7.1

- Organized the examples in the readme and top level crate documentation in a
 clearer way.

## 0.7.0

### Breaking changes

- `PrimesIter` no longer takes a const generic.

## 0.6.2

- Minor documentation tweaks.

## 0.6.1

- Corrected MSRV to 1.67.1.

## 0.6.0

### Breaking changes

- Made the `Primes<N>::iter` function and the `IntoIterator` implementation for
 `Primes<N>` return custom iterator types. Allows less disruptive refactoring in
  the future.

### Other changes

- Removed panics in functions (like `primes`) when `N` is zero. It results in an
 empty array, but may be what you want.

## 0.5.1

- Implemented `IntoIterator` for `&Primes<N>`.

## 0.5.0

This version focuses on adding support for generating primes and sieving numbers
in arbitrary ranges, instead of always having to start from 0.
It also shortens and clarifies some function names.

### Breaking changes

- Renamed `are_prime` to `sieve`.  
- Renamed `are_prime_below` to `sieve_lt`.  
- Changed function signature of `sieve_lt`.  
- Renamed `largest_prime_leq` to `previous_prime`.  
- Renamed `smallest_prime_lt` to `next_prime`.  
- Renamed `prime_counts` to `count_primes`.  
- Removed `moebius`, as it is out of scope of this crate. If you want the source
 code for that function it can be found on [Rosettacode](https://rosettacode.org/wiki/M%C3%B6bius_function#Rust),
 or in older versions of this crate.

## New features

- Added `primes_geq`, `primes_lt`, and `sieve_geq` functions to work with
 arbitrary ranges. They take two const generics, the number of values to return
 and the size of the sieve used during evaluation.  
- Added `primes_segment!` and `sieve_segment!` macros to simplify usage of the
 above functions. These macros compute the size of the sieve that the above
 functions need. Due to restrictions on const arithmetic this can not be done
 inside the functions.
- Added `isqrt` function. This can be useful if you wish to compute the size of
 the sieve yourself.

## Minor changes

- Speed up `PRIMES::count_primes_leq` by using a binary instead of linear search.
- Various documentation improvements.
