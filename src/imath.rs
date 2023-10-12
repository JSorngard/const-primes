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
        const TEST_CASES: [u64; 100] = [0, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
        for (x, ans) in TEST_CASES.into_iter().enumerate() {
            assert_eq!(isqrt(x as u64), ans);
        }
        assert_eq!(
            f64::from(u32::MAX).sqrt().floor() as u64,
            isqrt(u64::from(u32::MAX))
        );
        assert_eq!(isqrt(u64::MAX), 4294967296);
    }
}
