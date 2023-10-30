use crate::{restricted_array::RestrictedArray, sieve, sieving::sieve_segment, Underlying};

pub mod segmented_generation_result;

pub use segmented_generation_result::SegmentedGenerationResult;
/// Returns the `N` first prime numbers.
///
/// [`Primes`](crate::Primes) might be relevant for you if you intend to later use these prime numbers for related computations.
///
/// Uses a [segmented sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve).
///
/// # Example
/// ```
/// # use const_primes::primes;
/// const PRIMES: [u32; 10] = primes();
/// assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
/// ```
#[must_use = "the function only returns a new value"]
pub const fn primes<const N: usize>() -> [Underlying; N] {
    if N == 0 {
        return [0; N];
    } else if N == 1 {
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
/// The return array fills from the end until either it is full or there are no more primes.
/// If the primes run out before the array is filled the first elements will have a value of zero.
///
/// If you want to compute primes that are larger than the input, take a look at [`primes_geq`].
///
/// # Example
/// Basic usage:
/// ```
/// # use const_primes::generation::{SegmentedGenerationResult, primes_lt};
/// const PRIMES: SegmentedGenerationResult<10> = primes_lt(100);
/// assert_eq!(PRIMES[2], 61);
/// assert_eq!(PRIMES, [53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
/// ```
/// Compute larger primes without starting from zero:
/// ```
/// # use const_primes::generation::{SegmentedGenerationResult, primes_lt};
/// const N: usize = 70711;
/// # #[allow(long_running_const_eval)]
/// // If the generation results in a completely filled array, it can be extracted like this:
/// const BIG_PRIMES: [u64; N] = match primes_lt(5_000_000_030).complete() {Some(array) => array, _ => panic!()};
///
/// assert_eq!(BIG_PRIMES[..3],     [4_998_417_421, 4_998_417_427, 4_998_417_443]);
/// assert_eq!(BIG_PRIMES[N - 3..], [4_999_999_903, 4_999_999_937, 5_000_000_029]);
/// ```
/// If there are not enough primes to fill the requested array,
/// the output will be the [`SegmentedGenerationResult::Partial`] variant,
/// which contains fewer primes than requested:
/// ```
/// # use const_primes::generation::{SegmentedGenerationResult, primes_lt};
/// const PRIMES: SegmentedGenerationResult<9> = primes_lt(10);
/// // There are only four primes less than 10:
/// assert_eq!(PRIMES, [2, 3, 5, 7]);
/// ```
/// # Panics
/// Panics if `N^2` does not fit in a `u64` or if `upper_limit` is larger than `N^2`. This is a compile error
/// in const contexts:
/// ```compile_fail
/// # use const_primes::generation::{SegmentedGenerationResult,primes_lt};
/// //                                       N is too large
/// const PRIMES: SegmentedGenerationResult<{u32::MAX as usize + 1}> = primes_lt(100);
/// ```
/// ```compile_fail
/// # use const_primes::generation::{primes_lt, SegmentedGenerationResult};
/// //                                      N              upper_limit > N^2
/// const PRIMES: SegmentedGenerationResult<5> = primes_lt(26);
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn primes_lt<const N: usize>(mut upper_limit: u64) -> SegmentedGenerationResult<N> {
    let n64 = N as u64;
    match (n64).checked_mul(n64) {
        Some(prod) => assert!(
            upper_limit <= prod,
            "`upper_limit` must be less than or equal to `N^2`"
        ),
        None => panic!("`N^2` must fit in a `u64`"),
    }

    let mut primes: [u64; N] = [0; N];

    // This will be used to sieve all upper ranges.
    let base_sieve: [bool; N] = sieve();

    let mut total_primes_found: usize = 0;
    'generate: while total_primes_found < N {
        // This is the smallest prime we have found so far.
        let mut smallest_found_prime = primes[N - 1 - total_primes_found];
        // Sieve for primes in the segment.
        let upper_sieve: [bool; N] = sieve_segment(&base_sieve, upper_limit);

        let mut i: usize = 0;
        while i < N {
            // Iterate backwards through the upper sieve.
            if upper_sieve[N - 1 - i] {
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
        if upper_limit <= 2 {
            let restricted = RestrictedArray::new(N - total_primes_found..N, primes);
            return SegmentedGenerationResult::Partial(restricted);
        }
    }

    SegmentedGenerationResult::Complete(primes)
}

/// Returns the `N` smallest primes greater than or equal to `lower_limit`.
///
/// This function will fill the output array from index 0 and stop generating primes if they exceed `N^2`.
/// In that case the remaining elements of the output array will be 0.
///
/// If you want to compute primes smaller than the input, take a look at [`primes_lt`].
///
/// # Example
/// Basic usage:
/// ```
/// # use const_primes::primes_geq;
/// const PRIMES: [u64; 5] = primes_geq(10);
/// assert_eq!(PRIMES, [11, 13, 17, 19, 23]);
/// ```
/// Compute larger primes without starting from zero:
/// ```
/// # use const_primes::primes_geq;
/// const N: usize = 71_000;
/// # #[allow(long_running_const_eval)]
/// const P: [u64; N] = primes_geq(5_000_000_030);
/// assert_eq!(P[..3], [5_000_000_039, 5_000_000_059, 5_000_000_063]);
/// assert_eq!(P[N - 3..], [5_001_586_727, 5_001_586_729, 5_001_586_757]);
/// ```
/// Only primes smaller than `N^2` will be generated:
/// ```
/// # use const_primes::primes_geq;
/// const PRIMES: [u64; 3] = primes_geq(5);
/// assert_eq!(PRIMES, [5, 7, 0]);
/// ```
/// # Panics
/// Panics if `N^2` does not fit in a `u64`.
/// ```compile_fail
/// # use const_primes::primes_geq;
/// const P: [u64; u32::MAX as usize + 1] = primes_geq(0);
/// ```
#[must_use = "the function only returns a new value and does not modify its input"]
pub const fn primes_geq<const N: usize>(mut lower_limit: u64) -> SegmentedGenerationResult<N> {
    let n64 = N as u64;
    if n64.checked_mul(n64).is_none() {
        panic!("`N^2` must fit in a `u64`");
    }

    let mut primes = [0; N];
    let base_sieve: [bool; N] = sieve();
    let mut total_found_primes = 0;
    'generate: while total_found_primes < N {
        let mut largest_found_prime = primes[total_found_primes];
        let upper_sieve = sieve_segment(&base_sieve, lower_limit + n64);
        let mut i = 0;
        // Move the found primes into the output vector.
        while i < N {
            if upper_sieve[i] {
                largest_found_prime = lower_limit + i as u64;
                if largest_found_prime >= n64 * n64 {
                    // We do not know if this is actually a prime
                    // since the base sieve does not contain information about
                    // the prime status of numbers larger than or equal to N.
                    let restricted = RestrictedArray::new(0..total_found_primes, primes);
                    return SegmentedGenerationResult::Partial(restricted);
                }
                primes[total_found_primes] = largest_found_prime;
                total_found_primes += 1;
                if total_found_primes >= N {
                    // We've found enough primes
                    break 'generate;
                }
            }
            i += 1;
        }
        lower_limit = largest_found_prime + 1;
    }
    SegmentedGenerationResult::Complete(primes)
}

#[cfg(test)]
mod test {
    use crate::is_prime;

    use super::*;

    #[test]
    fn sanity_check_primes_geq() {
        {
            const P: [u64; 5] = match primes_geq(10).complete() {
                Some(a) => a,
                _ => panic!(),
            };
            assert_eq!(P, [11, 13, 17, 19, 23]);
        }
        {
            const P: [u64; 5] = match primes_geq(0).complete() {
                Some(a) => a,
                _ => panic!(),
            };
            assert_eq!(P, [2, 3, 5, 7, 11]);
        }
        {
            const P: SegmentedGenerationResult<0> = primes_geq(0);
            assert_eq!(P.as_ref(), []);
        }
        {
            const P: SegmentedGenerationResult<1> = primes_geq(0);
            assert_eq!(P, [2]);
        }
        for prime in primes_geq::<2_000>(3_998_000) {
            if prime == 0 {
                break;
            }
            assert!(is_prime(prime));
        }
    }
}
