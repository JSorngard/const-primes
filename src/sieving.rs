use crate::isqrt;

/// Uses the primalities of the first `N` integers in `base_sieve` to sieve the numbers in the range `[upper_limit - N, upper_limit)`.
pub(crate) const fn sieve_segment<const N: usize>(
    base_sieve: &[bool; N],
    upper_limit: u64,
) -> [bool; N] {
    let mut segment_sieve = [true; N];

    let lower_limit = upper_limit - N as u64;

    // In case 0 and/or 1 are included in the upper sieve we need to treat them as a special case
    // since they are not multiples of any prime in `base_sieve` even though they are not primes.
    if lower_limit == 0 && N > 1 {
        segment_sieve[0] = false;
        segment_sieve[1] = false;
    } else if lower_limit == 1 && N > 0 {
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

/// Returns an array of size `N` that indicates which of the integers in `[upper_limit - N, upper_limit)` are prime,
/// or in other words: the value at a given index represents whether `index + upper_limit - N` is prime.
///
/// If you just want the prime status of the first `N` integers, see [`sieve`].
///
/// Uses a sieve of Eratosthenes to sieve the first `N` integers
/// and then uses the result to sieve the output range if needed.
///
/// # Examples
/// Basic usage
/// ```
/// # use const_primes::sieve_less_than;
/// const PRIME_STATUSES: [bool; 10] = sieve_less_than(30);
///
/// assert_eq!(
///     PRIME_STATUSES,
/// //   20     21     22     23    24     25     26     27     28     29
///     [false, false, false, true, false, false, false, false, false, true],
/// );
/// ```
/// Sieve limited ranges of very large values
/// ```
/// # use const_primes::sieve_less_than;
/// const BIG_NUMBER: u64 = 5_000_000_031;
/// const CEIL_SQRT_BIG_NUMBER: usize = 70711;
/// const BIG_PRIME_STATUSES: [bool; CEIL_SQRT_BIG_NUMBER] = sieve_less_than(BIG_NUMBER);
/// assert_eq!(
///     BIG_PRIME_STATUSES[CEIL_SQRT_BIG_NUMBER - 3..],
/// //  5_000_000_028  5_000_000_029  5_000_000_030
///     [false,        true,          false],
/// );
/// ```
///
/// # Panics
/// Panics if `upper_limit` is not in the range `[N, N^2]`. In const contexts these are compile errors:
/// ```compile_fail
/// # use const_primes::sieve_less_than;
/// const PRIME_STATUSES: [bool; 5] = sieve_less_than(26);
/// ```
/// ```compile_fail
/// # use const_primes::sieve_less_than;
/// const PRIME_STATUSES: [bool; 5] = sieve_less_than(4);
/// ```
#[must_use = "the function returns a new value and does not modify its input"]
pub const fn sieve_less_than<const N: usize>(upper_limit: u64) -> [bool; N] {
    let n64 = N as u64;

    // Since panics are compile time errors in const contexts
    // we check all the preconditions here and panic early.
    match n64.checked_mul(n64) {
        Some(prod) => assert!(
            upper_limit <= prod,
            "`upper_limit` must be smaller than or equal to `N`^2"
        ),
        None => panic!("`N`^2 must fit in a `u64`"),
    }
    assert!(upper_limit >= n64, "`upper_limit` must be at least `N`");

    // Use a normal sieve of Eratosthenes for the first N numbers.
    let base_sieve: [bool; N] = sieve();

    if upper_limit == n64 {
        // If we are not interested in sieving a larger range we can just return early.
        return base_sieve;
    }

    sieve_segment(&base_sieve, upper_limit)
}

/// Returns an array of size `N` where the value at a given index indicates whether the index is prime.
///
/// Uses a sieve of Eratosthenes.
///
/// # Example
/// ```
/// # use const_primes::sieve;
/// const PRIMALITY: [bool; 10] = sieve();
/// //                     0      1      2     3     4      5     6      7     8      9
/// assert_eq!(PRIMALITY, [false, false, true, true, false, true, false, true, false, false]);
/// ```
#[must_use = "the function only returns a new value"]
pub const fn sieve<const N: usize>() -> [bool; N] {
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

/// Returns the prime status of the `N` smallest integers greater than or equal to `lower_limit`.
///
/// # Example
/// Basic usage:
/// ```
/// # use const_primes::sieve_greater_than_or_equal_to;
/// const PRIME_STATUS: [bool; 5] = sieve_greater_than_or_equal_to(10);
/// //                        10     11    12     13    14
/// assert_eq!(PRIME_STATUS, [false, true, false, true, false]);
/// ```
/// # Panics
/// Panics if `N + lower_limit` is larger than `N^2`. In const contexts this is a compile error:
/// ```compile_fail
/// # use const_primes::sieve_greater_than_or_equal_to;
/// const P: [bool; 5] = sieve_greater_than_or_equal_to(21);
/// ```
pub const fn sieve_greater_than_or_equal_to<const N: usize>(lower_limit: u64) -> [bool; N] {
    let n64 = N as u64;

    // Since panics are compile time errors in const contexts
    // we check all the preconditions here and panic early.
    let upper_limit = if let Some(sum) = n64.checked_add(lower_limit) {
        sum
    } else {
        panic!("`N + lower_limit` must fit in a `u64`")
    };
    if let Some(n_sqr) = n64.checked_mul(n64) {
        assert!(
            upper_limit <= n_sqr,
            "`lower_limit + N` must be less than or equal to `N^2`"
        );
    } else {
        panic!("`N^2` must fit in a `u64`")
    }

    let base_sieve: [bool; N] = sieve();

    // If `lower_limit` is zero the upper range is the same as what we already sieved,
    // so we return early.
    if lower_limit == 0 {
        return base_sieve;
    }

    sieve_segment(&base_sieve, upper_limit)
}
