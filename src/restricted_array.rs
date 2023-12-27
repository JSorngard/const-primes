use core::{iter::FusedIterator, ops::Range};

/// An array where only a section of the data may be viewed,
/// as the other data may e.g. not uphold some invariant.  
/// When this type is compared against some other type, only
/// data in the visible part is compared.
#[derive(Debug, Clone, Copy)]
pub struct RestrictedArray<T, const N: usize> {
    start: usize,
    end: usize,
    array: [T; N],
}

impl<const N: usize, T: PartialEq<T>> PartialEq<RestrictedArray<T, N>> for RestrictedArray<T, N> {
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.  
    /// Only compares the *visible* part of `self` against the *visible* part of `other`.
    fn eq(&self, other: &RestrictedArray<T, N>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<const N: usize, T: Eq> Eq for RestrictedArray<T, N> {}

impl<const N: usize, T> RestrictedArray<T, N> {
    /// Restrict an array so that only elements within the given range are visible.
    ///
    /// # Panics
    /// Panics if the range of indices is out of bounds of the array.
    pub const fn new(sub_range: Range<usize>, array: [T; N]) -> Self {
        assert!(
            sub_range.start < N && sub_range.end <= N,
            "the sub-range must be in bounds"
        );

        if sub_range.start > sub_range.end {
            Self {
                start: 0,
                end: 0,
                array,
            }
        } else {
            Self {
                start: sub_range.start,
                end: sub_range.end,
                array,
            }
        }
    }

    /// Returns a reference to the full underlying array. There is no guarantee about the data
    /// outside the visible region.
    pub const fn as_full_array(&self) -> &[T; N] {
        &self.array
    }

    /// Converts `self` into the full underlying array. There is no guarantee about the data
    /// outside the visible region.
    pub fn into_full_array(self) -> [T; N] {
        self.array
    }

    /// Returns the visible part of the array as a slice.
    pub const fn as_slice(&self) -> &[T] {
        let (_, tail) = self.array.split_at(self.start);
        tail.split_at(self.end - self.start).0
    }

    /// Returns the index of the first element of the underlying array that's inside the visible region.
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the index of the first element of the underlying array that is
    /// invisible again after the end of the visible part.
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Returns the length of the visible part of the array.
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns whether the visible part is empty.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns whether there are no parts of the array that are invisible.
    /// If this is `true` it is completely fine to call [`as_full_array`](RestrictedArray::as_full_array)
    /// or [`into_full_array`](RestrictedArray::into_full_array).
    pub const fn is_fully_visible(&self) -> bool {
        self.len() == N
    }

    /// Returns an iterator over the visible section.
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.as_slice().iter()
    }
}

impl<const N: usize, T> core::ops::Index<usize> for RestrictedArray<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let i = match self.start.checked_add(index) {
            Some(sum) => sum,
            None => panic!("index overflowed"),
        };

        if i >= self.end {
            panic!("index was {i} when len was {}", self.end - self.start);
        }

        &self.array[i]
    }
}

/// Created by the [`into_iter`](RestrictedArray::into_iter) function on [`RestrictedArray`], see it for more information.
pub struct RestrictedArrayIntoIter<const N: usize, T>(
    core::iter::Take<core::iter::Skip<core::array::IntoIter<T, N>>>,
);

impl<const N: usize, T> Iterator for RestrictedArrayIntoIter<N, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<const N: usize, T> FusedIterator for RestrictedArrayIntoIter<N, T> {}
impl<const N: usize, T> ExactSizeIterator for RestrictedArrayIntoIter<N, T> {}
impl<const N: usize, T> DoubleEndedIterator for RestrictedArrayIntoIter<N, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<const N: usize, T> IntoIterator for RestrictedArray<T, N> {
    type IntoIter = RestrictedArrayIntoIter<N, T>;
    type Item = <RestrictedArrayIntoIter<N, T> as Iterator>::Item;
    fn into_iter(self) -> Self::IntoIter {
        let start = self.start;
        let len = self.len();
        RestrictedArrayIntoIter(self.array.into_iter().skip(start).take(len))
    }
}

// region: PartialEq impls
impl<const N: usize, T, U> PartialEq<[U]> for RestrictedArray<T, N>
where
    U: PartialEq<T>,
{
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.  
    /// Only compares the *visible* part of `self` against `other`.
    fn eq(&self, other: &[U]) -> bool {
        other == self.as_slice()
    }
}

impl<const N: usize, T, U: PartialEq<T>> PartialEq<RestrictedArray<T, N>> for [U] {
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.  
    /// Only compares the *visible* part of `other` against `self`.
    fn eq(&self, other: &RestrictedArray<T, N>) -> bool {
        self == other.as_slice()
    }
}
// endregion: PartialEq impls

impl<const N: usize, T> AsRef<[T]> for RestrictedArray<T, N> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
