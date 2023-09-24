//! This module contains the implementation of `is_prime` that uses [trial division](https://en.wikipedia.org/wiki/Trial_division).

use crate::{isqrt, Underlying};

/// Returns whether `n` is prime.
///
/// For a version of this function that uses the
/// [sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes),
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
