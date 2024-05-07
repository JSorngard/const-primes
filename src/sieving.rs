use crate::isqrt;

/// Uses the primalities of the first `N` integers in `base_sieve` to sieve the numbers in the range `[upper_limit - N, upper_limit)`.
/// Assumes that the base sieve contains the prime status of the `N` fist integers. The output is only meaningful
/// for the numbers below `N^2`. Fails to compile if `N` is 0.
#[must_use = "the function only returns a new value and does not modify its inputs"]
pub(crate) const fn sieve_segment<const N: usize>(
    base_sieve: &[bool; N],
    upper_limit: u64,
) -> [bool; N] {
    const { assert!(N > 0, "`N` must be at least 1") }

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

/// Returns an array of size `N` that indicates which of the `N` integers in smaller than `upper_limit` are prime.
///
/// If you just want the prime status of the first `N` integers, see [`sieve`], and if you want the prime status of
/// the integers above some number, see [`sieve_geq`].
///
/// Fails to compile if `N` is 0, or if `MEM` is smaller than `N`.
///
/// # Examples
///
/// Basic usage
/// ```
/// # use const_primes::{sieve_lt, SieveError};
/// const PRIME_STATUSES: Result<[bool; 5], SieveError> = sieve_lt::<5, 6>(30);
///
/// assert_eq!(
///     PRIME_STATUSES,
/// //      25     26     27     28     29
///     Ok([false, false, false, false, true]),
/// );
/// ```
/// Sieve limited ranges of very large values
/// ```
/// # use const_primes::{sieve_lt, SieveError};
/// const BIG_PRIME_STATUSES: Result<[bool; 3], SieveError> = sieve_lt::<3, 70_711>(5_000_000_031);
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
/// assert_eq!(PS, Err(SieveError::TooLargeTotal));
/// ```
/// or smaller than `N`:
/// ```
/// # use const_primes::{sieve_lt, SieveError};
/// const PS: Result<[bool; 5], SieveError> = sieve_lt::<5, 5>(4);
/// assert_eq!(PS, Err(SieveError::TooSmallLimit));
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn sieve_lt<const N: usize, const MEM: usize>(
    upper_limit: u64,
) -> Result<[bool; N], SieveError> {
    const {
        assert!(N > 0, "`N` must be at least 1");
        assert!(MEM >= N, "`MEM` must be at least as large as `N`");
    }

    let mem_sqr = const {
        let mem64 = MEM as u64;
        match mem64.checked_mul(mem64) {
            Some(prod) => prod,
            None => panic!("`MEM`^2 must fit in a `u64`"),
        }
    };

    if upper_limit > mem_sqr {
        return Err(SieveError::TooLargeTotal);
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
/// Fails to compile if `N` is 0.
///
/// # Example
///
/// ```
/// # use const_primes::sieve;
/// const PRIMALITY: [bool; 10] = sieve();
/// //                     0      1      2     3     4      5     6      7     8      9
/// assert_eq!(PRIMALITY, [false, false, true, true, false, true, false, true, false, false]);
/// ```
#[must_use = "the function only returns a new value"]
pub const fn sieve<const N: usize>() -> [bool; N] {
    const { assert!(N > 0, "`N` must be at least 1") }

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
/// is invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SieveError {
    TooSmallLimit,
    TooLargeLimit,
    TooLargeTotal,
    TotalDoesntFitU64,
}

impl core::fmt::Display for SieveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooSmallLimit => write!(f, "`limit` must be at least `N`"),
            Self::TooLargeLimit => write!(f, "`limit` must be less than or equal to `MEM`^2"),
            Self::TooLargeTotal => write!(f, "`MEM + limit` must be less than or equal to `MEM`^2"),
            Self::TotalDoesntFitU64 => write!(f, "`MEM + limit` must fit in a `u64`"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SieveError {}

/// Returns the prime status of the `N` smallest integers greater than or equal to `lower_limit`.
///
/// Fails to compile if `N` is 0, or if `MEM` is smaller than `N`.
///
/// If you just want the prime status of the first N integers, see [`sieve`], and if you want the
/// prime status of the integers below some number, see [`sieve_lt`].
///
/// # Example
///
/// The size of the sieve, `MEM`, must be large enough for the largest sieved number to be smaller than `MEM`^2.
/// ```
/// # use const_primes::sieve_geq;
/// // The three numbers larger than or equal to 9 are 9, 10 and 11.
/// const N: usize = 3;
/// const LIMIT: u64 = 9;
/// // We thus need a memory size of at least 4, since 3*3 < 11, which isn't enough.
/// const MEM: usize = 4;
/// const PRIME_STATUS: [bool; N] = match sieve_geq::<N, MEM>(LIMIT) {Ok(s) => s, Err(_) => panic!()};
/// //                        9,     10,    11
/// assert_eq!(PRIME_STATUS, [false, false, true]);
/// ```
/// Enough memory can also be ensured with the help of functions provided by this crate:
/// ```
/// # use const_primes::{sieve_geq, SieveError};
/// use const_primes::isqrt;
/// const N: usize = 3;
/// const LIMIT: u64 = 5_000_000_038;
/// const PRIME_STATUS: Result<[bool; N], SieveError> = sieve_geq::<N, {isqrt(LIMIT) as usize + 1 + N}>(LIMIT);
/// //                           5_000_000_038  5_000_000_039  5_000_000_040
/// assert_eq!(PRIME_STATUS, Ok([false,         true,          false]));
/// ```
///
/// # Errors
///
/// Returns an error if `MEM + lower_limit` is larger than `MEM^2`.
/// ```
/// # use const_primes::{sieve_geq, SieveError};
/// const P: Result<[bool; 5], SieveError> = sieve_geq::<5, 5>(21);
/// assert_eq!(P, Err(SieveError::TooLargeTotal));
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn sieve_geq<const N: usize, const MEM: usize>(
    lower_limit: u64,
) -> Result<[bool; N], SieveError> {
    const {
        assert!(N > 0, "`N` must be at least 1");
        assert!(MEM >= N, "`MEM` must be at least as large as `N`");
    }
    let (mem64, mem_sqr) = const {
        let mem64 = MEM as u64;
        match mem64.checked_mul(mem64) {
            Some(prod) => (mem64, prod),
            None => panic!("`MEM`^2 must fit in a `u64`"),
        }
    };

    let Some(upper_limit) = mem64.checked_add(lower_limit) else {
        return Err(SieveError::TotalDoesntFitU64);
    };

    if upper_limit > mem_sqr {
        return Err(SieveError::TooLargeTotal);
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
