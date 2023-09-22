//! A crate for generating arrays of prime numbers at compile time.
//!
//! `#![no_std]` compatible.
//!
//! The crate is currently split into two modules, [`trial`] and [`sieve`], which contain implementations of
//! prime related `const` functions that use [trial division](https://en.wikipedia.org/wiki/Trial_division)
//! or the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) in their implementations respectively.
//!
//! The sieve needs `O(n)` memory, which means that the functions in the [`sieve`] module
//! need a const generic to be specified in order to compile.
//!
//! The implementations in [`trial`] are slower, but do not need const generics unless they return an array.
//!
//! # Examples
//! ## Prime generation
//! Generate arrays of prime numbers with the function [`trial::primes`](crate::trial::primes)
//! ```
//! use const_primes::trial::primes;
//! const PRIMES: [u32; 10] = primes();
//! assert_eq!(PRIMES[5], 13);
//! assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
//! ```
//! or with the type [`wrapper::Primes`](crate::wrapper::Primes)
//! which ensures that a non-zero number of primes are generated
//! ```
//! use const_primes::wrapper::Primes;
//! const PRIMES: Primes<10> = Primes::new();
//! assert_eq!(PRIMES[5], 13);
//! assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
//! ```
//! Creating a `Primes<0>` is a compile fail in const contexts and a panic otherwise.
//! ```compile_fail
//! # use const_primes::wrapper::Primes;
//! const PRIMES: Primes<0> = Primes::new();
//! ```
//! ## Primality testing
//! There is one implementation of `is_prime` in [`trial`] and one in [`sieve`]
//! ```
//! use const_primes::{trial, sieve};
//! const CHECK_5: bool = trial::is_prime(5);
//! const CHECK_1009: bool = sieve::is_prime::<1009>();
//! assert!(CHECK_5);
//! assert!(CHECK_1009);
//! ```
//! The [`Primes`](crate::wrapper::Primes) type lets you reuse an array of already computed primes for primality testing.
//! ```
//! # use const_primes::wrapper::Primes;
//! const CACHE: Primes<100> = Primes::new();
//! const CHECK_42: Option<bool> = CACHE.is_prime(42);
//! const CHECK_541: Option<bool> = CACHE.is_prime(541);
//! const CHECK_1000: Option<bool> = CACHE.is_prime(1000);
//! assert_eq!(CHECK_42, Some(false));
//! assert_eq!(CHECK_541, Some(true));
//! assert_eq!(CHECK_1000, None);
//! ```
//! The function [`sieve::primalities`] lets you compute the prime status of many integers
//! ```
//! # use const_primes::sieve::primalities;
//! const PRIME_STATUS: [bool; 10] = primalities();
//! assert_eq!(PRIME_STATUS, [false, false, true, true, false, true, false, true, false, false]);
//! ```

#![forbid(unsafe_code)]
#![no_std]

/// The functions that return or take integeres use this type. Currently `u32`.
// Just change this to whatever unsigned primitive integer type you want and it should work as long as it has enough bits for your purposes.
// This is used since there is currenlty no way to be generic over types that can do arithmetic at compile time.
type Underlying = u32;

pub mod sieve;
pub mod trial;
pub mod wrapper;

/// Returns the largest integer smaller than or equal to √n.
///
/// Uses a binary search.
#[must_use]
const fn isqrt(n: Underlying) -> Underlying {
    let mut left = 0;
    let mut right = n + 1;

    while left != right - 1 {
        let mid = left + (right - left) / 2;
        if mid * mid <= n {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_isqrt() {
        #[rustfmt::skip]
        const TEST_CASES: [(Underlying, Underlying); 100] = [(0, 0), (1, 1), (2, 1), (3, 1), (4, 2), (5, 2), (6, 2), (7, 2), (8, 2), (9, 3), (10, 3), (11, 3), (12, 3), (13, 3), (14, 3), (15, 3), (16, 4), (17, 4), (18, 4), (19, 4), (20, 4), (21, 4), (22, 4), (23, 4), (24, 4), (25, 5), (26, 5), (27, 5), (28, 5), (29, 5), (30, 5), (31, 5), (32, 5), (33, 5), (34, 5), (35, 5), (36, 6), (37, 6), (38, 6), (39, 6), (40, 6), (41, 6), (42, 6), (43, 6), (44, 6), (45, 6), (46, 6), (47, 6), (48, 6), (49, 7), (50, 7), (51, 7), (52, 7), (53, 7), (54, 7), (55, 7), (56, 7), (57, 7), (58, 7), (59, 7), (60, 7), (61, 7), (62, 7), (63, 7), (64, 8), (65, 8), (66, 8), (67, 8), (68, 8), (69, 8), (70, 8), (71, 8), (72, 8), (73, 8), (74, 8), (75, 8), (76, 8), (77, 8), (78, 8), (79, 8), (80, 8), (81, 9), (82, 9), (83, 9), (84, 9), (85, 9), (86, 9), (87, 9), (88, 9), (89, 9), (90, 9), (91, 9), (92, 9), (93, 9), (94, 9), (95, 9), (96, 9), (97, 9), (98, 9), (99, 9)];
        for (x, ans) in TEST_CASES {
            assert_eq!(isqrt(x), ans);
        }
    }
}
