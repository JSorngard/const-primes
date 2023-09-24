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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Primes<const N: usize> {
    primes: [Underlying; N],
}

impl<const N: usize> Primes<N> {
    /// Generates a new instance that contains the first `N` primes.
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
    pub const fn is_prime(&self, n: u32) -> Option<bool> {
        if n > *self.last() {
            return None;
        }

        let mut size = N;
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;
            let candidate = self.primes[mid];
            if candidate < n {
                left = mid + 1;
            } else if candidate > n {
                right = mid;
            } else {
                return Some(true);
            }
            size = right - left;
        }
        Some(false)
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
    #[must_use]
    pub const fn into_array(self) -> [Underlying; N] {
        self.primes
    }

    /// Returns a reference to the underlying array.
    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[Underlying; N] {
        &self.primes
    }

    // Returns a slice that contains the entire underlying array.
    #[inline]
    #[must_use]
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
    #[must_use]
    pub const fn get(&self, index: usize) -> Option<&Underlying> {
        if index < N {
            Some(&self.primes[index])
        } else {
            None
        }
    }

    /// Returns the last prime in `self`. This is also the largest prime in `self`.
    ///
    /// # Example
    /// Basic usage
    /// ```
    /// # use const_primes::Primes;
    /// const PRIMES: Primes<5> = Primes::new();
    /// assert_eq!(PRIMES.last(), &11);
    /// ```
    #[inline]
    #[must_use]
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
    // Can never be empty since we panic if the user tries to create an empty `Primes`.
    #[inline]
    #[must_use]
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

impl<const N: usize> PartialEq<[Underlying; N]> for Primes<N> {
    fn eq(&self, other: &[Underlying; N]) -> bool {
        &self.primes == other
    }
}

impl<const N: usize> PartialEq<Primes<N>> for [Underlying; N] {
    fn eq(&self, other: &Primes<N>) -> bool {
        self == &other.primes
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

impl<const N: usize> IntoIterator for Primes<N> {
    type Item = <[Underlying; N] as IntoIterator>::Item;
    type IntoIter = <[Underlying; N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.primes.into_iter()
    }
}
