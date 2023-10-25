use crate::{are_prime, sieve::sieve_segment, Underlying};

/// Returns the `N` first prime numbers.
///
/// [`Primes`] might be relevant for you if you intend to later use these prime numbers for related computations.
///
/// Uses a [segmented sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve).
///
/// # Example
/// ```
/// # use const_primes::primes;
/// const PRIMES: [u32; 10] = primes();
/// assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
/// ```
/// # Panics
/// Panics if a computed prime overflows a `u32`. This will result in a compile error in a const context.  
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
    let mut sieve: [bool; N] = are_prime();

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
                if prime_count == N {
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

/// Returns the `N` largest primes below the given upper limit.
///
/// The return array fills from the end until either it is full or there are no more primes.
/// If the primes run out before the array is filled the first elements will have a value of zero.
///
/// Due to the limitations on memory allocation in `const` contexts the value of `N`
/// must satisfy the bounds `N < upper_limit <= N^2`.
///
/// # Example
/// Basic usage:
/// ```
/// # use const_primes::primes_below;
/// const PRIMES: [u64; 10] = primes_below(100);
///
/// assert_eq!(PRIMES, [53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
/// ```
/// Compute larger primes without starting from zero:
/// ```
/// # use const_primes::primes_below;
/// const N: usize = 70711;
/// # #[allow(long_running_const_eval)]
/// const BIG_PRIMES: [u64; N] = primes_below(5_000_000_030);
///
/// assert_eq!(&BIG_PRIMES[..3], &[4_998_417_421, 4_998_417_427, 4_998_417_443]);
/// assert_eq!(&BIG_PRIMES[N - 3..], &[4_999_999_903, 4_999_999_937, 5_000_000_029]);
/// ```
/// If there are not enough primes to fill the requested array, the first
/// elements will have a value of zero:
/// ```
/// # use const_primes::primes_below;
/// const PRIMES: [u64; 9] = primes_below(10);
///
/// assert_eq!(PRIMES, [0, 0, 0, 0, 0, 2, 3, 5, 7]);
/// ```
/// # Panics
/// Panics if `upper_limit` is not in the range `(N, N^2]`. This is a compile error
/// in const contexts:
/// ```compile_fail
/// # use const_primes::primes_below;
/// const PRIMES: [u64; 5] = primes_below(5);
/// ```
/// ```compile_fail
/// # use const_primes::primes_below;
/// const PRIMES: [u64; 5] = primes_below(26);
/// ```
pub const fn primes_below<const N: usize>(mut upper_limit: u64) -> [u64; N] {
    let n64 = N as u64;
    assert!(upper_limit > n64, "`upper_limit` must be larger than `N`");
    match (n64).checked_mul(n64) {
        Some(prod) => assert!(
            upper_limit <= prod,
            "`upper_limit` must be less than or equal to `N^2`"
        ),
        None => panic!("`N^2` must fit in a `u64`"),
    }

    let mut primes: [u64; N] = [0; N];

    // This will be used to sieve all upper ranges.
    let base_sieve: [bool; N] = are_prime();

    let mut total_primes_found: usize = 0;
    'generate: while total_primes_found < N && upper_limit > 2 {
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
    }

    primes
}

/// Returns an array of the `N` smallest primes greater than or equal to `lower_limit`.
pub const fn primes_geq<const N: usize>(mut lower_limit: u64) -> [u64; N] {
    let n64 = N as u64;
    if n64.checked_mul(n64).is_none() {
        panic!("`N^2` must fit in a `u64`");
    }

    let mut primes = [0; N];
    let base_sieve: [bool; N] = are_prime();
    let mut total_found_primes = 0;
    'generate: while total_found_primes < N && lower_limit + n64 <= n64 * n64 {
        let mut largest_found_prime = primes[total_found_primes];
        let upper_sieve = sieve_segment(&base_sieve, lower_limit + n64);
        let mut i = 0;
        // Move the found primes into the output vector.
        while i < N {
            if upper_sieve[i] {
                largest_found_prime = lower_limit + i as u64;
                primes[total_found_primes] = largest_found_prime;
                total_found_primes += 1;
                if total_found_primes >= N {
                    break 'generate;
                }
            }
            i += 1;
        }
        lower_limit = largest_found_prime + 1;
    }
    primes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sanity_check_primes_geq() {
        {
            const P: [u64; 5] = primes_geq(10);
            assert_eq!(P, [11, 13, 17, 19, 23]);
        }
        {
            const P: [u64; 5] = primes_geq(0);
            assert_eq!(P, [2, 3, 5, 7, 11]);
        }
        {
            const P: [u64; 0] = primes_geq(0);
            assert_eq!(P, []);
        }
        {
            const P: [u64; 1] = primes_geq(0);
            assert_eq!(P, [0]);
        }
    }

    #[test]
    fn check_primes_geq_large() {
        const N: usize = 71_000;
        #[allow(long_running_const_eval)]
        const P: [u64; N] = primes_geq(5_000_000_030);
        assert_eq!(P[..3], [5_000_000_039, 5_000_000_059, 5_000_000_063]);
        assert_eq!(P[N - 3..], [5_001_586_727, 5_001_586_729, 5_001_586_757]);
    }
}
