//! This module contains an implementation of a deterministic Miller-Rabin primality test

use crate::imath::{mod_mul, mod_pow};

/// Returns whether `n` is prime.
///
/// Does trial division with a small wheel up to `log2(n)` and then uses a Miller-Rabin
/// primality test with a set of witnesses that
/// is known to be sufficient for numbers that fit in a `u64`.
///
/// # Example
/// Basic usage
/// ```
/// # use const_primes::is_prime;
/// const CHECK: bool = is_prime(18_446_744_073_709_551_557);
/// assert!(CHECK);
/// ```
#[must_use]
pub const fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 {
        true
    } else if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let mut candidate_factor = 5;
        let trial_limit = n.ilog2() as u64;
        while candidate_factor <= trial_limit {
            if n % candidate_factor == 0 || n % (candidate_factor + 2) == 0 {
                return false;
            }
            candidate_factor += 6;
        }

        // Find r such that n = 2^d * r + 1 for some r >= 1
        let mut d = n - 1;
        while d % 2 == 0 {
            d /= 2;
        }

        // Since we know the maximum size of the numbers we test against we can use the fact that there are known perfect bases
        // in order to make the test both fast and deterministic.
        // This list of witnesses was taken from https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Testing_against_small_sets_of_bases
        // and is sufficient for all numbers smaller than 2^64.
        const NUM_WITNESSES: usize = 12;
        const WITNESSES: [u64; NUM_WITNESSES] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

        let mut i = 0;
        while i < NUM_WITNESSES && WITNESSES[i] < n {
            if !miller_test(d, n, WITNESSES[i]) {
                return false;
            }
            i += 1;
        }

        true
    }
}

/// Performs a Miller-Rabin test with the witness k.
const fn miller_test(mut d: u64, n: u64, k: u64) -> bool {
    let mut x = mod_pow(k, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }

    while d != n - 1 {
        x = mod_mul(x, x, n);
        d *= 2;

        if x == 1 {
            return false;
        } else if x == n - 1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::is_prime;

    #[test]
    fn check_is_prime() {
        #[rustfmt::skip]
        const TEST_CASES: [bool; 100] = [false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false];
        for (x, ans) in TEST_CASES.into_iter().enumerate() {
            assert_eq!(is_prime(x as u64), ans);
        }
        assert!(is_prime(65521));
        assert!(is_prime(4294967291));
        assert!(is_prime(18446744073709551557));
    }
}
