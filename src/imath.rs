/// Returns the largest integer smaller than or equal to √n.
///
/// Uses a binary search.
#[must_use]
pub const fn isqrt(n: u64) -> u64 {
    if n == u64::MAX {
        return 4294967296;
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