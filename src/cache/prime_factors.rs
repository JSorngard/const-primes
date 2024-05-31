use super::Underlying;

use core::iter::FusedIterator;

/// An iterator over the prime factors of a number and their multiplicities.
///
/// Created by the [`prime_factorization`](super::Primes::prime_factorization) function on [`Primes`](super::Primes),
/// see it for more information.
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct PrimeFactorization<'a> {
    primes_cache: &'a [Underlying],
    cache_index: usize,
    number: Underlying,
}

impl<'a> PrimeFactorization<'a> {
    pub(crate) const fn new(primes_cache: &'a [Underlying], number: Underlying) -> Self {
        Self {
            primes_cache,
            cache_index: 0,
            number,
        }
    }

    /// If the number contains prime factors that are larger than the largest prime
    /// in the cache, this function returns their product.
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn remainder(mut self) -> Option<Underlying> {
        for _ in self.by_ref() {}
        if self.number > 1 {
            Some(self.number)
        } else {
            None
        }
    }
}

impl<'a> Iterator for PrimeFactorization<'a> {
    type Item = (Underlying, u8);
    fn next(&mut self) -> Option<Self::Item> {
        if self.number == 1 {
            return None;
        }

        while let Some(prime) = self.primes_cache.get(self.cache_index) {
            let mut count = 0;
            while self.number % prime == 0 {
                count += 1;
                self.number /= prime;
            }

            self.cache_index += 1;

            if count > 0 {
                return Some((*prime, count));
            }
        }

        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.primes_cache.len() - self.cache_index))
    }
}

impl<'a> FusedIterator for PrimeFactorization<'a> {}

/// An iterator over the prime factors of a given number.
///
/// Created by the [`prime_factors`](super::Primes::prime_factors)
/// function on [`Primes`](super::Primes), see it for more information.
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct PrimeFactors<'a> {
    primes_cache: &'a [Underlying],
    cache_index: usize,
    number: Underlying,
}

impl<'a> PrimeFactors<'a> {
    #[inline]
    pub(crate) const fn new(primes_cache: &'a [Underlying], number: Underlying) -> Self {
        Self {
            primes_cache,
            cache_index: 0,
            number,
        }
    }

    /// If the number contains prime factors that are larger than the largest prime
    /// in the cache, this function returns their product.
    ///
    /// It does this by doing all the work that [`PrimeFactorization`] would have done,
    /// so the performance advantage of this iterator over that one dissapears if this function is called.
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn remainder(self) -> Option<Underlying> {
        // We haven't actually divided out any of the factors to save work,
        // so we do that by just delegating to PrimeFactorization,
        // which does the work in its implementation of this function.
        PrimeFactorization::new(self.primes_cache, self.number).remainder()
    }
}

impl<'a> Iterator for PrimeFactors<'a> {
    type Item = Underlying;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(prime) = self.primes_cache.get(self.cache_index) {
            self.cache_index += 1;
            if self.number % prime == 0 {
                return Some(*prime);
            }
        }

        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.primes_cache.len() - self.cache_index))
    }
}

impl<'a> FusedIterator for PrimeFactors<'a> {}
