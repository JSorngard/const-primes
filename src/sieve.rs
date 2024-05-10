//! This module contains implementations of prime sieves.

use core::fmt;

use crate::isqrt;

/// Uses the primalities of the first `N` integers in `base_sieve` to sieve the numbers in the range `[upper_limit - N, upper_limit)`.
/// Assumes that the base sieve contains the prime status of the `N` fist integers. The output is only meaningful
/// for the numbers below `N^2`. Fails to compile if `N` is 0.
///
/// # Panics
///
/// Panics if `N` is 0.
#[must_use = "the function only returns a new value and does not modify its inputs"]
pub(crate) const fn sieve_segment<const N: usize>(
    base_sieve: &[bool; N],
    upper_limit: u64,
) -> [bool; N] {
    assert!(N > 0, "`N` must be at least 1");

    let mut segment_sieve = [true; N];

    let lower_limit = upper_limit - N as u64;

    if lower_limit == 0 && N > 1 {
        // If the lower limit is 0 we can just return the base sieve.
        return *base_sieve;
    } else if lower_limit == 1 && N > 0 {
        // In case 1 is included in the upper sieve we need to treat it as a special case
        // since it's not a multiple of any prime in `base_sieve` even though it's not prime.
        segment_sieve[0] = false;
    }

    let mut i = 0;
    while i < N {
        if base_sieve[i] {
            let prime = i as u64;

            // Find the smallest multiple of the prime larger than or equal to `lower_limit`.
            let mut composite = (lower_limit / prime) * prime;
            if composite < lower_limit {
                composite += prime;
            }
            if composite == prime {
                composite += prime;
            }

            // Sieve all numbers in the segment that are multiples of the prime.
            while composite < upper_limit {
                segment_sieve[(composite - lower_limit) as usize] = false;
                composite += prime;
            }
        }
        i += 1;
    }

    segment_sieve
}

/// Returns an array of size `N` that indicates which of the `N` largest integers smaller than `upper_limit` are prime.
///
/// Uses a sieve of size `MEM` during evaluation, but stores only the requested values in the output array.
/// `MEM` must be large enough for the sieve to be able to determine the prime status of all numbers in the requested range,
/// that is: `MEM`^2 must be at least as large as `upper_limit`.
///
/// If you just want the prime status of the first `N` integers, see [`sieve`], and if you want the prime status of
/// the integers above some number, see [`sieve_geq`].
///
/// If you do not wish to compute the size requirement of the sieve manually, take a look at [`sieve_segment!`](crate::sieve_segment).
///
/// # Examples
///
/// Basic usage
/// ```
/// # use const_primes::sieve_lt;
/// // The five largest numbers smaller than 30 are 25, 26, 27, 28 and 29.
/// const N: usize = 5;
/// const LIMIT: u64 = 30;
/// // We thus need a memory size of at least 6, since 5*5 < 29, and therefore isn't enough.
/// const MEM: usize = 6;
/// const PRIME_STATUSES: [bool; N] = match sieve_lt::<N, MEM>(LIMIT) {Ok(s) => s, Err(_) => panic!()};
///
/// assert_eq!(
///     PRIME_STATUSES,
/// //   25     26     27     28     29
///     [false, false, false, false, true],
/// );
/// ```
/// Sieve limited ranges of large values. Functions provided by the crate can help you
/// compute the needed sieve size:
/// ```
/// # use const_primes::{sieve_lt, SieveError};
/// use const_primes::isqrt;
/// const N: usize = 3;
/// const LIMIT: u64 = 5_000_000_031;
/// const MEM: usize = isqrt(LIMIT) as usize + 1;
/// const BIG_PRIME_STATUSES: Result<[bool; N], SieveError> = sieve_lt::<N, MEM>(LIMIT);
/// assert_eq!(
///     BIG_PRIME_STATUSES,
/// //      5_000_000_028  5_000_000_029  5_000_000_030
///     Ok([false,         true,          false]),
/// );
/// ```
///
/// # Errors
///
/// Returns an error if `upper_limit` is larger than `MEM`^2:
/// ```
/// # use const_primes::{sieve_lt, SieveError};
/// const PS: Result<[bool; 5], SieveError> = sieve_lt::<5, 5>(26);
/// assert_eq!(PS, Err(SieveError::TooSmallSieveSize));
/// ```
/// or smaller than `N`:
/// ```
/// # use const_primes::{sieve_lt, SieveError};
/// const PS: Result<[bool; 5], SieveError> = sieve_lt::<5, 5>(4);
/// assert_eq!(PS, Err(SieveError::TooSmallLimit));
/// ```
///
/// # Panics
///
/// Panics if `N` is 0, if `MEM` is smaller than `N`, or if `MEM`^2 does not fit in a `u64`.
/// In const contexts this is a compile error.
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn sieve_lt<const N: usize, const MEM: usize>(
    upper_limit: u64,
) -> Result<[bool; N], SieveError> {
    assert!(N > 0, "`N` must be at least 1");
    assert!(MEM >= N, "`MEM` must be at least as large as `N`");

    let mem64 = MEM as u64;

    let Some(mem_sqr) = mem64.checked_mul(mem64) else {
        panic!("`MEM`^2 must fit in a `u64`");
    };

    if upper_limit > mem_sqr {
        return Err(SieveError::TooSmallSieveSize);
    }

    let n64 = N as u64;

    if upper_limit < n64 {
        return Err(SieveError::TooSmallLimit);
    }

    if upper_limit == n64 {
        // If we are not interested in sieving a larger range we can just return early.
        return Ok(sieve());
    }

    // Use a normal sieve of Eratosthenes for the first N numbers.
    let base_sieve: [bool; MEM] = sieve();

    // Use the result to sieve the higher range.
    let upper_sieve = sieve_segment(&base_sieve, upper_limit);

    let mut ans = [false; N];
    let mut i = 0;
    while i < N {
        ans[N - 1 - i] = upper_sieve[MEM - 1 - i];
        i += 1;
    }
    Ok(ans)
}

/// Returns an array of size `N` where the value at a given index indicates whether the index is prime.
///
/// # Example
///
/// ```
/// # use const_primes::sieve;
/// const PRIMALITY: [bool; 10] = sieve();
/// //                     0      1      2     3     4      5     6      7     8      9
/// assert_eq!(PRIMALITY, [false, false, true, true, false, true, false, true, false, false]);
/// ```
///
/// # Panics
///
/// Panics if `N` is 0. In const contexts this is a compile error:
/// ```compile_fail
/// # use const_primes::sieve;
/// const PRIMALITY: [bool; 0] = sieve();
/// ```
#[must_use = "the function only returns a new value"]
pub const fn sieve<const N: usize>() -> [bool; N] {
    assert!(N > 0, "`N` must be at least 1");

    let mut sieve = [true; N];
    if N > 0 {
        sieve[0] = false;
    }
    if N > 1 {
        sieve[1] = false;
    }

    let mut number: usize = 2;
    let bound = isqrt(N as u64);
    // For all numbers up to and including sqrt(n):
    while (number as u64) <= bound {
        if sieve[number] {
            // If a number is prime we enumerate all multiples of it
            // starting from its square,
            let Some(mut composite) = number.checked_mul(number) else {
                break;
            };

            // and mark them as not prime.
            while composite < N {
                sieve[composite] = false;
                composite = match composite.checked_add(number) {
                    Some(sum) => sum,
                    None => break,
                };
            }
        }
        number += 1;
    }

    sieve
}

/// The error returned by [`sieve_lt`] and [`sieve_geq`] if the input
/// is invalid or does not work to sieve the requested range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SieveError {
    /// The limit was less than or equal to `N` (for `sieve_lt`).
    TooSmallLimit,
    /// `MEM`^2 was smaller than the largest encountered value.
    TooSmallSieveSize,
    /// `limit + MEM` did not fit in a `u64`.
    TotalDoesntFitU64,
}

impl fmt::Display for SieveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooSmallLimit => write!(f, "`limit` must be at least `N`"),
            Self::TooSmallSieveSize => {
                write!(f, "`MEM`^2 was smaller than the largest encountered value")
            }
            Self::TotalDoesntFitU64 => write!(f, "`MEM + limit` must fit in a `u64`"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SieveError {}

/// Returns an array of size N that indicates which of the `N` integers greater than or equal to `lower_limit` are prime.
///
/// Uses a sieve of size `MEM` during evaluation, but stores only the requested values in the output array.
/// `MEM` must be large enough for the sieve to be able to determine the prime status of all numbers in the requested range,
/// that is `MEM`^2 must be larger than `lower_limit + N`.
///
/// If you just want the prime status of the first N integers, see [`sieve`], and if you want the
/// prime status of the integers below some number, see [`sieve_lt`].
///
/// If you do not wish to compute the size requirement of the sieve manually, take a look at [`sieve_segment!`](crate::sieve_segment).
///
/// # Examples
///
/// The size of the sieve, `MEM`, must be large enough for the largest sieved number to be smaller than `MEM`^2.
/// ```
/// # use const_primes::sieve_geq;
/// // The three numbers larger than or equal to 9 are 9, 10 and 11.
/// const N: usize = 3;
/// const LIMIT: u64 = 9;
/// // We thus need a memory size of at least 4, since 3*3 < 11, and therefore isn't enough.
/// const MEM: usize = 4;
/// const PRIME_STATUS: [bool; N] = match sieve_geq::<N, MEM>(LIMIT) {Ok(s) => s, Err(_) => panic!()};
/// //                        9,     10,    11
/// assert_eq!(PRIME_STATUS, [false, false, true]);
/// ```
/// Sieve limited ranges of large values. Functions provided by the crate can help you
/// compute the needed sieve size:
/// ```
/// # use const_primes::{sieve_geq, SieveError};
/// use const_primes::isqrt;
/// const N: usize = 3;
/// const LIMIT: u64 = 5_000_000_038;
/// const MEM: usize = isqrt(LIMIT) as usize + 1 + N;
/// const BIG_PRIME_STATUS: Result<[bool; N], SieveError> = sieve_geq::<N, MEM>(LIMIT);
/// //                               5_000_000_038  5_000_000_039  5_000_000_040
/// assert_eq!(BIG_PRIME_STATUS, Ok([false,         true,          false]));
/// ```
///
/// # Errors
///
/// Returns an error if `MEM + lower_limit` is larger than `MEM^2` or doesn't fit in a `u64`.
/// ```
/// # use const_primes::{sieve_geq, SieveError};
/// const P1: Result<[bool; 5], SieveError> = sieve_geq::<5, 5>(21);
/// const P2: Result<[bool; 5], SieveError> = sieve_geq::<5, 5>(u64::MAX);
/// assert_eq!(P1, Err(SieveError::TooSmallSieveSize));
/// assert_eq!(P2, Err(SieveError::TotalDoesntFitU64));
/// ```
///
/// # Panics
///
/// Panics if `N` is 0, if `MEM` is smaller than `N`, or if `MEM`^2 does not fit in a `u64`.
/// In const contexts this is a compile error.
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn sieve_geq<const N: usize, const MEM: usize>(
    lower_limit: u64,
) -> Result<[bool; N], SieveError> {
    assert!(N > 0, "`N` must be at least 1");
    assert!(MEM >= N, "`MEM` must be at least as large as `N`");

    let mem64 = MEM as u64;

    let Some(mem_sqr) = mem64.checked_mul(mem64) else {
        panic!("`MEM`^2 must fit in a `u64`");
    };

    let Some(upper_limit) = mem64.checked_add(lower_limit) else {
        return Err(SieveError::TotalDoesntFitU64);
    };

    if upper_limit > mem_sqr {
        return Err(SieveError::TooSmallSieveSize);
    }

    // If `lower_limit` is zero then this is the same as just calling `sieve`, and we can return early.
    if lower_limit == 0 {
        // We do not merge it with the computation of `base_sieve` below, since here we only
        // compute `N` values instead of `MEM`.
        return Ok(sieve());
    }

    let base_sieve: [bool; MEM] = sieve();

    let upper_sieve = sieve_segment(&base_sieve, upper_limit);

    let mut ans = [false; N];
    let mut i = 0;
    while i < N {
        ans[i] = upper_sieve[i];
        i += 1;
    }
    Ok(ans)
}

/// Call [`sieve_geq`] or [`sieve_lt`], and automatically compute the memory requirement of the sieve.
///
/// Computes the value of the const generic `MEM` as `isqrt(upper_limit) + 1` for [`sieve_lt`]
/// and as `isqrt(lower_limit) + 1 + N` for [`sieve_geq`].
/// This may overestimate the memory requirement for `sieve_geq`.
///
/// # Examples
///
/// ```
/// # use const_primes::{sieve_segment, SieveError};
/// const PRIME_STATUS_LT: Result<[bool; 5], SieveError> = sieve_segment!(5; < 100_005);
/// const PRIME_STATUS_GEQ: Result<[bool; 7], SieveError> = sieve_segment!(7; >= 615);
/// assert_eq!(
///     PRIME_STATUS_LT,
/// //      100_000  100_101  100_102  100_103  100_104
///     Ok([false,   false,   false,   true,    false])
/// );
/// assert_eq!(
///     PRIME_STATUS_GEQ,
/// //      615    616    617   618    619   620    621
///     Ok([false, false, true, false, true, false, false])
/// );
/// ```
///
/// # Errors and panics
///
/// Has the same error and panic behaviour as [`sieve_geq`] and [`sieve_lt`], with the exception
/// that it sets `MEM` such that the functions don't panic or run out of memory.
#[macro_export]
macro_rules! sieve_segment {
    ($n:expr; < $lim:expr) => {
        $crate::sieve_lt::<
            { $n },
            {
                let mem = { $lim };
                $crate::isqrt(mem) as ::core::primitive::usize + 1
            },
        >({ $lim })
    };
    ($n:expr; >= $lim:expr) => {
        $crate::sieve_geq::<
            { $n },
            {
                let mem = { $lim };
                $crate::isqrt(mem) as ::core::primitive::usize + 1 + { $n }
            },
        >({ $lim })
    };
}

#[cfg(test)]
mod test {
    use super::{sieve, sieve_segment};

    #[test]
    fn test_consistency_of_sieve_segment() {
        const P: [bool; 10] = sieve_segment(&sieve(), 10);
        const PP: [bool; 10] = sieve_segment(&sieve(), 11);
        assert_eq!(P, sieve());
        assert_eq!(PP, sieve::<11>()[1..]);
    }
}
