use core::iter::FusedIterator;

use super::Underlying;

/// An owning iterator over prime numbers.
///
/// Created by the [`IntoIterator`] implementation on [`Primes`](super::Primes).
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
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
