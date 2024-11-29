//! This module contains implementations of functions that search for primes that neighbour a given number.

use crate::is_prime;

// Generalised function for nearest search by incrementing/decrementing by 1
// Any attempt at optimising this would be largely pointless since the largest prime gap under 2^64 is only 1550
// And is_prime's trial division already eliminates most of those
const fn bounded_search(mut n: u64, stride: u64) -> Option<u64> {
    loop {
        // Addition over Z/2^64, aka regular addition under optimisation flags
        n = n.wrapping_add(stride);
        // If either condition is met then we started either below or above the smallest or largest prime respectively
        // Any two values from 2^64-58 to 1 would also work
        if n == 0u64 || n == u64::MAX {
            return None;
        }

        if is_prime(n) {
            return Some(n);
        }
    }
}
/// Returns the largest prime smaller than `n` if there is one.
///
/// Scans for primes downwards from the input with [`is_prime`].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use const_primes::previous_prime;
/// const PREV: Option<u64> = previous_prime(400);
/// assert_eq!(PREV, Some(397));
/// ```
///
/// There's no prime smaller than two:
/// ```
///
/// # use const_primes::previous_prime;
/// const NO_SUCH: Option<u64> = previous_prime(2);
/// assert_eq!(NO_SUCH, None);
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn previous_prime(n: u64) -> Option<u64> {
    // Adding by 2^64-1 over Z/2^64 is equivalent to subtracting by 1
    bounded_search(n, u64::MAX)
}

/// Returns the smallest prime greater than `n` if there is one that
/// can be represented by a `u64`.
///
/// Scans for primes upwards from the input with [`is_prime`].
///
/// # Example
///
/// Basic usage:
///
/// ```
/// # use const_primes::next_prime;
/// const NEXT: Option<u64> = next_prime(400);
/// assert_eq!(NEXT, Some(401));
/// ```
///
/// Primes larger than 18446744073709551557 can not be represented by a `u64`:
/// ```
///
/// # use const_primes::next_prime;
/// const NO_SUCH: Option<u64> = next_prime(18_446_744_073_709_551_557);
/// assert_eq!(NO_SUCH, None);
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn next_prime(n: u64) -> Option<u64> {
    bounded_search(n, 1)
}
