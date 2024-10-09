use super::Underlying;
use core::iter::FusedIterator;

/// A borrowing iterator over prime numbers.
///
/// Created by the [`iter`](super::Primes::iter) function on [`Primes`](super::Primes),
/// see it for more information.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "zerocopy",
    derive(zerocopy::IntoBytes, zerocopy::Immutable, zerocopy::KnownLayout)
)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[cfg_attr(feature = "zerocopy", repr(transparent))]
pub struct PrimesIter<'a>(core::slice::Iter<'a, Underlying>);

impl<'a> PrimesIter<'a> {
    pub(crate) const fn new(iter: core::slice::Iter<'a, Underlying>) -> Self {
        Self(iter)
    }

    /// Returns an immutable slice of all the primes that have not been yielded yet.
    pub fn as_slice(&self) -> &[Underlying] {
        self.0.as_slice()
    }
}

impl<'a> Iterator for PrimesIter<'a> {
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

impl<'a> ExactSizeIterator for PrimesIter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> FusedIterator for PrimesIter<'a> {}

impl<'a> DoubleEndedIterator for PrimesIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }
}
