# const-primes

A crate for generating arrays of prime numbers at compile time.

`#![no_std]` compatible.

The crate is currently split into two modules, `trial` and `sieve`, which contain implementations of
prime related `const` functions that use [trial division](https://en.wikipedia.org/wiki/Trial_division)
or the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) in their implementations respectively.

The sieve needs `O(n)` memory, which means that the functions in the `sieve` module
need a const generic to be specified in order to compile.

The implementations in `trial` are slower, but do not need const generics unless they return an array.

## Examples
Generate arrays of prime numbers with the function [`trial::primes`](crate::trial::primes)
```rust
const PRIMES: [u32; 10] = primes();
assert_eq!(PRIMES[5], 13);
assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
```
or with the type [`wrapper::Primes`](crate::wrapper::Primes)
which ensures that a non-zero number of primes are generated in const contexts
```rust
const PRIMES: Primes<10> = Primes::new();
assert_eq!(PRIMES[5], 13);
assert_eq!(PRIMES, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
```
as such the following fails to compile:
```rust
# use const_primes::wrapper::Primes;
const PRIMES: Primes<0> = Primes::new();
```

License: MIT OR Apache-2.0
