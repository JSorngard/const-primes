use core::iter::FusedIterator;

use crate::restricted_array::RestrictedArray;

/// An enum describing whether the requested array could be filled completely, or only a partially.
/// A partial array can be returned by [`primes_lt`](crate::generation::primes_lt) if the size of the requested
/// array is larger than the actual number of primes less than the given `upper_limit`.
/// It can also be returned by [`primes_geq`](crate::generation::primes_geq) if it needs to sieve into a
/// region of numbers that exceed the square of the size of the requested array.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SegmentedGenerationResult<const N: usize> {
    /// The complete range could be sieved, and the entire output array contains prime numbers.
    Complete([u64; N]),
    /// Only a part of the range could be sieved before the primes either exceeded `N^2` or ran out,
    /// and so only a part of the output array contains prime numbers.
    Partial(RestrictedArray<u64, N>),
}

// region: enum convenience method impls
impl<const N: usize> SegmentedGenerationResult<N> {
    /// Returns the complete array, if there is one.
    pub const fn complete(self) -> Option<[u64; N]> {
        match self {
            Self::Complete(array) => Some(array),
            _ => None,
        }
    }

    /// Returns the restriced array, if there is one.
    pub const fn partial(self) -> Option<RestrictedArray<u64, N>> {
        match self {
            Self::Partial(restricted_array) => Some(restricted_array),
            _ => None,
        }
    }

    /// Returns `true` if this is the `Complete` variant.
    pub const fn is_complete(&self) -> bool {
        match self {
            Self::Complete(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if this is the `Partial` variant.
    pub const fn is_partial(&self) -> bool {
        match self {
            Self::Partial(_) => true,
            _ => false,
        }
    }
}
// endregion: enum convenience method impls

// region: IntoIterator impl
/// An iterator created by the [`IntoIterator`] impl on [`SegmentedGenerationResult`].
pub enum SegmentedGenerationResultIntoIter<const N: usize> {
    Complete(<[u64; N] as IntoIterator>::IntoIter),
    Partial(<RestrictedArray<u64, N> as IntoIterator>::IntoIter),
}

impl<const N: usize> Iterator for SegmentedGenerationResultIntoIter<N> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Complete(array_iter) => array_iter.next(),
            Self::Partial(restricted_array_iter) => restricted_array_iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Complete(array_iter) => array_iter.size_hint(),
            Self::Partial(restricted_array_iter) => restricted_array_iter.size_hint(),
        }
    }
}

impl<const N: usize> DoubleEndedIterator for SegmentedGenerationResultIntoIter<N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Complete(array_iter) => array_iter.next_back(),
            Self::Partial(restricted_array_iter) => restricted_array_iter.next_back(),
        }
    }
}

impl<const N: usize> FusedIterator for SegmentedGenerationResultIntoIter<N> {}
impl<const N: usize> ExactSizeIterator for SegmentedGenerationResultIntoIter<N> {}

impl<const N: usize> IntoIterator for SegmentedGenerationResult<N> {
    type IntoIter = SegmentedGenerationResultIntoIter<N>;
    type Item = u64;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Complete(array) => SegmentedGenerationResultIntoIter::Complete(array.into_iter()),
            Self::Partial(restricted_array) => {
                SegmentedGenerationResultIntoIter::Partial(restricted_array.into_iter())
            }
        }
    }
}
// endregion: IntoIterator impl

// region: PartialEq impls
impl<const N: usize, T> PartialEq<T> for SegmentedGenerationResult<N>
where
    T: PartialEq<[u64]>,
{
    fn eq(&self, other: &T) -> bool {
        match self {
            Self::Complete(array) => other == array.as_ref(),
            Self::Partial(restricted_array) => other == restricted_array.as_slice(),
        }
    }
}

impl<const N: usize, T> PartialEq<SegmentedGenerationResult<N>> for [T]
where
    T: PartialEq<u64>,
{
    fn eq(&self, other: &SegmentedGenerationResult<N>) -> bool {
        match other {
            SegmentedGenerationResult::Complete(array) => self == array,
            SegmentedGenerationResult::Partial(restricted_array) => restricted_array == self,
        }
    }
}
// endregion: PartialEq impls

impl<const N: usize> AsRef<[u64]> for SegmentedGenerationResult<N> {
    fn as_ref(&self) -> &[u64] {
        match self {
            Self::Complete(array) => array.as_slice(),
            Self::Partial(restricted_array) => restricted_array.as_slice(),
        }
    }
}

impl<const N: usize> core::ops::Index<usize> for SegmentedGenerationResult<N> {
    type Output = u64;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Complete(array) => &array[index],
            Self::Partial(restricted_array) => &restricted_array[index],
        }
    }
}