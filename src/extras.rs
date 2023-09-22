//! The functions defined in this module are
//! either used in the generation of prime numbers or relevant to the purpose
//! of the crate in other ways.

use crate::Underlying;

/// Determines whether `n` is prime using trial division.
///
/// # Example
/// ```
/// # use const_primes::extras::is_prime;
/// const IS_101_A_PRIME: bool = is_prime(101);
/// assert!(IS_101_A_PRIME);
/// ```
#[must_use]
pub const fn is_prime(n: Underlying) -> bool {
    if n == 2 || n == 3 {
        true
    } else if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let mut candidate_factor = 5;
        let bound = isqrt(n) + 1;
        while candidate_factor < bound {
            if n % candidate_factor == 0 || n % (candidate_factor + 2) == 0 {
                return false;
            }
            candidate_factor += 6;
        }
        true
    }
}

/// Returns the largest integer smaller than or equal to √n.
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
        for (x, ans) in TEST_CASES.into_iter().enumerate() {
            assert_eq!(is_prime(x as Underlying), ans);
        }
    }
}
