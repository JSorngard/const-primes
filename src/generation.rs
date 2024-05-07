use core::fmt;

use crate::{sieve, sieving::sieve_segment, Underlying};

/// Returns the `N` first prime numbers.
/// Fails to compile if `N` is 0.
///
/// [`Primes`](crate::Primes) might be relevant for you if you intend to later use these prime numbers for related computations.
///
/// Uses a [segmented sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve).
///
/// # Example
///
/// ```
/// # use const_primes::primes;
/// const PRIMES: [u32; 10] = primes();
/// assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
/// ```
/// Fails to compile if `N = 0`:
/// ```compile_fail
/// # use const_primes::primes;
/// let primes: [u32; 0] = primes();
/// ```
///
#[must_use = "the function only returns a new value"]
pub const fn primes<const N: usize>() -> [Underlying; N] {
    const { assert!(N > 0, "`N` must be at least 1") }

    if N == 1 {
        return [2; N];
    } else if N == 2 {
        let mut primes = [0; N];
        primes[0] = 2;
        primes[1] = 3;
        return primes;
    }

    // This is a segmented sieve that runs until it has found enough primes.

    // This array is the output in the end
    let mut primes = [0; N];
    // This keeps track of how many primes we've found so far.
    let mut prime_count = 0;

    // Sieve the first primes below N
    let mut sieve: [bool; N] = sieve();

    // Count how many primes we found
    // and store them in the final array
    let mut number = 0;
    while number < N {
        if sieve[number] {
            primes[prime_count] = number as Underlying;
            prime_count += 1;
        }

        number += 1;
    }

    // For every segment of N numbers
    let mut low = N - 1;
    let mut high = 2 * N - 1;
    'generate: while prime_count < N {
        // reset the sieve for the segment
        sieve = [true; N];
        let mut i = 0;

        // and repeat for each prime found so far:
        while i < prime_count {
            let prime = primes[i] as usize;

            // Find the smallest composite in the current segment,
            let mut composite = (low / prime) * prime;
            if composite < low {
                composite += prime;
            }

            // and sieve all numbers in the segment that are multiples of the prime.
            while composite < high {
                sieve[composite - low] = false;
                composite += prime;
            }

            i += 1;
        }

        // Move the found primes into the final array
        i = low;
        while i < high {
            if sieve[i - low] {
                primes[prime_count] = i as Underlying;
                prime_count += 1;
                // and stop the generation of primes if we're done.
                if prime_count >= N {
                    break 'generate;
                }
            }
            i += 1;
        }

        // Update low and high for the next segment
        low += N;
        high += N;
    }

    primes
}

/// Returns the `N` largest primes less than `upper_limit`.
///
/// This function uses a segmented sieve of size `MEM` for computation,
/// but only returns the `N` requested primes in the output array.
///
/// Set `MEM` such that `MEM*MEM >= upper_limit`.
///
/// Fails to compile if `N` or `MEM` is 0, if `MEM < N` or if `MEM`^2 does not fit in a u64.
///
/// If you want to compute primes that are larger than some limit, take a look at [`primes_geq`].
///
/// If you do not wish to compute the size requirement of the sieve manually, take a look at [`primes_segment!`].
///
/// # Example
///
/// Basic usage:
/// ```
/// # use const_primes::{primes_lt, GenerationError};
/// // Sieving up to 100 means the sieve needs to be of size ceil(sqrt(100)) = 10.
/// // However, we only save the 4 largest primes in the constant.
/// const PRIMES: [u64;4] = match primes_lt::<4, 10>(100) {Ok(ps) => ps, Err(_) => panic!()};
/// assert_eq!(PRIMES, [79, 83, 89, 97]);
/// ```
/// Compute larger primes:
/// ```
/// # use const_primes::{primes_lt, GenerationError};
/// use const_primes::isqrt;
/// const LIMIT: u64 = 5_000_000_030;
/// const BIG_PRIMES: Result<[u64; 3], GenerationError> = primes_lt::<3, {isqrt(LIMIT) as usize + 1}>(LIMIT);
///
/// assert_eq!(BIG_PRIMES, Ok([4_999_999_903, 4_999_999_937, 5_000_000_029]));
/// ```
///
/// # Errors
///
/// If the number of primes requested, `N`, is larger than
/// the number of primes that exists below the `upper_limit` this function
/// returns an error:
/// ```
/// # use const_primes::{primes_lt, GenerationError};
/// const N: usize = 9;
/// const PRIMES: Result<[u64; N], GenerationError> = primes_lt::<N, N>(10);
/// assert_eq!(PRIMES, Err(GenerationError::OutOfPrimes));
/// ```
///
/// It also returns an error if `upper_limit` is larger than `MEM`^2 or if `upper_limit` is smaller than or equal to 2:
/// ```
/// # use const_primes::{primes_lt, GenerationError};
/// const TOO_LARGE_LIMIT: Result<[u64; 3], GenerationError> = primes_lt::<3, 5>(26);
/// const TOO_SMALL_LIMIT: Result<[u64; 1], GenerationError> = primes_lt::<1, 1>(1);
/// assert_eq!(TOO_LARGE_LIMIT, Err(GenerationError::TooLargeLimit));
/// assert_eq!(TOO_SMALL_LIMIT, Err(GenerationError::TooSmallLimit));
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn primes_lt<const N: usize, const MEM: usize>(
    mut upper_limit: u64,
) -> Result<[u64; N], GenerationError> {
    const {
        assert!(N > 0, "`N` must be at least 1");
        assert!(MEM >= N, "`MEM` must be at least as large as `N`");
    }
    let mem_sqr = const {
        let mem64 = MEM as u64;
        match mem64.checked_mul(mem64) {
            Some(prod) => prod,
            None => panic!("`MEM`^2 must fit in a u64"),
        }
    };

    if upper_limit <= 2 {
        return Err(GenerationError::TooSmallLimit);
    }

    if upper_limit > mem_sqr {
        return Err(GenerationError::TooLargeLimit);
    }

    let mut primes: [u64; N] = [0; N];

    // This will be used to sieve all upper ranges.
    let base_sieve: [bool; MEM] = sieve();

    let mut total_primes_found: usize = 0;
    'generate: while total_primes_found < N {
        // This is the smallest prime we have found so far.
        let mut smallest_found_prime = primes[N - 1 - total_primes_found];
        // Sieve for primes in the segment.
        let upper_sieve: [bool; MEM] = sieve_segment(&base_sieve, upper_limit);

        let mut i: usize = 0;
        while i < MEM {
            // Iterate backwards through the upper sieve.
            if upper_sieve[MEM - 1 - i] {
                smallest_found_prime = upper_limit - 1 - i as u64;
                // Write every found prime to the primes array.
                primes[N - 1 - total_primes_found] = smallest_found_prime;
                total_primes_found += 1;
                if total_primes_found >= N {
                    // If we have found enough primes we stop sieving.
                    break 'generate;
                }
            }
            i += 1;
        }
        upper_limit = smallest_found_prime;
        if upper_limit <= 2 && total_primes_found < N {
            return Err(GenerationError::OutOfPrimes);
        }
    }

    Ok(primes)
}

/// Call [`primes_geq`] or [`primes_lt`], and automatically compute the memory requirement.
///
/// Estimates the value of the const generic `MEM` as `isqrt(upper_limit) + 1` for [`primes_lt`]
/// and as `isqrt(lower_limit) + 1 + N` for [`primes_geq`].
/// This will use more memory than needed for `primes_geq`.
///
/// # Example
///
/// ```
/// # use const_primes::{primes_segment, GenerationError};
/// const LIMIT: u64 = 5_000_000_031;
/// const PRIMES_GEQ: Result<[u64; 3], GenerationError> = primes_segment!(3; >= LIMIT);
/// const PRIMES_LT: Result<[u64; 3], GenerationError> = primes_segment!(3; < LIMIT);
/// // Can also be used at runtime:
/// let primes_geq = primes_segment!(3; >= LIMIT);
///
/// assert_eq!(PRIMES_GEQ, primes_geq);
/// assert_eq!(PRIMES_GEQ, Ok([5000000039, 5000000059, 5000000063]));
/// assert_eq!(PRIMES_LT, Ok([4999999903, 4999999937, 5000000029]));
/// ```
#[macro_export]
macro_rules! primes_segment {
    ($n:expr; < $lim:expr) => {
        $crate::primes_lt::<
            { $n },
            {
                let mem = { $lim };
                $crate::isqrt(mem) as ::core::primitive::usize + 1
            },
        >({ $lim })
    };
    ($n:expr; >= $lim:expr) => {
        $crate::primes_geq::<
            { $n },
            {
                let mem = { $lim };
                $crate::isqrt(mem) as ::core::primitive::usize + 1 + { $n }
            },
        >({ $lim })
    };
}

/// Returns the `N` smallest primes greater than or equal to `lower_limit`.
///
/// This function uses a segmented sieve of size `MEM` for computation,
/// but only returns the `N` requested primes in the output array.
///
/// Set `MEM` such that `MEM`^2 is larger than the largest prime you will encounter.
///
/// Fails to compile if `N` is 0. If `lower_limit` is less than 2 this functions assumes that it is 2,
/// since there are no primes smaller than 2.
///
/// If you want to compute primes smaller than some limit, take a look at [`primes_lt`].
///
/// If you do not wish to compute the size requirement of the sieve manually, take a look at [`primes_segment!`].
///
/// # Examples
///
/// Basic usage:
/// ```
/// use const_primes::{primes_geq, GenerationError};
/// // Compute 5 primes larger than 40. The largest will be 59, so `MEM` needs to be at least 8.
/// const PRIMES: [u64; 5] = match primes_geq::<5, 8>(40) {Ok(ps) => ps, Err(_) => panic!()};
/// assert_eq!(PRIMES, [41, 43, 47, 53, 59]);
/// ```
/// Estimate the size of `MEM` using the square root of the limit (and some extra, proportional to `N`):
/// ```
/// # use const_primes::{primes_geq, GenerationError};
/// use const_primes::isqrt;
/// const N: usize = 3;
/// const LIMIT: u64 = 5_000_000_030;
/// # #[allow(long_running_const_eval)]
/// const PRIMES_GEQ: Result<[u64; N], GenerationError> = primes_geq::<N, {isqrt(LIMIT) as usize + 1 + N}>(LIMIT);
/// assert_eq!(PRIMES_GEQ, Ok([5_000_000_039, 5_000_000_059, 5_000_000_063]));
/// # Ok::<(), GenerationError>(())
/// ```
///
/// # Errors
///
/// Only primes smaller than `MEM^2` can be generated, so if the sieve
/// encounters a number larger than that it results in an error:
/// ```
/// # use const_primes::{primes_geq, GenerationError};
/// const PRIMES: Result<[u64; 3], GenerationError> = primes_geq::<3, 3>(5);
/// // The sieve is unable to determine the prime status of 9, since that is the same or larger than `MEM`^2.
/// assert_eq!(PRIMES, Err(GenerationError::SieveOverrun(9)));
/// ```
///
/// Also returns an error if `lower_limit` is larger than or equal to `MEM^2`:
/// ```
/// # use const_primes::{primes_geq, GenerationError};
/// const PRIMES: Result<[u64; 5], GenerationError> = primes_geq::<5, 5>(26);
/// assert_eq!(PRIMES, Err(GenerationError::TooLargeLimit));
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn primes_geq<const N: usize, const MEM: usize>(
    lower_limit: u64,
) -> Result<[u64; N], GenerationError> {
    const {
        assert!(N > 0, "`N` must be at least 1");
        assert!(MEM >= N, "`MEM` must be at least as large as `N`");
    }
    let (mem64, mem_sqr) = const {
        let mem64 = MEM as u64;
        let Some(mem_sqr) = mem64.checked_mul(mem64) else {
            panic!("`MEM`^2 must fit in a `u64`")
        };
        (mem64, mem_sqr)
    };

    // If `lower_limit` is 2 or less, this is the same as calling `primes`,
    // so we just do that and convert the result to `u64`.
    if lower_limit <= 2 {
        let ans32: [u32; N] = primes();
        let mut ans64 = [0; N];
        let mut i = 0;
        while i < N {
            ans64[i] = ans32[i] as u64;
            i += 1;
        }
        return Ok(ans64);
    }

    if lower_limit >= mem_sqr {
        return Err(GenerationError::TooLargeLimit);
    }

    let mut primes = [0; N];
    let mut total_found_primes = 0;
    let mut largest_found_prime = 0;
    let base_sieve: [bool; MEM] = sieve();
    let mut sieve_limit = lower_limit;
    'generate: while total_found_primes < N {
        let upper_sieve = sieve_segment(&base_sieve, sieve_limit + mem64);

        let mut i = 0;
        while i < MEM {
            if upper_sieve[i] {
                largest_found_prime = sieve_limit + i as u64;

                // We can not know whether this is actually a prime since
                // the base sieve contains no information
                // about numbers larger than or equal to `MEM`^2.
                if largest_found_prime >= mem_sqr {
                    return Err(GenerationError::SieveOverrun(largest_found_prime));
                }

                if largest_found_prime >= lower_limit {
                    primes[total_found_primes] = largest_found_prime;
                    total_found_primes += 1;
                    if total_found_primes >= N {
                        // We've found enough primes.
                        break 'generate;
                    }
                }
            }
            i += 1;
        }
        sieve_limit = largest_found_prime + 1;
    }

    Ok(primes)
}

/// The error returned by [`primes_lt`] and [`primes_geq`] if the input
/// is invalid or does not work to produce the requested primes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerationError {
    /// The limit was larger than `MEM^2`.
    TooLargeLimit,
    /// The limit was smaller than or equal to 2.
    TooSmallLimit,
    /// Encountered a number larger than `MEM`^2.
    SieveOverrun(u64),
    /// Ran out of primes.
    OutOfPrimes,
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TooLargeLimit => write!(
                f,
                "the limit was larger than `MEM`^2"
            ),
            Self::TooSmallLimit => write!(
                f,
                "the limit was smaller than or equal to 2"
            ),
            Self::SieveOverrun(number) => write!(
                f,
                "encountered the number {number} which would have needed `MEM` to be at least {} to sieve", crate::imath::isqrt(*number) + 1
            ),
            Self::OutOfPrimes => write!(f, "ran out of primes before the array was filled"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GenerationError {}

#[cfg(test)]
mod test {
    use crate::is_prime;

    use super::*;

    #[test]
    fn sanity_check_primes_geq() {
        {
            const P: Result<[u64; 5], GenerationError> = primes_geq::<5, 5>(10);
            assert_eq!(P, Ok([11, 13, 17, 19, 23]));
        }
        {
            const P: Result<[u64; 5], GenerationError> = primes_geq::<5, 5>(0);
            assert_eq!(P, Ok([2, 3, 5, 7, 11]));
        }
        {
            const P: Result<[u64; 1], GenerationError> = primes_geq::<1, 1>(0);
            assert_eq!(P, Ok([2]));
        }
        for &prime in primes_geq::<2_000, 2_008>(3_998_000).unwrap().as_slice() {
            assert!(is_prime(prime));
        }
    }

    #[test]
    fn check_primes_segment() {
        const P_GEQ: Result<[u64; 10], GenerationError> = primes_segment!(10; >= 1000);
        const P_LT: Result<[u64; 10], GenerationError> = primes_segment!(10; < 1000);

        assert_eq!(
            P_GEQ,
            Ok([1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061])
        );
        assert_eq!(P_LT, Ok([937, 941, 947, 953, 967, 971, 977, 983, 991, 997]));
    }
}
