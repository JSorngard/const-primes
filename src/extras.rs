//! The functions defined in this module are
//! either used in the generation of prime numbers or relevant to the purpose
//! of the crate in other ways.

use crate::Underlying;

/// Returns whether `n` is prime.
///
/// Uses the sieve of Eratosthenes.
/// # Example
/// Basic usage
/// ```
/// # use const_primes::extras::is_prime;
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

/// Returns an array that describes whether the number at each index is prime.
///
/// Uses the sieve of Eratosthenes.
/// # Example
/// ```
/// # use const_primes::extras::primalities;
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

/// Returns the largest integer smaller than or equal to √n.
/// 
/// Uses a binary search.
///
/// # Example
/// ```
/// # use const_primes::extras::isqrt;
/// const sqrt_27: u32 = isqrt(27);
/// assert_eq!(sqrt_27, 5);
/// ```
#[must_use]
pub const fn isqrt(n: Underlying) -> Underlying {
    let mut left = 0;
    let mut right = n + 1;

    while left != right - 1 {
        let mid = left + (right - left) / 2;
        if mid * mid <= n {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_isqrt() {
        #[rustfmt::skip]
        const TEST_CASES: [(Underlying, Underlying); 100] = [(0, 0), (1, 1), (2, 1), (3, 1), (4, 2), (5, 2), (6, 2), (7, 2), (8, 2), (9, 3), (10, 3), (11, 3), (12, 3), (13, 3), (14, 3), (15, 3), (16, 4), (17, 4), (18, 4), (19, 4), (20, 4), (21, 4), (22, 4), (23, 4), (24, 4), (25, 5), (26, 5), (27, 5), (28, 5), (29, 5), (30, 5), (31, 5), (32, 5), (33, 5), (34, 5), (35, 5), (36, 6), (37, 6), (38, 6), (39, 6), (40, 6), (41, 6), (42, 6), (43, 6), (44, 6), (45, 6), (46, 6), (47, 6), (48, 6), (49, 7), (50, 7), (51, 7), (52, 7), (53, 7), (54, 7), (55, 7), (56, 7), (57, 7), (58, 7), (59, 7), (60, 7), (61, 7), (62, 7), (63, 7), (64, 8), (65, 8), (66, 8), (67, 8), (68, 8), (69, 8), (70, 8), (71, 8), (72, 8), (73, 8), (74, 8), (75, 8), (76, 8), (77, 8), (78, 8), (79, 8), (80, 8), (81, 9), (82, 9), (83, 9), (84, 9), (85, 9), (86, 9), (87, 9), (88, 9), (89, 9), (90, 9), (91, 9), (92, 9), (93, 9), (94, 9), (95, 9), (96, 9), (97, 9), (98, 9), (99, 9)];
        for (x, ans) in TEST_CASES {
            assert_eq!(isqrt(x), ans);
        }
    }

    #[test]
    fn check_is_prime() {
        #[rustfmt::skip]
        const TEST_CASES: [bool; 100] = [false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false];
        macro_rules! test_n {
            ($($n:expr),+) => {
                $(
                    assert!(is_prime::<$n>() == TEST_CASES[$n]);
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
