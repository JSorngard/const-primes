//! This module contains const math operations on integers that are used by the other
//! functions in the crate.

/// Returns the largest integer smaller than or equal to `√n`.
///
/// Uses a binary search.
#[must_use]
pub const fn isqrt(n: u64) -> u64 {
    if n == u64::MAX {
        return 4_294_967_296;
    }

    let mut left = 0;
    let mut right = n + 1;

    while left != right - 1 {
        let mid = left + (right - left) / 2;
        if mid as u128 * mid as u128 <= n as u128 {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

/// Calculates `(base ^ exp) % modulo` without overflow.
#[must_use]
pub const fn mod_pow(mut base: u64, mut exp: u64, modulo: u64) -> u64 {
    let mut res = 1;

    base %= modulo;

    while exp > 0 {
        if exp % 2 == 1 {
            res = mod_mul(res, base, modulo);
        }
        base = mod_mul(base, base, modulo);
        exp >>= 1;
    }

    res
}

/// Calculates `(a * b) % modulo` without overflow.
#[must_use]
pub const fn mod_mul(a: u64, b: u64, modulo: u64) -> u64 {
    ((a as u128 * b as u128) % modulo as u128) as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_isqrt() {
        #[rustfmt::skip]
        const TEST_CASES: [(u64, u64); 101] = [(0, 0), (1, 1), (2, 1), (3, 1), (4, 2), (5, 2), (6, 2), (7, 2), (8, 2), (9, 3), (10, 3), (11, 3), (12, 3), (13, 3), (14, 3), (15, 3), (16, 4), (17, 4), (18, 4), (19, 4), (20, 4), (21, 4), (22, 4), (23, 4), (24, 4), (25, 5), (26, 5), (27, 5), (28, 5), (29, 5), (30, 5), (31, 5), (32, 5), (33, 5), (34, 5), (35, 5), (36, 6), (37, 6), (38, 6), (39, 6), (40, 6), (41, 6), (42, 6), (43, 6), (44, 6), (45, 6), (46, 6), (47, 6), (48, 6), (49, 7), (50, 7), (51, 7), (52, 7), (53, 7), (54, 7), (55, 7), (56, 7), (57, 7), (58, 7), (59, 7), (60, 7), (61, 7), (62, 7), (63, 7), (64, 8), (65, 8), (66, 8), (67, 8), (68, 8), (69, 8), (70, 8), (71, 8), (72, 8), (73, 8), (74, 8), (75, 8), (76, 8), (77, 8), (78, 8), (79, 8), (80, 8), (81, 9), (82, 9), (83, 9), (84, 9), (85, 9), (86, 9), (87, 9), (88, 9), (89, 9), (90, 9), (91, 9), (92, 9), (93, 9), (94, 9), (95, 9), (96, 9), (97, 9), (98, 9), (99, 9), (u64::MAX, 4294967296)];
        for (x, ans) in TEST_CASES {
            assert_eq!(isqrt(x as u64), ans);
        }
        assert_eq!(
            f64::from(u32::MAX).sqrt().floor() as u64,
            isqrt(u64::from(u32::MAX))
        );
    }
}
