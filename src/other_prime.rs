use crate::is_prime;

/// Returns the largest prime smaller than `n` if there is one.
///
/// Scans for primes downwards from the input with [`is_prime`].
///
/// # Examples
/// Basic usage:
/// ```
/// # use const_primes::previous_prime;
/// const LPLEQ: Option<u64> = previous_prime(400);
/// assert_eq!(LPLEQ, Some(397));
/// ```
/// There's no prime smaller than two:
/// ```
/// # use const_primes::previous_prime;
/// const NOSUCH: Option<u64> = previous_prime(2);
/// assert!(NOSUCH.is_none());
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn previous_prime(mut n: u64) -> Option<u64> {
    if n <= 2 {
        None
    } else if n == 3 {
        Some(2)
    } else {
        n -= 1;

        if n % 2 == 0 {
            n -= 1;
        }

        while !is_prime(n) {
            n -= 2;
        }

        Some(n)
    }
}

/// Returns the smallest prime greater than `n` if there is one that
/// can be represented by a `u64`.
///
/// Scans for primes upwards from the input with [`is_prime`].
///
/// # Example
/// Basic usage:
/// ```
/// # use const_primes::next_prime;
/// const SPGEQ: Option<u64> = next_prime(400);
/// assert_eq!(SPGEQ, Some(401));
/// ```
/// Primes larger than 18446744073709551557 can not be represented by a `u64`:
/// ```
/// # use const_primes::next_prime;
/// const NOSUCH: Option<u64> = next_prime(18_446_744_073_709_551_557);
/// assert!(NOSUCH.is_none());
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn next_prime(mut n: u64) -> Option<u64> {
    // The largest prime smaller than u64::MAX
    if n >= 18_446_744_073_709_551_557 {
        None
    } else if n <= 1 {
        Some(2)
    } else {
        n += 1;

        if n % 2 == 0 {
            n += 1;
        }

        while !is_prime(n) {
            n += 2;
        }

        Some(n)
    }
}
