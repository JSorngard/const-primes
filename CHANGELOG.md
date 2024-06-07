This file contains the changes to the crate since version 0.4.8.

# 0.8.2

 - Add the `serde` feature that derives the `Serialize` and `Deserialize` traits from `serde` for the `Primes` struct.

# 0.8.1

 - Added a crate feature flag badge to the docs.
 - Mention what can be done with the crate clearer in the description.


# 0.8.0

## Breaking changes

 - Change `Primes<N>::binary_search` to have the same API as `slice::binary_search`.

# 0.7.4

 - Correct wrong doclink to `remainder` in docstring of `Primes<N>::prime_factors`.

# 0.7.3

 - Add `Primes<N>::prime_factors` function that returns an iterator over the prime factors of the given number (and not their multiplicities).

# 0.7.2

 - Add `Primes<N>::prime_factorization` function that returns an iterator over the prime factors of the given number and their multiplicities.

# 0.7.1

 - Organize the examples in the readme and top level crate documentation in a clearer way.

# 0.7.0

## Breaking changes

 - `PrimesIter` no longer takes a const generic.

# 0.6.2

 - Minor documentation tweaks.

# 0.6.1

 - Corrected MSRV to 1.67.1.

# 0.6.0

## Breaking changes

 - Make the `Primes<N>::iter` function and the `IntoIterator` implementation for `Primes<N>` return custom iterator types. Allows less disruptive refactoring in the future.

## Other changes

 - Remove panics in functions (like `primes`) when `N` is zero. It results in an empty array, but may be what you want.

# 0.5.1

 - Implement `IntoIterator` for `&Primes<N>`.

# 0.5.0

This version focuses on adding support for generating primes and sieving numbers in arbitrary ranges, instead of always having to start from 0.  
It also shortens and clarifies some function names.

## Breaking changes

 - Rename `are_prime` to `sieve`.  
 - Rename `are_prime_below` to `sieve_lt`.  
 - Change function signature of `sieve_lt`.  
 - Rename `largest_prime_leq` to `previous_prime`.  
 - Rename `smallest_prime_lt` to `next_prime`.  
 - Rename `prime_counts` to `count_primes`.  
 - Remove `moebius`, as it is out of scope of this crate. If you want the source code for that function it can be found on [Rosettacode](https://rosettacode.org/wiki/M%C3%B6bius_function#Rust), or in older versions of this crate.

## New features

 - Add `primes_geq`, `primes_lt`, and `sieve_geq` functions to work with arbitrary ranges. They take two const generics, the number of values to return and the size of the sieve used during evaluation.  
 - Add `primes_segment!` and `sieve_segment!` macros to simplify usage of the above functions. These macros compute the size of the sieve that the above functions need. Due to restrictions on const arithmetic this can not be done inside the functions.  
 - Add `isqrt` function. This can be useful if you wish to compute the size of the sieve yourself.  

## Minor changes

 - Speed up `PRIMES::count_primes_leq` by using a binary instead of linear search.  
 - Various documentation improvements.
