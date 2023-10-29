use core::{iter::FusedIterator, ops::Range};

/// An array where only a section of the data may be viewed,
/// as the other data may be e.g. not uphold some invariant.
#[derive(Debug)]
pub struct RestrictedArray<const N: usize, T> {
    start: usize,
    end: usize,
    array: [T; N],
}

impl<const N: usize, T: Clone> Clone for RestrictedArray<N, T> {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            end: self.end,
            array: self.array.clone(),
        }
    }
}

impl<const N: usize, T: Copy> Copy for RestrictedArray<N, T> {}

impl<const N: usize, T: PartialEq<T>> PartialEq<RestrictedArray<N, T>> for RestrictedArray<N, T> {
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.  
    /// Only compares the *unrestricted* part of `self` against the *unrestricted* part of `other`.
    fn eq(&self, other: &RestrictedArray<N, T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<const N: usize, T> RestrictedArray<N, T> {
    /// Restrict an array so that only elements within the given range are viewable.
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

    /// Returns a reference to the full underlying array.
    pub const fn as_full_array(&self) -> &[T; N] {
        &self.array
    }

    /// Converts `self` into the full underlying array.
    pub fn into_full_array(self) -> [T; N] {
        self.array
    }

    /// Returns the unrestricted part of the array as a slice.
    pub const fn as_slice(&self) -> &[T] {
        let (_, tail) = self.array.split_at(self.start);
        tail.split_at(self.end - self.start).0
    }

    /// Returns the length of the unrestricted part of the array.
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns whether the unrestricted part is empty.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns whether there are parts of the array that are restricted.
    /// If this is `false` it is completely fine to call [`as_full_array`](RestrictedArray::as_full_array)
    /// or [`into_full_array`](RestrictedArray::into_full_array).
    pub const fn is_restricted(&self) -> bool {
        self.len() == N
    }

    /// Returns an iterator over the unrestricted section.
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.as_slice().iter()
    }
}

impl<const N: usize, T> core::ops::Index<usize> for RestrictedArray<N, T> {
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

impl<const N: usize, T> IntoIterator for RestrictedArray<N, T> {
    type IntoIter = RestrictedArrayIntoIter<N, T>;
    type Item = <RestrictedArrayIntoIter<N, T> as Iterator>::Item;
    fn into_iter(self) -> Self::IntoIter {
        let start = self.start;
        let len = self.len();
        RestrictedArrayIntoIter(self.array.into_iter().skip(start).take(len))
    }
}

impl<const N: usize, T, U> PartialEq<U> for RestrictedArray<N, T>
where
    U: PartialEq<[T]>,
{
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.  
    /// Only compares the *unrestricted* part of `self` against `other`.
    fn eq(&self, other: &U) -> bool {
        other == self.as_slice()
    }
}
