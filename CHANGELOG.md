This file contains the changes to the crate since version 0.4.8.

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

 - Add `primes_geq`, `primes_lt`, and `sieve_geq` functions to work with arbitrary ranges.  
 - Add `primes_segment!` and `sieve_segment!` macros to simplify usage of the above functions. These macros compute the size of the second const generic that the above functions need. Due to restrictions on const arithmetic this can not be done inside the functions.  
 - Add `isqrt` function. This can be useful if you wish to compute the size of the second const generic yourself.  

## Minor changes

 - Speed up `PRIMES::count_primes_leq` by using a binary instead of linear search.  
 - Various documentation improvements.
