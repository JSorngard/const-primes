/// Returns the greatest integer smaller than or equal to sqrt(x).
/// # Example
/// ```
/// # use const_primes::isqrt;
/// assert_eq!(isqrt(27), 5);
/// ```
pub const fn isqrt(x: usize) -> usize {
    let mut left = 0;
    let mut right = x + 1;
    
    while left != right - 1 {
        let mid = left + (right - left) / 2;
        if mid * mid <= x {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Primes<const N: usize> {
    primes: [usize; N],
}

impl<const N: usize> Primes<N> {
    pub const fn new() -> Self {
        let mut primes = [2; N];
        let mut number = 3;
        let mut i = 1;

        while i < N {
            let mut j = 0;
            let mut is_prime = true;
            while primes[j] < isqrt(number) + 1 {
                if number % primes[j] == 0 {
                    is_prime = false;
                    break;
                }
                j += 1;
            }
            if is_prime {
                primes[i] = number;
                i += 1;
            }
            number += 1;
        }

        Self { primes }
    }
}

impl<const N: usize> PartialEq<[usize; N]> for Primes<N> {
    fn eq(&self, other: &[usize; N]) -> bool {
        &self.primes == other
    }
}

impl<const N: usize> PartialEq<Primes<N>> for [usize; N] {
    fn eq(&self, other: &Primes<N>) -> bool {
        self == &other.primes
    }
}

impl<const N: usize> From<Primes<N>> for [usize; N] {
    fn from(const_primes: Primes<N>) -> Self {
        const_primes.primes
    }
}

impl<const N: usize> AsRef<[usize]> for Primes<N> {
    fn as_ref(&self) -> &[usize] {
        &self.primes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify() {
        assert_eq!([2, 3, 5, 7, 11], Primes::<5>::new());
    }
}