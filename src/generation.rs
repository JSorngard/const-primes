use core::{
    fmt,
    ops::{Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use crate::{array_section::ArraySection, sieve, sieving::sieve_segment, Underlying};

/// Type alias for the type returned by the segmented generation functions, that otherwise has two generics that must be the same.
pub type Result<const N: usize> = core::result::Result<PrimesArray<N>, Error<N>>;

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
/// but only saves the `N` requested primes in the binary.
///
/// Set `MEM` such that `MEM*MEM >= upper_limit`.
///
/// Fails to compile if `N` or `MEM` is 0, if `MEM < N` or if `MEM`^2 does not fit in a u64.
///
/// The return array fills from the end until either it is full
/// (in which case the function returns the [`PrimesArray::Full`] variant)
/// or there are no more primes (in which case the function returns the [`PrimesArray::Partial`] variant).
///
/// If you want to compute primes that are larger than some limit, take a look at [`primes_geq`].
///
/// # Example
///
/// Basic usage:
/// ```
/// # use const_primes::generation::{Result, primes_lt, Error};
/// const PRIMES: Result<4> = primes_lt::<4, 10>(100);
/// assert_eq!(PRIMES?, [79, 83, 89, 97]);
/// # Ok::<(), Error<4>>(())
/// ```
/// Compute larger primes without starting from zero:
/// ```
/// # use const_primes::generation::{Result, primes_lt, Error};
/// # #[allow(long_running_const_eval)]
/// const BIG_PRIMES: Result<3> = primes_lt::<3, 70_711>(5_000_000_030);
///
/// assert_eq!(BIG_PRIMES?, [4_999_999_903, 4_999_999_937, 5_000_000_029]);
/// # Ok::<(), Error<3>>(())
/// ```
/// If there are not enough primes to fill the requested array,
/// the output will be the [`PrimesArray::Partial`] variant,
/// which contains fewer primes than requested:
/// ```
/// # use const_primes::generation::{Result, primes_lt, Error};
/// const PRIMES: Result<9> = primes_lt::<9, 9>(10);
/// assert_eq!(PRIMES?.as_slice(), &[2, 3, 5, 7]);
/// # Ok::<(), Error<9>>(())
/// ```
/// # Errors
///
/// Returns an error if `upper_limit` is larger than `MEM`^2 or if `upper_limit` is smaller than or equal to 2.
/// ```
/// # use const_primes::generation::{primes_lt, Result};
/// const TOO_LARGE_LIMIT: Result<3> = primes_lt::<3, 5>(26);
/// const TOO_SMALL_LIMIT: Result<1> = primes_lt::<1, 1>(1);
/// assert!(TOO_LARGE_LIMIT.is_err());
/// assert!(TOO_SMALL_LIMIT.is_err());
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn primes_lt<const N: usize, const MEM: usize>(mut upper_limit: u64) -> Result<N> {
    const {
        assert!(N > 0, "`N` must be at least 1");
        assert!(MEM >= N, "`MEM` must be at least as large as `N`");
        let mem64 = MEM as u64;
        assert!(
            mem64.checked_mul(mem64).is_some(),
            "`MEM`^2 must fit in a u64"
        );
    }

    if upper_limit <= 2 {
        return Err(Error::TooSmallLimit);
    }

    let mem64 = MEM as u64;
    match (mem64).checked_mul(mem64) {
        Some(prod) => {
            if upper_limit > prod {
                return Err(Error::TooLargeLimit);
            }
        }
        None => return Err(Error::TooLargeN),
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
            let restricted = ArraySection::new(primes, N - total_primes_found..N);
            return Ok(PrimesArray::Partial(restricted));
        }
    }

    Ok(PrimesArray::Full(primes))
}

/// Returns the `N` smallest primes greater than or equal to `lower_limit`.
/// Fails to compile if `N` is 0. If `lower_limit` is less than 2 this functions assumes that it is 2,
/// since there are no primes smaller than 2.
///
/// This function will fill the output array from index 0 and stop generating primes if they exceed `N^2`.
/// In that case the remaining elements of the output array will be 0.
///
/// If you want to compute primes smaller than the input, take a look at [`primes_lt`].
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use const_primes::{primes_geq, Result, Error};
/// const PRIMES: Result<5> = primes_geq::<5, 5>(10);
/// assert_eq!(PRIMES?, [11, 13, 17, 19, 23]);
/// # Ok::<(), Error<5>>(())
/// ```
/// Compute larger primes without starting from zero:
/// ```
/// # use const_primes::{primes_geq, Result, Error};
/// # #[allow(long_running_const_eval)]
/// const P: Result<3> = primes_geq::<3, 71_000>(5_000_000_030);
/// assert_eq!(P?, [5_000_000_039, 5_000_000_059, 5_000_000_063]);
/// // assert_eq!(P?[N - 3..], &[5_001_586_727, 5_001_586_729, 5_001_586_757]);
/// # Ok::<(), Error<3>>(())
/// ```
/// Only primes smaller than `MEM^2` will be generated:
/// ```
/// # use const_primes::{primes_geq, Result, Error};
/// const PRIMES: Result<3> = primes_geq::<3, 3>(5);
/// assert_eq!(PRIMES?.as_slice(), &[5, 7]);
/// # Ok::<(), Error<3>>(())
/// ```
///
/// # Errors
///
/// Returns an error if `lower_limit` is larger than or equal to `MEM^2`.
/// ```
/// # use const_primes::{primes_geq, Result};
/// const PRIMES: Result<5> = primes_geq::<5, 5>(26);
/// assert!(PRIMES.is_err());
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn primes_geq<const N: usize, const MEM: usize>(lower_limit: u64) -> Result<N> {
    const {
        assert!(N > 0, "`N` must be at least 1");
        assert!(MEM >= N, "`MEM` must be at least as large as `N`");
        let mem64 = MEM as u64;
        assert!(
            mem64.checked_mul(mem64).is_some(),
            "`MEM`^2 must fit in a u64"
        );
    }

    // There are no primes smaller than 2, so we will always start looking at 2.
    let lower_limit = if lower_limit >= 2 { lower_limit } else { 2 };

    let mem64 = MEM as u64;
    let mem_sqr = mem64 * mem64;

    if lower_limit >= mem_sqr {
        return Err(Error::TooLargeLimit);
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
                // about numbers larger than or equal to `MEM`.
                if largest_found_prime >= mem64 * mem64 {
                    return Ok(PrimesArray::Partial(ArraySection::new(
                        primes,
                        0..total_found_primes,
                    )));
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

        // let mut i = 0;
        // // Move all large enough primes into the output array.
        // while i < N {
        //     if upper_sieve[i] {
        //         largest_found_prime = lower_limit + i as u64;
        //         if largest_found_prime >= mem64 * mem64 {
        //             // We do not know if this is actually a prime
        //             // since the base sieve does not contain information about
        //             // the prime status of numbers larger than or equal to N.
        //             let restricted = ArraySection::new(primes, 0..total_found_primes);
        //             return Ok(PrimesArray::Partial(restricted));
        //         }
        //         primes[total_found_primes] = largest_found_prime;
        //         total_found_primes += 1;
        //         if total_found_primes >= N {
        //             // We've found enough primes
        //             break 'generate;
        //         }
        //     }
        //     i += 1;
        // }
        // lower_limit = largest_found_prime + 1;
    }
    Ok(PrimesArray::Full(primes))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// An array where potentially only a section of it contains valid prime numbers.
pub enum PrimesArray<const N: usize> {
    /// The entire allocated array contains prime numbers
    Full([u64; N]),
    /// Only a section of the allocated array contains prime numbers.
    Partial(ArraySection<u64, N>),
}

impl<const N: usize> PrimesArray<N> {
    /// Returns a slice of all the generated prime numbers.
    #[inline]
    pub const fn as_slice(&self) -> &[u64] {
        match self {
            Self::Full(ref arr) => arr.as_slice(),
            Self::Partial(ref arr_sec) => arr_sec.as_slice(),
        }
    }

    /// Returns the underlying array if it is full of valid primes.
    #[inline]
    pub const fn full(self) -> Option<[u64; N]> {
        match self {
            Self::Full(arr) => Some(arr),
            Self::Partial(_) => None,
        }
    }

    /// Returns a reference to the underlying array if it is full of valid primes.
    #[inline]
    pub const fn as_full(&self) -> Option<&[u64; N]> {
        match self {
            Self::Full(ref arr) => Some(arr),
            Self::Partial(_) => None,
        }
    }

    /// Returns the primes as a (maybe fully populated) [`ArraySection`].
    #[inline]
    pub const fn into_array_section(self) -> ArraySection<u64, N> {
        match self {
            Self::Partial(arr_sec) => arr_sec,
            Self::Full(arr) => {
                let l = arr.len();
                ArraySection::new(arr, 0..l)
            }
        }
    }

    /// Returns an [`ArraySection`] if not all of the elements
    /// in the underlying array could be computed.
    #[inline]
    pub const fn partial(self) -> Option<ArraySection<u64, N>> {
        match self {
            Self::Partial(arr_sec) => Some(arr_sec),
            Self::Full(_) => None,
        }
    }

    /// Returns a reference to the [`ArraySection`] if not all elements of the underlying array
    /// could be computed.
    #[inline]
    pub const fn as_partial(&self) -> Option<&ArraySection<u64, N>> {
        match self {
            Self::Partial(ref arr_sec) => Some(arr_sec),
            Self::Full(_) => None,
        }
    }

    /// Returns whether all the elements in the underlying array could be computed.
    #[inline]
    pub const fn is_full(&self) -> bool {
        matches!(self, Self::Full(_))
    }

    /// Returns whether only a subset of the underlying array could be computed.
    #[inline]
    pub const fn is_partial(&self) -> bool {
        matches!(self, Self::Partial(_))
    }

    /// Returns the length of the populated part of the array.
    #[inline]
    pub const fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// Returns wether the populated part of the array is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize, T> PartialEq<[T; N]> for PrimesArray<N>
where
    u64: PartialEq<T>,
{
    fn eq(&self, other: &[T; N]) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<const N: usize, T> PartialEq<[T]> for PrimesArray<N>
where
    u64: PartialEq<T>,
{
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice().eq(other)
    }
}

impl<const N: usize, T: PartialEq<u64>> PartialEq<PrimesArray<N>> for [T] {
    fn eq(&self, other: &PrimesArray<N>) -> bool {
        self.eq(other.as_slice())
    }
}

impl<const N: usize> From<PrimesArray<N>> for ArraySection<u64, N> {
    #[inline]
    fn from(value: PrimesArray<N>) -> Self {
        value.into_array_section()
    }
}

impl<const N: usize> AsRef<[u64]> for PrimesArray<N> {
    #[inline]
    fn as_ref(&self) -> &[u64] {
        self.as_slice()
    }
}

macro_rules! impl_index_range {
    ($($n:ty),*) => {
        $(
            impl<const N: usize> Index<$n> for PrimesArray<N> {
                type Output = [u64];
                fn index(&self, index: $n) -> &Self::Output {
                    self.as_slice().index(index)
                }
            }
        )*
    };
}

impl_index_range! {Range<usize>, RangeTo<usize>, RangeFrom<usize>, RangeToInclusive<usize>, RangeFull, RangeInclusive<usize>}

/// An enum describing whether the requested array could be filled completely, or only a partially.
/// A partial array can be returned by [`primes_lt`] if the size of the requested
/// array is larger than the actual number of primes less than the given `upper_limit`.
/// It can also be returned by [`primes_geq`] if it needs to sieve into a
/// region of numbers that exceed the square of the size of the requested array.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error<const N: usize> {
    /// `N^2`` did not fit in a `u64`.
    TooLargeN,
    /// the upper limit was larger than `N^2`.
    TooLargeLimit,
    /// the lower limit was smaller than or equal to 2.
    TooSmallLimit,
}

impl<const N: usize> fmt::Display for Error<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TooLargeN => write!(f, "`N^2` did not fit in a `u64`"),
            Self::TooLargeLimit => write!(f, "the upper limit was larger than `N^2`"),
            Self::TooSmallLimit => write!(f, "the lower limit was smaller than or equal to 2"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::is_prime;

    use super::*;

    #[test]
    fn sanity_check_primes_geq() {
        {
            const P: Result<5> = primes_geq::<5, 5>(10);
            assert_eq!(P.unwrap().as_slice(), [11, 13, 17, 19, 23]);
        }
        {
            const P: Result<5> = primes_geq::<5, 5>(0);
            assert_eq!(P.unwrap().as_slice(), [2, 3, 5, 7, 11]);
        }
        {
            const P: Result<1> = primes_geq::<1, 1>(0);
            assert_eq!(P, Err(Error::TooLargeLimit),);
        }
        for &prime in primes_geq::<2_000, 2_000>(3_998_000).unwrap().as_slice() {
            if prime == 0 {
                break;
            }
            assert!(is_prime(prime));
        }
    }
}
