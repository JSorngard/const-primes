/// Returns the greatest integer smaller than or equal to sqrt(x).
/// E.g. isqrt(27) is 5.
const fn isqrt(x: usize) -> usize {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstPrimes<const N: usize> {
    primes: [usize; N],
}

impl<const N: usize> ConstPrimes<N> {
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

impl<const N: usize> PartialEq<[usize; N]> for ConstPrimes<N> {
    fn eq(&self, other: &[usize; N]) -> bool {
        &self.primes == other
    }
}

impl<const N: usize> PartialEq<ConstPrimes<N>> for [usize; N] {
    fn eq(&self, other: &ConstPrimes<N>) -> bool {
        self == &other.primes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify() {
        assert_eq!([2, 3, 5, 7, 11], ConstPrimes::<5>::new());
    }
}