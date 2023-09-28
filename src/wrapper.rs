use crate::{primes, Underlying};

/// A wrapper around an array that consists of the first `N` primes.
/// Can be created in const contexts, and if so it ensures that `N` is non-zero at compile time.
///
/// # Examples
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
/// assert_eq!(PRIME_CHECK, Some(true));
/// assert_eq!(PRIME_COUNT, Some(46));
/// // If questions are asked about numbers outside the cache it returns None
/// assert!(CACHE.is_prime(1000).is_none());
/// assert!(CACHE.count_primes_leq(1000).is_none());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Primes<const N: usize> {
    primes: [Underlying; N],
}

impl<const N: usize> Primes<N> {
    /// Generates a new instance that contains the first `N` primes.
    ///
    /// Uses a [segmented sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve).
    ///
    /// # Example
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
    /// # Panics
    /// Panics if `N` is zero. In const contexts this will fail to compile
    /// ```compile_fail
    /// # use const_primes::Primes;
    /// const NO_PRIMES: Primes<0> = Primes::new();
    /// ```
    /// In other contexts it may panic at runtime instead.  
    /// If any of the primes overflow a `u32` it will panic in const contexts or debug mode.
    #[must_use = "the associated method only returns a new value"]
    pub const fn new() -> Self {
        assert!(N >= 1, "`N` must be at least 1");

        Self { primes: primes() }
    }

    /// Returns whether `n` is prime if it is smaller than or equal to the largest prime in `self`.
    ///
    /// Uses a binary search.
    ///
    /// # Example
    /// Basic usage
    /// ```
    /// # use const_primes::Primes;
    /// const PRIMES: Primes<100> = Primes::new();
    /// const TMOLTUAE: Option<bool> = PRIMES.is_prime(42);
    /// assert_eq!(PRIMES.is_prime(13), Some(true));
    /// assert_eq!(TMOLTUAE, Some(false));
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
    /// Uses a linear search to count the primes.
    /// # Example
    /// Basic usage
    /// ```
    /// # use const_primes::Primes;
    /// const CACHE: Primes<100> = Primes::new();
    /// const COUNT: Option<usize> = CACHE.count_primes_leq(500);
    /// const OUT_OF_BOUNDS: Option<usize> = CACHE.count_primes_leq(1_000);
    /// assert_eq!(COUNT, Some(95));
    /// assert_eq!(OUT_OF_BOUNDS, None);
    /// ```
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn count_primes_leq(&self, n: Underlying) -> Option<usize> {
        if n > *self.last() {
            return None;
        }

        let mut i = 0;
        let mut count = 0;
        while i < N {
            if self.primes[i] <= n {
                count += 1;
            } else {
                break;
            }
            i += 1;
        }
        Some(count)
    }

    /// Returns the largest prime less than or equal to `n`.  
    /// If `n` is 0, 1, or larger than the largest prime in `self` this returns `None`.
    ///
    /// Uses a binary search.
    /// # Example
    /// ```
    /// # use const_primes::Primes;
    /// const CACHE: Primes<100> = Primes::new();
    /// const LPLEQ400: Option<u32> = CACHE.largest_prime_leq(400);
    /// assert_eq!(LPLEQ400, Some(397));
    /// ```
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn largest_prime_leq(&self, n: Underlying) -> Option<Underlying> {
        if n <= 1 {
            None
        } else {
            match self.binary_search(n) {
                Ok(i) => Some(self.primes[i]),
                Err(Some(i)) => Some(self.primes[i - 1]),
                Err(None) => None,
            }
        }
    }

    /// Returns the smallest prime greater than or equal to `n`.  
    /// If `n` is larger than the largest prime in `self` this returns `None`.
    ///
    /// Uses a binary search.
    /// # Example
    /// ```
    /// # use const_primes::Primes;
    /// const CACHE: Primes<100> = Primes::new();
    /// const SPGEQ: Option<u32> = CACHE.smallest_prime_geq(400);
    /// assert_eq!(SPGEQ, Some(401));
    /// ```
    #[must_use = "the method only returns a new value and does not modify `self`"]
    pub const fn smallest_prime_geq(&self, n: Underlying) -> Option<Underlying> {
        match self.binary_search(n) {
            Ok(i) | Err(Some(i)) => Some(self.primes[i]),
            Err(None) => None,
        }
    }

    /// Searches the underlying array of primes for the target integer.
    /// If the target is found it returns a [`Result::Ok`] that contains the index of the matching element.
    /// If the target is not found in the array a [`Result::Err`] is returned that contains an [`Option`].   
    /// If the target could be inserted into the array while maintaining the sorted order, the [`Some`](Option::Some)
    /// variant contains the index of that location.
    /// If the target is larger than the largest prime in the array no information about where it might fit is available,
    /// and a [`None`](Option::None) is returned.
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

    /// Converts `self` into an array of size `N`.
    ///
    /// # Example
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

    /// Returns a reference to the element at the given index if it is within bounds.
    ///
    /// # Example
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

impl<const N: usize, I> core::ops::Index<I> for Primes<N>
where
    I: core::slice::SliceIndex<[Underlying]>,
{
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.primes[index]
    }
}

impl<const N: usize> From<Primes<N>> for [Underlying; N] {
    #[inline]
    fn from(const_primes: Primes<N>) -> Self {
        const_primes.primes
    }
}

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

impl<const N: usize> IntoIterator for Primes<N> {
    type Item = <[Underlying; N] as IntoIterator>::Item;
    type IntoIter = <[Underlying; N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.primes.into_iter()
    }
}

impl<const N: usize, T: PartialEq<[Underlying; N]>> PartialEq<T> for Primes<N> {
    fn eq(&self, other: &T) -> bool {
        other == &self.primes
    }
}

impl<const N: usize> PartialEq<Primes<N>> for [Underlying; N] {
    fn eq(&self, other: &Primes<N>) -> bool {
        self == &other.primes
    }
}

impl<const N: usize> PartialEq<Primes<N>> for &[Underlying] {
    fn eq(&self, other: &Primes<N>) -> bool {
        self == &other.primes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ensure_partial_eq_is_implemented() {
        const P1: Primes<3> = Primes::new();
        macro_rules! partial_eq_check {
            ($($t:expr),+) => {
                $(
                    assert_eq!(P1, $t);
                    assert_eq!(&P1, &$t);
                    assert_eq!(&$t, &P1);
                    assert_eq!($t, P1);
                )+
            };
        }
        let v = vec![2, 3, 5];
        partial_eq_check!([2, 3, 5], v.as_slice());
    }

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
    fn check_largest_prime_leq() {
        const CACHE: Primes<100> = Primes::new();
        const LPLEQ0: Option<Underlying> = CACHE.largest_prime_leq(0);
        const LPLEQ400: Option<Underlying> = CACHE.largest_prime_leq(400);
        const LPLEQ541: Option<Underlying> = CACHE.largest_prime_leq(541);
        const LPLEQ542: Option<Underlying> = CACHE.largest_prime_leq(542);
        assert_eq!(LPLEQ0, None);
        assert_eq!(LPLEQ400, Some(397));
        assert_eq!(LPLEQ541, Some(541));
        assert_eq!(LPLEQ542, None);
    }

    #[test]
    fn check_smallest_prime_geq() {
        const CACHE: Primes<100> = Primes::new();
        const SPGEQ0: Option<Underlying> = CACHE.smallest_prime_geq(0);
        const SPGEQ400: Option<Underlying> = CACHE.smallest_prime_geq(400);
        const SPGEQ541: Option<Underlying> = CACHE.smallest_prime_geq(541);
        const SPGEQ542: Option<Underlying> = CACHE.smallest_prime_geq(542);
        assert_eq!(SPGEQ0, Some(2));
        assert_eq!(SPGEQ400, Some(401));
        assert_eq!(SPGEQ541, Some(541));
        assert_eq!(SPGEQ542, None);
    }
}
