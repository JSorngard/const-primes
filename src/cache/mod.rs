//! This module contains the implementation of the type [`Primes`] (and related iterators),
//! which functions as a cache of prime numbers for related computations.

mod prime_factors;
mod primes_into_iter;
mod primes_iter;

pub use prime_factors::{PrimeFactorization, PrimeFactors};
pub use primes_into_iter::PrimesIntoIter;
pub use primes_iter::PrimesIter;

use crate::{primes, Underlying};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// region: Primes<N>

/// A wrapper around an array that consists of the first `N` primes. Can use those primes for related computations.
///
/// Can be created and used in const contexts, and if so it ensures that `N` is non-zero at compile time.
///
/// # Examples
///
/// Basic usage
/// ```
/// # use const_primes::Primes;
/// const PRIMES: Primes<3> = Primes::new();
/// assert_eq!(PRIMES[2], 5);
/// assert_eq!(PRIMES, [2, 3, 5]);
/// ```
/// Reuse sieved primes for other computations
/// ```
/// # use const_primes::Primes;
/// const CACHE: Primes<100> = Primes::new();
/// const PRIME_CHECK: Option<bool> = CACHE.is_prime(541);
/// const PRIME_COUNT: Option<usize> = CACHE.count_primes_leq(200);
///
/// assert_eq!(PRIME_CHECK, Some(true));
/// assert_eq!(PRIME_COUNT, Some(46));
///
/// // If questions are asked about numbers outside the cache it returns None
/// assert_eq!(CACHE.is_prime(1000), None);
/// assert_eq!(CACHE.count_primes_leq(1000), None);
/// ```
#[derive(Debug, Clone, Copy, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Primes<const N: usize>(
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))] [Underlying; N],
);

impl<const N: usize> Primes<N> {
    /// Generates a new instance that contains the first `N` primes.
    ///
    /// Uses a [segmented sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve).
    ///
    /// # Examples
    ///
    /// Basic usage
    /// ```
    /// # use const_primes::Primes;
    /// const PRIMES: Primes<3> = Primes::new();
    /// assert_eq!(PRIMES, [2, 3, 5]);
    /// ```
    /// Determine `N` through type inference
    /// ```
    /// # use const_primes::Primes;
    /// assert_eq!(Primes::new(), [2, 3, 5, 7, 11]);
    /// ```
    /// Specify `N` manually
    /// ```
    /// # use const_primes::Primes;
    /// let primes = Primes::<5>::new();
    /// assert_eq!(primes, [2, 3, 5, 7, 11]);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `N` is 0, which is a compile error in const contexts:
    /// ```compile_fail
    /// # use const_primes::Primes;
    /// const NO_PRIMES: Primes<0> = Primes::new();
    /// ```
    /// This is always a compile error if the `const_assert` feature is enabled.
    ///
    /// If any of the primes overflow a `u32` it will panic in const contexts or debug mode.
    #[must_use = "the associated method only returns a new value"]
    pub const fn new() -> Self {
        #[cfg(feature = "const_assert")]
        inline_const!(assert!(N > 0, "`N` must be at least 1"));
        #[cfg(not(feature = "const_assert"))]
        assert!(N > 0, "`N` must be at least 1");

        Self(primes())
    }

    /// Returns whether `n` is prime, if it is smaller than or equal to the largest prime in `self`.
    ///
    /// Uses a binary search.
    ///
    /// # Example
    ///
    /// Basic usage
    /// ```
    /// # use const_primes::Primes;
    /// const PRIMES: Primes<100> = Primes::new();
    /// const TMOLTUAE: Option<bool> = PRIMES.is_prime(42);
    ///
    /// assert_eq!(PRIMES.is_prime(13), Some(true));
    /// assert_eq!(TMOLTUAE, Some(false));
    /// // 1000 is larger than 541, the largest prime in the cache,
    /// // so we don't know whether it's prime.
    /// assert_eq!(PRIMES.is_prime(1000), None);
    /// ```
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn is_prime(&self, n: u32) -> Option<bool> {
        match self.binary_search(n) {
            Ok(_) => Some(true),
            Err(i) => {
                if i < N {
                    Some(false)
                } else {
                    None
                }
            }
        }
    }

    /// Returns the number of primes smaller than or equal to `n`, if it's smaller than or equal to the largest prime in `self`.
    ///
    /// Uses a binary search to count the primes.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// # use const_primes::Primes;
    /// const CACHE: Primes<100> = Primes::new();
    /// const COUNT1: Option<usize> = CACHE.count_primes_leq(500);
    /// const COUNT2: Option<usize> = CACHE.count_primes_leq(11);
    /// const OUT_OF_BOUNDS: Option<usize> = CACHE.count_primes_leq(1_000);
    ///
    /// assert_eq!(COUNT1, Some(95));
    /// assert_eq!(COUNT2, Some(5));
    /// assert_eq!(OUT_OF_BOUNDS, None);
    /// ```
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn count_primes_leq(&self, n: Underlying) -> Option<usize> {
        match self.binary_search(n) {
            Ok(i) => Some(i + 1),
            Err(maybe_i) => {
                if maybe_i < N {
                    Some(maybe_i)
                } else {
                    None
                }
            }
        }
    }

    /// Returns an iterator over the prime factors of the given number in increasing order as well as their
    /// multiplicities.
    ///
    /// If a number contains prime factors larger than the largest prime in `self`,
    /// they will not be yielded by the iterator, but their product can be retrieved by calling
    /// [`remainder`](PrimeFactorization::remainder) on the iterator.
    ///
    /// If you do not need to know the multiplicity of each prime factor,
    /// it may be faster to use [`prime_factors`](Self::prime_factors).
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use const_primes::Primes;
    /// // Contains the primes [2, 3, 5]
    /// const CACHE: Primes<3> = Primes::new();
    ///
    /// assert_eq!(CACHE.prime_factorization(15).collect::<Vec<_>>(), &[(3, 1), (5, 1)]);
    /// ```
    /// The second element of the returned tuples is the multiplicity of the prime in the number:
    /// ```
    /// # use const_primes::Primes;
    /// # const CACHE: Primes<3> = Primes::new();
    /// // 1024 = 2^10
    /// assert_eq!(CACHE.prime_factorization(1024).next(), Some((2, 10)));
    /// ```
    /// 294 has 7 as a prime factor, but 7 is not in the cache:
    /// ```
    /// # use const_primes::Primes;
    /// # const CACHE: Primes<3> = Primes::new();
    /// // 294 = 2*3*7*7
    /// let mut factorization_of_294 = CACHE.prime_factorization(294);
    ///
    /// // only 2 and 3 are in the cache:
    /// assert_eq!(factorization_of_294.by_ref().collect::<Vec<_>>(), &[(2, 1), (3, 1)]);
    ///
    /// // the factor of 7*7 can be found with the remainder function:
    /// assert_eq!(factorization_of_294.remainder(), Some(49));
    /// ```
    #[inline]
    pub fn prime_factorization(&self, number: Underlying) -> PrimeFactorization<'_> {
        PrimeFactorization::new(&self.0, number)
    }

    /// Returns an iterator over all the prime factors of the given number in increasing order.
    ///
    /// If a number contains prime factors larger than the largest prime in `self`,
    /// they will not be yielded by the iterator, but their product can be retrieved by calling
    /// [`remainder`](PrimeFactors::remainder) on the iterator.
    ///
    /// If you also wish to know the multiplicity of each prime factor of the number,
    /// take a look at [`prime_factorization`](Self::prime_factorization).
    ///
    /// # Examples
    ///
    /// ```
    /// # use const_primes::Primes;
    /// // Contains [2, 3, 5]
    /// const CACHE: Primes<3> = Primes::new();
    ///
    /// assert_eq!(CACHE.prime_factors(3*5).collect::<Vec<_>>(), &[3, 5]);
    /// assert_eq!(CACHE.prime_factors(2*2*2*2*3).collect::<Vec<_>>(), &[2, 3]);
    /// ```
    /// 294 has 7 as a prime factor, but 7 is not in the cache:
    /// ```
    /// # use const_primes::Primes;
    /// # const CACHE: Primes<3> = Primes::new();
    /// // 294 = 2*3*7*7
    /// let mut factors_of_294 = CACHE.prime_factors(294);
    ///
    /// // only 2 and 3 are in the cache
    /// assert_eq!(factors_of_294.by_ref().collect::<Vec<_>>(), &[2, 3]);
    ///
    /// // the factor of 7*7 can be found with the remainder function
    /// assert_eq!(factors_of_294.remainder(), Some(49));
    /// ```
    #[inline]
    pub fn prime_factors(&self, number: Underlying) -> PrimeFactors<'_> {
        PrimeFactors::new(&self.0, number)
    }

    // region: Next prime

    /// Returns the largest prime less than `n`.  
    /// If `n` is 0, 1, 2, or larger than the largest prime in `self` this returns `None`.
    ///
    /// Uses a binary search.
    ///
    /// # Example
    ///
    /// ```
    /// # use const_primes::Primes;
    /// const CACHE: Primes<100> = Primes::new();
    /// const PREV400: Option<u32> = CACHE.previous_prime(400);
    /// assert_eq!(PREV400, Some(397));
    /// ```
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn previous_prime(&self, n: Underlying) -> Option<Underlying> {
        if n <= 2 {
            None
        } else {
            match self.binary_search(n) {
                Ok(i) | Err(i) => {
                    if i > 0 && i < N {
                        Some(self.0[i - 1])
                    } else {
                        None
                    }
                }
            }
        }
    }

    /// Returns the smallest prime greater than `n`.  
    /// If `n` is larger than or equal to the largest prime in `self` this returns `None`.
    ///
    /// Uses a binary search.
    ///
    /// # Example
    ///
    /// ```
    /// # use const_primes::Primes;
    /// const CACHE: Primes<100> = Primes::new();
    /// const NEXT: Option<u32> = CACHE.next_prime(400);
    /// assert_eq!(NEXT, Some(401));
    /// ```
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn next_prime(&self, n: Underlying) -> Option<Underlying> {
        match self.binary_search(n) {
            Ok(i) => {
                if i + 1 < self.len() {
                    Some(self.0[i + 1])
                } else {
                    None
                }
            }
            Err(i) => {
                if i < N {
                    Some(self.0[i])
                } else {
                    None
                }
            }
        }
    }

    // endregion: Next prime

    /// Searches the underlying array of primes for the target integer.
    ///
    /// If the target is found it returns a [`Result::Ok`] that contains the index of the matching element.
    /// If the target is not found in the array a [`Result::Err`] is returned that indicates where the
    /// target could be inserted into the array while maintaining the sorted order.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// # use const_primes::Primes;
    /// // [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
    /// const PRIMES: Primes<10> = Primes::new();
    ///
    /// const WHERE_29: Result<usize, usize> = PRIMES.binary_search(29);
    /// const WHERE_6: Result<usize, usize> = PRIMES.binary_search(6);
    /// const WHERE_1000: Result<usize, usize> = PRIMES.binary_search(1_000);
    ///
    /// assert_eq!(WHERE_29, Ok(9));
    /// assert_eq!(WHERE_6, Err(3));
    /// assert_eq!(WHERE_1000, Err(10));
    /// ```
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn binary_search(&self, target: Underlying) -> Result<usize, usize> {
        let mut size = N;
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;
            let candidate = self.0[mid];
            if candidate < target {
                left = mid + 1;
            } else if candidate > target {
                right = mid;
            } else {
                return Ok(mid);
            }
            size = right - left;
        }
        Err(left)
    }

    // region: Conversions

    /// Converts `self` into an array of size `N`.
    ///
    /// # Example
    ///
    /// Basic usage
    /// ```
    /// # use const_primes::Primes;
    /// const PRIMES: [u32; 5] = Primes::new().into_array();
    /// assert_eq!(PRIMES, [2, 3, 5, 7, 11]);
    /// ```
    #[inline]
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn into_array(self) -> [Underlying; N] {
        self.0
    }

    /// Returns a reference to the underlying array.
    #[inline]
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn as_array(&self) -> &[Underlying; N] {
        &self.0
    }

    /// Returns a slice that contains the entire underlying array.
    #[inline]
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn as_slice(&self) -> &[Underlying] {
        self.0.as_slice()
    }

    /// Returns a borrowing iterator over the primes.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// # use const_primes::Primes;
    /// const PRIMES: Primes<10> = Primes::new();
    ///
    /// let mut primes = PRIMES.iter();
    ///
    /// assert_eq!(primes.nth(5), Some(&13));
    /// assert_eq!(primes.next(), Some(&17));
    /// assert_eq!(primes.as_slice(), &[19, 23, 29]);
    /// ```
    #[inline]
    pub fn iter(&self) -> PrimesIter<'_> {
        PrimesIter::new(IntoIterator::into_iter(&self.0))
    }

    // endregion: Conversions

    /// Returns a reference to the element at the given index if it is within bounds.
    ///
    /// # Example
    ///
    /// Basic usage
    /// ```
    /// # use const_primes::Primes;
    /// const PRIMES: Primes<5> = Primes::new();
    /// const THIRD_PRIME: Option<&u32> = PRIMES.get(2);
    /// assert_eq!(THIRD_PRIME, Some(&5));
    /// ```
    #[inline]
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn get(&self, index: usize) -> Option<&Underlying> {
        if index < N {
            Some(&self.0[index])
        } else {
            None
        }
    }

    /// Returns a reference to the last prime in `self`. This is also the largest prime in `self`.
    ///
    /// # Example
    ///
    /// Basic usage
    /// ```
    /// # use const_primes::Primes;
    /// const PRIMES: Primes<5> = Primes::new();
    /// assert_eq!(PRIMES.last(), &11);
    /// ```
    #[inline]
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn last(&self) -> &Underlying {
        match self.0.last() {
            Some(l) => l,
            None => panic!("unreachable: an empty `Primes<N>` can not be created"),
        }
    }

    /// Returns the number of primes in `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use const_primes::Primes;
    /// const PRIMES: Primes<5> = Primes::new();
    /// assert_eq!(PRIMES.len(), 5);
    /// ```
    #[inline]
    #[must_use = "the method only returns a new value and does not modify `self`"]
    // Can never be empty since we panic if the user tries to create an empty `Primes`.
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns the value of the Euler totient function of `n`:
    /// the number of positive integers up to `n` that are relatively prime to it.
    ///
    /// # Errors
    ///
    /// The totient function is computed here as the product over all factors of the form p^(k-1)*(p-1) where
    /// p is the primes in the prime factorization of `n` and k is their multiplicity.
    /// If `n` contains prime factors that are not part of `self`, a [`Result::Err`] is returned
    /// that contains a [`PartialTotient`] struct that contains the result from using only the primes in `self`,
    /// as well as the product of the prime factors that are not included in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use const_primes::{Primes, cache::PartialTotient};
    /// const CACHE: Primes<3> = Primes::new();
    /// const TOTIENT_OF_6: Result<u32, PartialTotient> = CACHE.totient(2*3);
    ///
    /// assert_eq!(TOTIENT_OF_6, Ok(2));
    /// ```
    /// The number 2450 is equal to 2\*5\*5\*7\*7, but the cache does not contain 7.
    /// This means that the function runs out of primes after 5, and can not finish the computation:
    /// ```
    /// # use const_primes::{Primes, cache::PartialTotient};
    /// # const CACHE: Primes<3> = Primes::new();
    /// const TOTIENT_OF_2450: Result<u32, PartialTotient> = CACHE.totient(2*5*5*7*7);
    ///
    /// assert_eq!(
    ///     TOTIENT_OF_2450,
    ///     Err( PartialTotient {
    /// //                 totient(2*5*5) = 20
    ///         totient_using_known_primes: 20,
    ///         product_of_unknown_prime_factors: 49
    ///     })
    /// );
    /// ```
    pub const fn totient(&self, mut n: Underlying) -> Result<Underlying, PartialTotient> {
        if n == 0 {
            return Ok(0);
        }

        let mut i = 0;
        let mut ans = 1;
        while let Some(&prime) = self.get(i) {
            let mut count = 0;
            while n % prime == 0 {
                n /= prime;
                count += 1;
            }

            if count > 0 {
                ans *= prime.pow(count - 1) * (prime - 1);
            }

            if n == 1 {
                break;
            }
            i += 1;
        }

        if n == 1 {
            Ok(ans)
        } else {
            Err(PartialTotient {
                totient_using_known_primes: ans,
                product_of_unknown_prime_factors: n,
            })
        }
    }
}

/// Contains the result of a partially successful evaluation of the [`totient`](Primes::totient) function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PartialTotient {
    /// The result of computing the totient function with only the primes in the related [`Primes`] struct.
    pub totient_using_known_primes: Underlying,
    /// The product of all remaining prime factors of the number.
    pub product_of_unknown_prime_factors: Underlying,
}

impl<const N: usize> Default for Primes<N> {
    /// Panics if `N` is 0. This is a compile error if the `const_assert` feature is enabled.
    fn default() -> Self {
        #[cfg(feature = "const_assert")]
        inline_const!(assert!(N > 0, "`N` must be at least 1"));
        #[cfg(not(feature = "const_assert"))]
        assert!(N > 0, "`N` must be at least 1");

        Self(primes())
    }
}

impl<const N: usize, I> core::ops::Index<I> for Primes<N>
where
    I: core::slice::SliceIndex<[Underlying]>,
{
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}

impl<const N: usize> From<Primes<N>> for [Underlying; N] {
    #[inline]
    fn from(const_primes: Primes<N>) -> Self {
        const_primes.0
    }
}

// region: AsRef

impl<const N: usize> AsRef<[Underlying]> for Primes<N> {
    #[inline]
    fn as_ref(&self) -> &[Underlying] {
        &self.0
    }
}

impl<const N: usize> AsRef<[Underlying; N]> for Primes<N> {
    #[inline]
    fn as_ref(&self) -> &[Underlying; N] {
        &self.0
    }
}

// endregion: AsRef

// region: IntoIterator

impl<const N: usize> IntoIterator for Primes<N> {
    type Item = Underlying;
    type IntoIter = PrimesIntoIter<N>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        PrimesIntoIter::new(self.0.into_iter())
    }
}

impl<'a, const N: usize> IntoIterator for &'a Primes<N> {
    type IntoIter = PrimesIter<'a>;
    type Item = &'a Underlying;
    fn into_iter(self) -> Self::IntoIter {
        PrimesIter::new(IntoIterator::into_iter(&self.0))
    }
}

// endregion: IntoIterator

// region: PartialEq

impl<const N: usize, T: PartialEq<[Underlying; N]>> PartialEq<T> for Primes<N> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        other == &self.0
    }
}

impl<const N: usize> PartialEq<Primes<N>> for [Underlying; N] {
    #[inline]
    fn eq(&self, other: &Primes<N>) -> bool {
        self == &other.0
    }
}

impl<const N: usize> PartialEq<Primes<N>> for &[Underlying] {
    #[inline]
    fn eq(&self, other: &Primes<N>) -> bool {
        self == &other.0
    }
}

impl<const N: usize> PartialEq<[Underlying]> for Primes<N> {
    #[inline]
    fn eq(&self, other: &[Underlying]) -> bool {
        self.0 == other
    }
}

// endregion: PartialEq

// region: PartialOrd

use core::cmp::Ordering;
impl<const N: usize, T: PartialOrd<[Underlying; N]>> PartialOrd<T> for Primes<N> {
    #[inline]
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        other.partial_cmp(&self.0)
    }
}

impl<const N: usize> PartialOrd<Primes<N>> for [Underlying; N] {
    #[inline]
    fn partial_cmp(&self, other: &Primes<N>) -> Option<Ordering> {
        other.0.partial_cmp(self)
    }
}

impl<const N: usize> PartialOrd<Primes<N>> for &[Underlying] {
    #[inline]
    fn partial_cmp(&self, other: &Primes<N>) -> Option<Ordering> {
        other.0.as_slice().partial_cmp(self)
    }
}

// endregion: PartialOrd

// endregion: Primes<N>

#[cfg(test)]
mod test {
    use crate::next_prime;

    use super::*;

    // region: TraitImpls

    #[test]
    fn partial_eq_impl() {
        const P1: Primes<3> = Primes::new();
        macro_rules! partial_eq_check {
            ($($t:ident),+) => {
                $(
                    assert_eq!(P1, $t);
                    assert_eq!(&P1, &$t);
                    assert_eq!(&$t, &P1);
                    assert_eq!($t, P1);
                )+
            };
        }
        let v = [2, 3, 5];
        let s = v.as_slice();
        partial_eq_check!(v, s);
    }

    #[test]
    fn verify_impl_from_primes_traits() {
        const N: usize = 10;
        const P: Primes<N> = Primes::new();
        let p: [Underlying; N] = P.into();
        assert_eq!(P, p);
        assert_eq!(p, P.as_ref());
        assert_eq!(
            P.as_array(),
            <Primes<N> as AsRef<[Underlying; N]>>::as_ref(&P)
        );
    }

    #[test]
    fn check_into_iter() {
        const P: Primes<10> = Primes::new();
        for (i, prime) in P.into_iter().enumerate() {
            assert_eq!(prime, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29][i]);
        }
    }

    // endregion: TraitImpls

    #[test]
    fn check_binary_search() {
        const CACHE: Primes<100> = Primes::new();
        type BSResult = Result<usize, usize>;
        const FOUND2: BSResult = CACHE.binary_search(2);
        const INSERT0: BSResult = CACHE.binary_search(0);
        const INSERT4: BSResult = CACHE.binary_search(4);
        const FOUND541: BSResult = CACHE.binary_search(541);
        const NOINFO542: BSResult = CACHE.binary_search(542);
        const BIG: BSResult = CACHE.binary_search(1000000);
        assert_eq!(FOUND2, Ok(0));
        assert_eq!(INSERT0, Err(0));
        assert_eq!(INSERT4, Err(2));
        assert_eq!(FOUND541, Ok(99));
        assert_eq!(NOINFO542, Err(100));
        assert_eq!(BIG, Err(100));
    }

    #[test]
    fn test_into_iter() {
        const PRIMES: Primes<10> = Primes::new();

        for (&prime, ans) in (&PRIMES)
            .into_iter()
            .zip([2, 3, 5, 7, 11, 13, 17, 19, 23, 29])
        {
            assert_eq!(prime, ans);
        }
    }

    #[test]
    fn check_previous_prime() {
        const CACHE: Primes<100> = Primes::new();
        const PREV0: Option<Underlying> = CACHE.previous_prime(0);
        const PREV400: Option<Underlying> = CACHE.previous_prime(400);
        const PREV541: Option<Underlying> = CACHE.previous_prime(541);
        const PREV542: Option<Underlying> = CACHE.previous_prime(542);
        const PREVS: [Underlying; 18] = [
            2, 3, 3, 5, 5, 7, 7, 7, 7, 11, 11, 13, 13, 13, 13, 17, 17, 19,
        ];
        for (i, prev) in PREVS.into_iter().enumerate() {
            assert_eq!(Some(prev), CACHE.previous_prime(i as u32 + 3));
        }
        assert_eq!(PREV0, None);
        assert_eq!(PREV400, Some(397));
        assert_eq!(PREV541, Some(523));
        assert_eq!(PREV542, None);
    }

    #[test]
    fn check_prime_factorization() {
        const CACHE: Primes<3> = Primes::new();

        let mut factorization_of_14 = CACHE.prime_factorization(14);

        assert_eq!(factorization_of_14.next(), Some((2, 1)));
        assert_eq!(factorization_of_14.next(), None);
        assert_eq!(factorization_of_14.remainder(), Some(7));

        let mut factorization_of_15 = CACHE.prime_factorization(15);

        assert_eq!(factorization_of_15.next(), Some((3, 1)));
        assert_eq!(factorization_of_15.next(), Some((5, 1)));
        assert!(factorization_of_15.remainder().is_none());

        let mut factorization_of_270 = CACHE.prime_factorization(2 * 3 * 3 * 3 * 5);
        assert_eq!(factorization_of_270.next(), Some((2, 1)));
        assert_eq!(factorization_of_270.next(), Some((3, 3)));
        assert_eq!(factorization_of_270.next(), Some((5, 1)));
    }

    #[test]
    fn check_prime_factors() {
        const CACHE: Primes<3> = Primes::new();

        let mut factors_of_14 = CACHE.prime_factors(14);

        assert_eq!(factors_of_14.next(), Some(2));
        assert_eq!(factors_of_14.next(), None);
        assert_eq!(factors_of_14.remainder(), Some(7));

        let mut factors_of_15 = CACHE.prime_factors(15);
        assert_eq!(factors_of_15.next(), Some(3));
        assert_eq!(factors_of_15.next(), Some(5));
        assert!(factors_of_15.remainder().is_none());

        let mut factors_of_270 = CACHE.prime_factors(2 * 3 * 3 * 3 * 5);
        assert_eq!(factors_of_270.next(), Some(2));
        assert_eq!(factors_of_270.next(), Some(3));
        assert_eq!(factors_of_270.next(), Some(5));
    }

    #[test]
    fn check_next_prime() {
        const CACHE: Primes<100> = Primes::new();
        const SPGEQ0: Option<Underlying> = CACHE.next_prime(0);
        const SPGEQ400: Option<Underlying> = CACHE.next_prime(400);
        const SPGEQ541: Option<Underlying> = CACHE.next_prime(540);
        const SPGEQ542: Option<Underlying> = CACHE.next_prime(541);
        assert_eq!(SPGEQ0, Some(2));
        assert_eq!(SPGEQ400, Some(401));
        assert_eq!(SPGEQ541, Some(541));
        assert_eq!(SPGEQ542, None);

        const N: usize = 31;
        const NEXT_PRIME: [u32; N] = [
            2, 2, 3, 5, 5, 7, 7, 11, 11, 11, 11, 13, 13, 17, 17, 17, 17, 19, 19, 23, 23, 23, 23,
            29, 29, 29, 29, 29, 29, 31, 31,
        ];
        const P: Primes<N> = Primes::new();

        for (n, next) in NEXT_PRIME.iter().enumerate().take(N) {
            assert_eq!(P.next_prime(n as u32), Some(*next));
        }
    }

    #[test]
    fn verify_into_array() {
        const N: usize = 10;
        const P: Primes<N> = Primes::new();
        const A: [Underlying; N] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert_eq!(P.into_array(), A);
    }

    #[test]
    fn verify_as_slice() {
        const N: usize = 10;
        const P: Primes<N> = Primes::new();
        const A: [Underlying; N] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert_eq!(P.as_slice(), &A);
    }

    #[test]
    fn verify_as_array() {
        const N: usize = 10;
        const P: Primes<N> = Primes::new();
        const A: [Underlying; N] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert_eq!(P.as_array(), &A);
    }

    #[test]
    fn check_get() {
        const N: usize = 10;
        const P: Primes<N> = Primes::new();
        const A: [Underlying; N] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for (n, gotten) in A.iter().enumerate().take(N) {
            assert_eq!(P.get(n), Some(gotten));
        }
        for n in N + 1..2 * N {
            assert!(P.get(n).is_none());
        }
    }

    #[test]
    fn check_last_and_len() {
        const PRIMES: [Underlying; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        macro_rules! check_last_n {
            ($($n:literal),+) => {
                $(
                    {
                        let p: Primes<$n> = Primes::new();
                        assert_eq!(*p.last(), PRIMES[$n - 1]);
                        assert_eq!(p.len(), $n);
                        assert_eq!(*p.last(), p[$n - 1]);
                    }
                )+
            };
        }
        check_last_n!(1, 2, 3, 4, 5, 6, 7, 8, 9);
    }

    #[test]
    fn check_count_primes_leq() {
        const N: usize = 79;
        const PRIME_COUNTS: [usize; N] = [
            0, 0, 1, 2, 2, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9,
            10, 10, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 13, 13, 14, 14, 14, 14, 15, 15, 15, 15,
            15, 15, 16, 16, 16, 16, 16, 16, 17, 17, 18, 18, 18, 18, 18, 18, 19, 19, 19, 19, 20, 20,
            21, 21, 21, 21, 21, 21,
        ];
        const P: Primes<N> = Primes::new();

        for (n, count) in PRIME_COUNTS.iter().enumerate().take(N) {
            assert_eq!(P.count_primes_leq(n as u32), Some(*count));
        }

        for n in *P.last() + 1..*P.last() * 2 {
            assert!(P.count_primes_leq(n).is_none());
        }
    }

    #[test]
    fn check_iter() {
        const P: Primes<10> = Primes::new();
        for (p1, p2) in P.iter().zip([2, 3, 5, 7, 11, 13, 17, 19, 23, 29].iter()) {
            assert_eq!(p1, p2);
        }
    }

    #[test]
    fn check_totient() {
        const TOTIENTS: [Underlying; 101] = [
            0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20,
            12, 18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18, 24, 16, 40, 12, 42, 20, 24, 22, 46,
            16, 42, 20, 32, 24, 52, 18, 40, 24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66, 32, 44,
            24, 70, 24, 72, 36, 40, 36, 60, 24, 78, 32, 54, 40, 82, 24, 64, 42, 56, 40, 88, 24, 72,
            44, 60, 46, 72, 32, 96, 42, 60, 40,
        ];
        const NEXT_OUTSIDE: Underlying = match next_prime(*BIG_CACHE.last() as u64) {
            Some(np) => np as Underlying,
            None => panic!(),
        };

        const SMALL_CACHE: Primes<3> = Primes::new();
        const BIG_CACHE: Primes<100> = Primes::new();

        assert_eq!(SMALL_CACHE.totient(6), Ok(2));
        assert_eq!(
            SMALL_CACHE.totient(2 * 5 * 5 * 7 * 7),
            Err(PartialTotient {
                totient_using_known_primes: 20,
                product_of_unknown_prime_factors: 49
            })
        );

        for (i, totient) in TOTIENTS.into_iter().enumerate() {
            assert_eq!(BIG_CACHE.totient(i as Underlying), Ok(totient));
            if i != 0 {
                assert_eq!(
                    BIG_CACHE.totient((i as Underlying) * NEXT_OUTSIDE),
                    Err(PartialTotient {
                        totient_using_known_primes: totient,
                        product_of_unknown_prime_factors: NEXT_OUTSIDE
                    })
                );
            }
        }
    }
}
