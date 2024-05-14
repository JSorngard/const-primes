//! This module contains the implementation of the type [`Primes`], which functions as a cache of prime numbers
//! for related computations.
//!
//! It also contains the implementation of related iterators.

use crate::{primes, Underlying};

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
pub struct Primes<const N: usize> {
    primes: [Underlying; N],
}

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
    /// Panics if `N` is zero. In const contexts this is a compile error.
    /// ```compile_fail
    /// # use const_primes::Primes;
    /// const NO_PRIMES: Primes<0> = Primes::new();
    /// ```
    ///
    /// If any of the primes overflow a `u32` it will panic in const contexts or debug mode.
    #[must_use = "the associated method only returns a new value"]
    pub const fn new() -> Self {
        assert!(N > 0, "`N` must be at least 1");
        Self { primes: primes() }
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
            Err(Some(_)) => Some(false),
            Err(None) => None,
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
            Err(maybe_i) => match maybe_i {
                Some(i) => Some(i),
                None => None,
            },
        }
    }

    // region: Next prime

    /// Returns the largest prime less than `n`.  
    /// If `n` is 0, 1, 2, or larger than the largest prime in `self` this returns `None`.
    ///
    /// Uses a binary search.
    /// # Example
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
                Ok(i) | Err(Some(i)) => {
                    if i > 0 {
                        Some(self.primes[i - 1])
                    } else {
                        None
                    }
                }
                Err(None) => None,
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
                    Some(self.primes[i + 1])
                } else {
                    None
                }
            }
            Err(Some(i)) => Some(self.primes[i]),
            Err(None) => None,
        }
    }

    // endregion: Next prime

    /// Searches the underlying array of primes for the target integer.
    /// If the target is found it returns a [`Result::Ok`] that contains the index of the matching element.
    /// If the target is not found in the array a [`Result::Err`] is returned that contains an [`Option`].   
    /// If the target could be inserted into the array while maintaining the sorted order, the [`Option::Some`]
    /// variant is returned and contains the index of that location.
    /// If the target is larger than the largest prime in the array no information about where it might fit is available,
    /// and an [`Option::None`] is returned.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// # use const_primes::Primes;
    /// const PRIMES: Primes<10> = Primes::new();
    /// const WHERE_29: Result<usize, Option<usize>> = PRIMES.binary_search(29);
    /// const WHERE_6: Result<usize, Option<usize>> = PRIMES.binary_search(6);
    /// const WHERE_1000: Result<usize, Option<usize>> = PRIMES.binary_search(1_000);
    ///
    /// assert_eq!(WHERE_29, Ok(9));
    /// assert_eq!(WHERE_6, Err(Some(3)));
    /// assert_eq!(WHERE_1000, Err(None));
    /// ```
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn binary_search(&self, target: Underlying) -> Result<usize, Option<usize>> {
        if target > *self.last() {
            Err(None)
        } else {
            let mut size = N;
            let mut left = 0;
            let mut right = size;
            while left < right {
                let mid = left + size / 2;
                let candidate = self.primes[mid];
                if candidate < target {
                    left = mid + 1;
                } else if candidate > target {
                    right = mid;
                } else {
                    return Ok(mid);
                }
                size = right - left;
            }
            Err(Some(left))
        }
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
        self.primes
    }

    /// Returns a reference to the underlying array.
    #[inline]
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn as_array(&self) -> &[Underlying; N] {
        &self.primes
    }

    /// Returns a slice that contains the entire underlying array.
    #[inline]
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn as_slice(&self) -> &[Underlying] {
        self.primes.as_slice()
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
    pub fn iter(&self) -> PrimesIter<'_, N> {
        PrimesIter::new(IntoIterator::into_iter(&self.primes))
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
    /// assert_eq!(PRIMES.get(2), Some(&5));
    /// ```
    #[inline]
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn get(&self, index: usize) -> Option<&Underlying> {
        if index < N {
            Some(&self.primes[index])
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
        match self.primes.last() {
            Some(l) => l,
            None => panic!("this should panic during creation"),
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
}

/// Panics if `N` is 0.
impl<const N: usize> Default for Primes<N> {
    fn default() -> Self {
        assert!(N > 0, "`N` must be at least 1");
        Self { primes: primes() }
    }
}

impl<const N: usize, I> core::ops::Index<I> for Primes<N>
where
    I: core::slice::SliceIndex<[Underlying]>,
{
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.primes.index(index)
    }
}

impl<const N: usize> From<Primes<N>> for [Underlying; N] {
    #[inline]
    fn from(const_primes: Primes<N>) -> Self {
        const_primes.primes
    }
}

// region: AsRef

impl<const N: usize> AsRef<[Underlying]> for Primes<N> {
    #[inline]
    fn as_ref(&self) -> &[Underlying] {
        &self.primes
    }
}

impl<const N: usize> AsRef<[Underlying; N]> for Primes<N> {
    #[inline]
    fn as_ref(&self) -> &[Underlying; N] {
        &self.primes
    }
}

// endregion: AsRef

// region: IntoIterator

impl<const N: usize> IntoIterator for Primes<N> {
    type Item = Underlying;
    type IntoIter = PrimesIntoIter<N>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        PrimesIntoIter::new(self.primes.into_iter())
    }
}

pub use primes_iter::PrimesIter;
mod primes_iter {
    use super::Underlying;
    use core::iter::FusedIterator;

    /// An iterator over prime numbers.
    /// Created by the [`iter`](super::Primes::iter) function on [`Primes`](super::Primes),
    /// see it for more information.
    #[derive(Debug, Clone)]
    pub struct PrimesIter<'a, const N: usize>(core::slice::Iter<'a, Underlying>);

    impl<'a, const N: usize> PrimesIter<'a, N> {
        pub(crate) const fn new(iter: core::slice::Iter<'a, Underlying>) -> Self {
            Self(iter)
        }

        /// Returns an immutable slice of all the primes that have not been yielded yet.
        pub fn as_slice(&self) -> &[Underlying] {
            self.0.as_slice()
        }
    }

    impl<'a, const N: usize> Iterator for PrimesIter<'a, N> {
        type Item = &'a Underlying;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }

        #[inline]
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.0.nth(n)
        }

        #[inline]
        fn count(self) -> usize {
            self.0.count()
        }

        #[inline]
        fn last(self) -> Option<Self::Item> {
            self.0.last()
        }
    }

    impl<'a, const N: usize> ExactSizeIterator for PrimesIter<'a, N> {
        #[inline]
        fn len(&self) -> usize {
            self.0.len()
        }
    }

    impl<'a, const N: usize> FusedIterator for PrimesIter<'a, N> {}

    impl<'a, const N: usize> DoubleEndedIterator for PrimesIter<'a, N> {
        #[inline]
        fn next_back(&mut self) -> Option<Self::Item> {
            self.0.next_back()
        }

        #[inline]
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            self.0.nth_back(n)
        }
    }
}

pub use primes_into_iter::PrimesIntoIter;
mod primes_into_iter {
    use core::iter::FusedIterator;

    use super::Underlying;

    /// An owning iterator over prime numbers.
    /// Created by the [`IntoIterator`] implementation on [`Primes`](super::Primes).
    #[derive(Debug, Clone)]
    pub struct PrimesIntoIter<const N: usize>(core::array::IntoIter<Underlying, N>);

    impl<const N: usize> PrimesIntoIter<N> {
        pub(crate) const fn new(iter: core::array::IntoIter<Underlying, N>) -> Self {
            Self(iter)
        }

        /// Returns an immutable slice of all primes that have not been yielded yet.
        pub fn as_slice(&self) -> &[Underlying] {
            self.0.as_slice()
        }
    }

    impl<const N: usize> Iterator for PrimesIntoIter<N> {
        type Item = Underlying;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }

        #[inline]
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.0.nth(n)
        }

        #[inline]
        fn count(self) -> usize {
            self.0.count()
        }

        #[inline]
        fn last(self) -> Option<Self::Item> {
            self.0.last()
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
    }

    impl<const N: usize> DoubleEndedIterator for PrimesIntoIter<N> {
        #[inline]
        fn next_back(&mut self) -> Option<Self::Item> {
            self.0.next_back()
        }

        #[inline]
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            self.0.nth_back(n)
        }
    }

    impl<const N: usize> FusedIterator for PrimesIntoIter<N> {}

    impl<const N: usize> ExactSizeIterator for PrimesIntoIter<N> {
        #[inline]
        fn len(&self) -> usize {
            self.0.len()
        }
    }
}

impl<'a, const N: usize> IntoIterator for &'a Primes<N> {
    type IntoIter = PrimesIter<'a, N>;
    type Item = &'a Underlying;
    fn into_iter(self) -> Self::IntoIter {
        PrimesIter::new(IntoIterator::into_iter(&self.primes))
    }
}

// endregion: IntoIterator

// region: PartialEq

impl<const N: usize, T: PartialEq<[Underlying; N]>> PartialEq<T> for Primes<N> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        other == &self.primes
    }
}

impl<const N: usize> PartialEq<Primes<N>> for [Underlying; N] {
    #[inline]
    fn eq(&self, other: &Primes<N>) -> bool {
        self == &other.primes
    }
}

impl<const N: usize> PartialEq<Primes<N>> for &[Underlying] {
    #[inline]
    fn eq(&self, other: &Primes<N>) -> bool {
        self == &other.primes
    }
}

impl<const N: usize> PartialEq<[Underlying]> for Primes<N> {
    #[inline]
    fn eq(&self, other: &[Underlying]) -> bool {
        self.primes == other
    }
}

// endregion: PartialEq

// region: PartialOrd

use core::cmp::Ordering;
impl<const N: usize, T: PartialOrd<[Underlying; N]>> PartialOrd<T> for Primes<N> {
    #[inline]
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        other.partial_cmp(&self.primes)
    }
}

impl<const N: usize> PartialOrd<Primes<N>> for [Underlying; N] {
    #[inline]
    fn partial_cmp(&self, other: &Primes<N>) -> Option<Ordering> {
        other.primes.partial_cmp(self)
    }
}

impl<const N: usize> PartialOrd<Primes<N>> for &[Underlying] {
    #[inline]
    fn partial_cmp(&self, other: &Primes<N>) -> Option<Ordering> {
        other.primes.as_slice().partial_cmp(self)
    }
}

// endregion: PartialOrd

// endregion: Primes<N>

#[cfg(test)]
mod test {
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
        type BSResult = Result<usize, Option<usize>>;
        const FOUND2: BSResult = CACHE.binary_search(2);
        const INSERT0: BSResult = CACHE.binary_search(0);
        const INSERT4: BSResult = CACHE.binary_search(4);
        const FOUND541: BSResult = CACHE.binary_search(541);
        const NOINFO542: BSResult = CACHE.binary_search(542);
        assert_eq!(FOUND2, Ok(0));
        assert_eq!(INSERT0, Err(Some(0)));
        assert_eq!(INSERT4, Err(Some(2)));
        assert_eq!(FOUND541, Ok(99));
        assert_eq!(NOINFO542, Err(None));
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
}
