// Copyright 2025 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::sieve;

/// Returns an array of size `N` where the value at a given index is how many primes are less than or equal to the index.
///
/// Sieves primes with [`sieve`](crate::sieve()) and then counts them.
///
/// # Example
///
/// Basic usage
///
/// ```
/// # use const_primes::prime_pi;
/// const COUNTS: [usize; 10] = prime_pi();
/// assert_eq!(COUNTS, [0, 0, 1, 2, 2, 3, 3, 4, 4, 4]);
/// ```
#[must_use = "the function only returns a new value"]
pub const fn prime_pi<const N: usize>() -> [usize; N] {
    let mut counts = [0; N];
    if N <= 2 {
        return counts;
    }
    counts[2] = 1;
    let prime_status: [bool; N] = sieve();
    let mut count = 1;
    let mut i = 3;
    while i < N {
        if prime_status[i] {
            count += 1;
        }
        counts[i] = count;
        if i + 1 < N {
            counts[i + 1] = count;
        }
        i += 2;
    }
    counts
}
