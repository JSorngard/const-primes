extern crate alloc;

use crate::{Primes, Underlying};

use core::{ops::Index, slice::SliceIndex};

use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtendingPrimes<const N: usize>(Inner<N>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Inner<const N: usize> {
    /// The primes are stored in an array on the stack.
    Stack(Primes<N>),
    /// The primes are stored in a `Vec` on the heap.
    Heap(Vec<Underlying>),
}

impl<const N: usize> ExtendingPrimes<N> {
    pub const fn new() -> Self {
        if N > 0 {
            Self(Inner::Stack(Primes::new()))
        } else {
            Self(Inner::Heap(Vec::new()))
        }
    }

    pub fn as_slice(&self) -> &[Underlying] {
        match self.0 {
            Inner::Stack(ref a) => a.as_slice(),
            Inner::Heap(ref v) => v.as_slice(),
        }
    }

    pub fn binary_search_cache(&self, prime: Underlying) -> Result<usize, usize> {
        self.as_slice().binary_search(&prime)
    }

    pub fn is_prime_cached(&self, number: Underlying) -> Option<bool> {
        match self.binary_search_cache(number) {
            Ok(_) => Some(true),
            Err(i) => {
                if i < N {
                    Some(false)
                } else {
                    None
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }
}

impl<const N: usize, I: SliceIndex<[Underlying]>> Index<I> for ExtendingPrimes<N> {
    type Output = <I as SliceIndex<[Underlying]>>::Output;
    fn index(&self, index: I) -> &Self::Output {
        self.as_slice().index(index)
    }
}

impl<const N: usize> Default for ExtendingPrimes<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> AsRef<[Underlying]> for ExtendingPrimes<N> {
    fn as_ref(&self) -> &[Underlying] {
        self.as_slice()
    }
}
