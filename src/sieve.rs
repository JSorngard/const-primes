//! This module contains implementations of prime related functions that use the
//! [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).

/// Returns an array where the value at a given index indicates whether the index is prime.
///
/// # Example
/// ```
/// # use const_primes::sieve::primalities;
/// const PRIMALITY: [bool; 10] = primalities();
/// assert_eq!(PRIMALITY, [false, false, true, true, false, true, false, true, false, false]);
/// ```
pub const fn primalities<const N: usize>() -> [bool; N] {
    let mut is_prime = [true; N];
    if N > 0 {
        is_prime[0] = false;
    }
    if N > 1 {
        is_prime[1] = false;
    }

    let mut number = 2;
    while number * number < N {
        if is_prime[number] {
            let mut composite = number * number;
            while composite < N {
                is_prime[composite] = false;
                composite += number;
            }
        }
        number += 1;
    }

    is_prime
}

/// Returns whether `N` is prime.
///
/// For a version of this function that does not need const generics, see [`trial::is_prime`](crate::trial::is_prime).
///
/// # Example
/// Basic usage
/// ```
/// # use const_primes::sieve::is_prime;
/// const IS_101_A_PRIME: bool = is_prime::<101>();
/// assert!(IS_101_A_PRIME);
/// ```
#[must_use]
pub const fn is_prime<const N: usize>() -> bool {
    if N == 2 || N == 3 {
        true
    } else if N <= 1 || N % 2 == 0 || N % 3 == 0 {
        false
    } else {
        let mut primality = [true; N];
        // 1 is not prime
        primality[0] = false;
        let mut number = 3;
        while number * number <= N {
            if primality[number - 1] {
                let mut composite = number * number;
                while composite <= N {
                    primality[composite - 1] = false;
                    composite += number;
                }
            }
            number += 1;
        }
        match primality.last() {
            Some(n) => *n,
            None => panic!("`primality` always has a non-zero size at this point"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_is_prime() {
        #[rustfmt::skip]
        const TEST_CASES: [bool; 100] = [false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false];
        macro_rules! test_n {
            ($($n:expr),+) => {
                $(
                    {
                        assert_eq!(is_prime::<$n>(), TEST_CASES[$n]);
                    }
                )+
            };
        }
        test_n!(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99
        );
    }
}
