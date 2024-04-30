use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    iter::FusedIterator,
    ops::{Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

/// An array where only a section of the data may be viewed,
/// as the other data may e.g. not uphold some invariant.
/// The other data is not considered in comparisons, ordering or hashing.
#[derive(Debug, Clone, Copy, Eq)]
pub struct ArraySection<T, const N: usize> {
    start: usize,
    end: usize,
    array: [T; N],
}

impl<const N: usize, T: Hash> Hash for ArraySection<T, N> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

/// Only compares the data in the sections, and not the full arrays.
impl<const N: usize, T: PartialEq> PartialEq<ArraySection<T, N>> for ArraySection<T, N> {
    #[inline]
    fn eq(&self, other: &ArraySection<T, N>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<const N: usize, T: PartialOrd> PartialOrd for ArraySection<T, N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<const N: usize, T: Ord> Ord for ArraySection<T, N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<const N: usize, T> ArraySection<T, N> {
    /// Restrict an array so that only elements within the given range are visible.
    ///
    /// # Panics
    ///
    /// Panics if the range of indices is out of bounds of the array.
    #[inline]
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

    /// Returns the first index of the full underlying array that is part of the section.
    /// I.e. the section is the subrange `start ..`[`end`](ArraySection::end).
    #[inline]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the first index of the full underlying array that is outside the section (to the right).
    /// I.e. the section is the subrange [`start`](ArraySection::start)`.. end`.
    #[inline]
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Returns a reference to the full underlying array. There is no guarantee about the data
    /// outside the section.
    #[inline]
    pub const fn as_full_array(&self) -> &[T; N] {
        &self.array
    }

    /// Converts `self` into the full underlying array. There is no guarantee about the data
    /// outside the section.
    #[inline]
    pub fn into_full_array(self) -> [T; N] {
        self.array
    }

    /// Returns the section of the array as a slice.
    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        self.array
            // Split &[head, section, tail] into (&[head], &[section, tail])
            .split_at(self.start)
            // and extract the second element of the tuple.
            .1
            // Split &[section, tail] into (&[section], &[tail])
            .split_at(self.end - self.start)
            // and extract the first element of the tuple.
            .0
    }

    /// Returns the length of the array section.
    #[inline]
    pub const fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// Returns whether the array section is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    /// Returns whether the section is just the entire array.
    /// If this is `true` it is completely fine to call [`as_full_array`](RestrictedArray::as_full_array)
    /// or [`into_full_array`](RestrictedArray::into_full_array).
    #[inline]
    pub const fn section_is_full_array(&self) -> bool {
        self.len() == N
    }

    /// Returns an iterator over the array section.
    #[inline]
    pub fn iter(&self) -> ArraySectionIter<'_, T> {
        ArraySectionIter::new(self.as_slice().iter())
    }
}

impl<const N: usize, T> AsRef<[T]> for ArraySection<T, N> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

// region: Index impls

impl<const N: usize, T> Index<usize> for ArraySection<T, N> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.as_slice().index(index)
    }
}

macro_rules! impl_index_range {
    ($($t:ty),+) => {
        $(
            impl<const N: ::core::primitive::usize, T> ::core::ops::Index<$t> for $crate::array_section::ArraySection<T, N> {
                type Output = [T];
                #[inline]
                fn index(&self, index: $t) -> &Self::Output {
                    ::core::ops::Index::index(self.as_slice(), index)
                }
            }
        )+
    };
}

impl_index_range! {Range<usize>, RangeFrom<usize>, RangeFull, RangeTo<usize>, RangeInclusive<usize>, RangeToInclusive<usize>}

// endregion: Index impls

// region: PartialEq impls
impl<const N: usize, T, U> PartialEq<[U]> for ArraySection<T, N>
where
    U: PartialEq<T>,
{
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.  
    /// Only compares the *visible* part of `self` against `other`.
    fn eq(&self, other: &[U]) -> bool {
        other == self.as_slice()
    }
}

impl<const N: usize, T, U> PartialEq<ArraySection<T, N>> for [U]
where
    U: PartialEq<T>,
{
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.  
    /// Only compares the *visible* part of `other` against `self`.
    fn eq(&self, other: &ArraySection<T, N>) -> bool {
        self == other.as_slice()
    }
}
// endregion: PartialEq impls

impl<const N: usize, T> AsRef<[T]> for ArraySection<T, N> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<const N: usize, T> IntoIterator for ArraySection<T, N> {
    type IntoIter = ArraySectionIntoIter<T, N>;
    type Item = <ArraySectionIntoIter<T, N> as Iterator>::Item;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let start = self.start;
        let len = self.len();
        ArraySectionIntoIter::new(self.array.into_iter().skip(start).take(len))
    }
}

impl<'a, const N: usize, T> IntoIterator for &'a ArraySection<T, N> {
    type IntoIter = ArraySectionIter<'a, T>;
    type Item = <ArraySectionIter<'a, T> as Iterator>::Item;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ArraySectionIter::new(self.as_slice().iter())
    }
}

use array_section_iter::ArraySectionIter;
mod array_section_iter {
    use super::FusedIterator;

    #[derive(Debug, Clone)]
    pub struct ArraySectionIter<'a, T>(core::slice::Iter<'a, T>);

    impl<'a, T> ArraySectionIter<'a, T> {
        pub const fn new(iter: core::slice::Iter<'a, T>) -> Self {
            Self(iter)
        }
    }

    impl<'a, T> Iterator for ArraySectionIter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }

        fn last(self) -> Option<Self::Item> {
            self.0.last()
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.0.nth(n)
        }

        fn count(self) -> usize {
            self.0.count()
        }
    }
    impl<'a, T> DoubleEndedIterator for ArraySectionIter<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.0.next_back()
        }
    }
    impl<'a, T> ExactSizeIterator for ArraySectionIter<'a, T> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }
    impl<'a, T> FusedIterator for ArraySectionIter<'a, T> {}
}

use array_section_into_iter::ArraySectionIntoIter;
mod array_section_into_iter {
    use super::FusedIterator;

    #[derive(Debug, Clone)]
    /// Created by the [`into_iter`](RestrictedArray::into_iter) function on [`RestrictedArray`], see it for more information.
    pub struct ArraySectionIntoIter<T, const N: usize>(
        core::iter::Take<core::iter::Skip<core::array::IntoIter<T, N>>>,
    );

    impl<const N: usize, T> ArraySectionIntoIter<T, N> {
        pub const fn new(
            iter: core::iter::Take<core::iter::Skip<core::array::IntoIter<T, N>>>,
        ) -> Self {
            Self(iter)
        }
    }

    impl<const N: usize, T> Iterator for ArraySectionIntoIter<T, N> {
        type Item = T;
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            let l = self.0.len();
            (l, Some(l))
        }

        #[inline]
        fn nth(&mut self, index: usize) -> Option<Self::Item> {
            self.0.nth(index)
        }

        #[inline]
        fn last(self) -> Option<T> {
            self.0.last()
        }

        #[inline]
        fn count(self) -> usize {
            self.0.count()
        }
    }
    impl<const N: usize, T> FusedIterator for ArraySectionIntoIter<T, N> {}
    impl<const N: usize, T> ExactSizeIterator for ArraySectionIntoIter<T, N> {}
    impl<const N: usize, T> DoubleEndedIterator for ArraySectionIntoIter<T, N> {
        #[inline]
        fn next_back(&mut self) -> Option<Self::Item> {
            self.0.next_back()
        }
    }
}
