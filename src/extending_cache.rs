extern crate alloc;

use crate::{Primes, Underlying};

use core::{ops::Index, slice::SliceIndex};

use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExtendingPrimes<const N: usize> {
    Stack(Primes<N>),
    Heap(Vec<Underlying>),
}

impl<const N: usize> ExtendingPrimes<N> {
    pub const fn new() -> Self {
        if N > 0 {
            Self::Stack(Primes::new())
        } else {
            Self::Heap(Vec::new())
        }
    }

    pub fn as_slice(&self) -> &[Underlying] {
        match self {
            Self::Stack(a) => a.as_slice(),
            Self::Heap(v) => v.as_slice(),
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
