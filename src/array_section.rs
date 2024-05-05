use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    iter::FusedIterator,
    ops::{Index, Range},
    slice::SliceIndex,
};

/// An array where only a section of the data may be viewed,
/// as the other data may e.g. not uphold some invariant.
///
/// Indexing into the `ArraySection` indices only into the section:
/// ```
/// # use const_primes::array_section::ArraySection;
/// //                                                     v  v
/// const AS: ArraySection<i32, 4> = ArraySection::new([0, 1, 2, 0], 1..3);
/// assert_eq![AS[0], 1];
/// assert_eq![AS[1], 2];
/// ```
///
/// The other data is not considered in comparisons, ordering or hashing:
/// ```
/// # use const_primes::array_section::ArraySection;
/// //                       v  v
/// const A1: [i32; 4] = [1, 3, 7, 1];
/// const A2: [i32; 5] = [0, 3, 7, 100, -5];
/// const AS1: ArraySection<i32, 4> = ArraySection::new(A1, 1..3);
/// const AS2: ArraySection<i32, 5> = ArraySection::new(A2, 1..3);
///
/// // Even though the arrays are different
/// assert_ne!(A1.as_slice(), A2.as_slice());
/// // The sections are the same
/// assert_eq!(AS1, AS2);
/// ```
#[derive(Debug, Clone, Copy, Eq)]
pub struct ArraySection<T, const N: usize> {
    start: usize,
    end: usize,
    array: [T; N],
}

/// Only hashes the data in the section, and not the full array.
impl<const N: usize, T: Hash> Hash for ArraySection<T, N> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

/// Only checks the data in the sections, and not the full arrays.
impl<const N: usize, const M: usize, T: PartialOrd> PartialOrd<ArraySection<T, M>>
    for ArraySection<T, N>
{
    #[inline]
    fn partial_cmp(&self, other: &ArraySection<T, M>) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

/// Only compares the data in the sections and not the full arrays.
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
    pub const fn new(array: [T; N], sub_range: Range<usize>) -> Self {
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

    /// Returns a reference to the full underlying array if it is fully populated.
    #[inline]
    pub const fn try_as_full_array(&self) -> Option<&[T; N]> {
        if self.section_is_full_array() {
            Some(&self.array)
        } else {
            None
        }
    }

    /// Returns a reference to the full underlying array.
    #[inline]
    pub const fn as_full_array(&self) -> &[T; N] {
        &self.array
    }

    /// Converts `self` into the full underlying array.
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
    /// If this is `true` it is completely fine to call [`as_full_array`](ArraySection::as_full_array)
    /// or [`into_full_array`](ArraySection::into_full_array).
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

// region: TryFrom impls

/// Returned by `TryFrom<ArraySection<T, N>> for [T; N]` if the [`ArraySection`] was not fully valid.
/// Contains the original [`ArraySection`], which can be retrieved via the [`array_section`](TryFromArraySectionError::array_section) function.
#[derive(Debug, Clone, Copy)]
pub struct TryFromArraySectionError<T, const N: usize>(ArraySection<T, N>);

impl<T, const N: usize> TryFromArraySectionError<T, N> {
    /// Returns the original [`ArraySection`].
    pub fn array_section(self) -> ArraySection<T, N> {
        self.0
    }
}

impl<T, const N: usize> core::fmt::Display for TryFromArraySectionError<T, N> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "the array was not fully populated")
    }
}

#[cfg(feature = "std")]
impl<T: core::fmt::Debug, const N: usize> std::error::Error for TryFromArraySectionError<T, N> {}

impl<T, const N: usize> From<TryFromArraySectionError<T, N>> for ArraySection<T, N> {
    fn from(value: TryFromArraySectionError<T, N>) -> Self {
        value.0
    }
}

/// Converts the `ArraySection` into an array if the section is actually the entire array.
impl<const N: usize, T> TryFrom<ArraySection<T, N>> for [T; N] {
    type Error = TryFromArraySectionError<T, N>;

    #[inline]
    fn try_from(value: ArraySection<T, N>) -> Result<Self, Self::Error> {
        if value.section_is_full_array() {
            Ok(value.array)
        } else {
            Err(TryFromArraySectionError(value))
        }
    }
}

// endregion: TryFrom impls

impl<const N: usize, T> From<[T; N]> for ArraySection<T, N> {
    #[inline]
    fn from(value: [T; N]) -> Self {
        Self {
            start: 0,
            end: N,
            array: value,
        }
    }
}

impl<const N: usize, T> AsRef<[T]> for ArraySection<T, N> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<const N: usize, T, I: SliceIndex<[T]>> Index<I> for ArraySection<T, N> {
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.as_slice().index(index)
    }
}

// region: PartialEq impls

impl<const N: usize, const M: usize, T, U> PartialEq<ArraySection<T, N>> for ArraySection<U, M>
where
    [U]: PartialEq<[T]>,
{
    #[inline]
    fn eq(&self, other: &ArraySection<T, N>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<const N: usize, T, U> PartialEq<[U]> for ArraySection<T, N>
where
    U: PartialEq<T>,
{
    #[inline]
    fn eq(&self, other: &[U]) -> bool {
        other == self.as_slice()
    }
}

impl<const N: usize, T, U> PartialEq<ArraySection<T, N>> for [U]
where
    U: PartialEq<T>,
{
    #[inline]
    fn eq(&self, other: &ArraySection<T, N>) -> bool {
        self == other.as_slice()
    }
}

impl<const N: usize, const M: usize, T, U> PartialEq<[T; N]> for ArraySection<U, M>
where
    [U]: PartialEq<[T]>,
{
    #[inline]
    fn eq(&self, other: &[T; N]) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<const N: usize, const M: usize, T, U> PartialEq<ArraySection<U, M>> for [T; N]
where
    [T]: PartialEq<[U]>,
{
    #[inline]
    fn eq(&self, other: &ArraySection<U, M>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

// endregion: PartialEq impls

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

pub use array_section_iter::ArraySectionIter;
mod array_section_iter {
    use super::FusedIterator;

    #[derive(Debug, Clone)]
    pub struct ArraySectionIter<'a, T>(core::slice::Iter<'a, T>);

    impl<'a, T> ArraySectionIter<'a, T> {
        pub(crate) const fn new(iter: core::slice::Iter<'a, T>) -> Self {
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

pub use array_section_into_iter::ArraySectionIntoIter;
mod array_section_into_iter {
    use super::FusedIterator;

    #[derive(Debug, Clone)]
    /// Created by the [`into_iter`](super::ArraySection::into_iter) function on [`ArraySection`](super::ArraySection), see it for more information.
    pub struct ArraySectionIntoIter<T, const N: usize>(
        core::iter::Take<core::iter::Skip<core::array::IntoIter<T, N>>>,
    );

    impl<const N: usize, T> ArraySectionIntoIter<T, N> {
        pub(crate) const fn new(
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
