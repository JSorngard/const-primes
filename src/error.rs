//! Contains the implementation of the error type that is returned by the segmented sieving and generation functions.

use core::fmt;

/// The error returned by [`primes_lt`](crate::primes_lt) and [`primes_geq`](crate::primes_geq) if the input
/// is invalid or does not work to produce the requested primes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// The limit was larger than `MEM^2`.
    TooLargeLimit(u64, u64),
    /// The limit was smaller than or equal to 2.
    TooSmallLimit(u64),
    /// Encountered a number larger than `MEM`^2.
    SieveOverrun(u64),
    /// Ran out of primes.
    OutOfPrimes,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TooLargeLimit(limit, mem_sqr) => write!(
                f,
                "the limit ({limit}) was larger than `MEM`^2 ({mem_sqr})"
            ),
            Self::TooSmallLimit(limit) => write!(
                f,
                "the limit was {limit}, which is smaller than or equal to 2"
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
impl std::error::Error for Error {}
