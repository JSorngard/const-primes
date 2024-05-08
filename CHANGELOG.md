This file contains the changes to the crate since version 0.4.8.

# 0.5.0

This version focuses on adding support for generating primes and sieving numbers in arbitrary ranges, instead of always having to start from 0.

 - Add `primes_geq`, `primes_lt`, `sieve_geq`, and `sieve_lt` functions to work with arbitrary ranges.
 - Add `primes_segment!` and `sieve_segment!` macros to simplify usage of the above functions. These macros compute the size of the second const generic that the above functions need. Due to restrictions on const arithmetic this can not be done inside the functions.
 - Add `isqrt` function. This can be useful if you wish to compute the size of the second const generic yourself.
 - Speed up `PRIMES::count_primes_leq` by using a binary instead of linear search.
