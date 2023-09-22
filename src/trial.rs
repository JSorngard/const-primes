//! This module contains implementations of prime related functions that use [trial division](https://en.wikipedia.org/wiki/Trial_division).

use crate::{isqrt, Underlying};

/// Returns whether `n` is prime.
///
/// For a version of this function that uses the
/// [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes),
/// see [`sieve::is_prime`](crate::sieve::is_prime).
///
/// # Example
/// Basic usage
/// ```
/// # use const_primes::trial::is_prime;
/// const IS_101_A_PRIME: bool = is_prime(101);
/// assert!(IS_101_A_PRIME);
/// ```
#[must_use]
pub const fn is_prime(n: Underlying) -> bool {
    if n == 2 || n == 3 {
        true
    } else if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let mut candidate_factor = 5;
        let bound = isqrt(n) + 1;
        while candidate_factor < bound {
            if n % candidate_factor == 0 || n % (candidate_factor + 2) == 0 {
                return false;
            }
            candidate_factor += 6;
        }
        true
    }
}

/// Returns an array of the first `N` prime numbers.
///
/// # Example
/// ```
/// # use const_primes::trial::primes;
/// const PRIMES: [u32; 6] = primes();
/// assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13]);
/// ```
#[must_use]
pub const fn primes<const N: usize>() -> [Underlying; N] {
    let mut primes = [2; N];
    let mut number = 3;
    let mut i = 1;

    while i < N {
        let mut j = 0;
        let mut is_prime = true;
        let max_bound = isqrt(number) + 1;
        while primes[j] < max_bound {
            if number % primes[j] == 0 {
                is_prime = false;
                break;
            }
            j += 1;
        }
        if is_prime {
            primes[i] = number;
            i += 1;
        }
        number += 1;
    }
    primes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_is_prime() {
        #[rustfmt::skip]
        const TEST_CASES: [bool; 100] = [false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false];
        for (x, ans) in TEST_CASES.into_iter().enumerate() {
            assert_eq!(is_prime(x as Underlying), ans);
        }
    }
}
