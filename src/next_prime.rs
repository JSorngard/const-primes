use crate::is_prime;

/// Returns the largest prime smaller than or equal to `n` if there is one.
///
/// Scans for primes downwards from the input with [`is_prime`].
///
/// # Examples
/// ```
/// # use const_primes::largest_prime_less_than_or_equal_to;
/// const LPLEQ: Option<u64> = largest_prime_less_than_or_equal_to(400);
/// assert_eq!(LPLEQ, Some(397));
/// ```
/// There's no prime smaller than or equal to one
/// ```
/// # use const_primes::largest_prime_less_than_or_equal_to;
/// const NOSUCH: Option<u64> = largest_prime_less_than_or_equal_to(1);
/// assert!(NOSUCH.is_none());
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn largest_prime_less_than_or_equal_to(mut n: u64) -> Option<u64> {
    if n == 0 || n == 1 {
        None
    } else if n == 2 {
        Some(2)
    } else {
        if n % 2 == 0 {
            n -= 1;
        }

        while !is_prime(n) {
            n -= 2;
        }

        Some(n)
    }
}

/// Returns the smallest prime greater than or equal to `n` if there is one that
/// can be represented by a `u64`.
///
/// Scans for primes upwards from the input with [`is_prime`].
///
/// # Example
/// ```
/// # use const_primes::smallest_prime_greater_than_or_equal_to;
/// const SPGEQ: Option<u64> = smallest_prime_greater_than_or_equal_to(400);
/// assert_eq!(SPGEQ, Some(401));
/// ```
/// Primes larger than 18446744073709551557 can not be represented by a `u64`
/// ```
/// # use const_primes::smallest_prime_greater_than_or_equal_to;
/// const NOSUCH: Option<u64> = smallest_prime_greater_than_or_equal_to(18_446_744_073_709_551_558);
/// assert!(NOSUCH.is_none());
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn smallest_prime_greater_than_or_equal_to(mut n: u64) -> Option<u64> {
    // The largest prime smaller than 2^64
    if n > 18_446_744_073_709_551_557 {
        None
    } else if n <= 2 {
        Some(2)
    } else {
        if n % 2 == 0 {
            n += 1;
        }

        while !is_prime(n) {
            n += 2;
        }

        Some(n)
    }
}
