# const-primes

This is a small crate for creating arrays of prime numbers at compile time.  

Due to limitations imposed by compile time rust
this crate uses trial division to generate its arrays. This renders it unsuitable for creating very large arrays.

`#![no_std]` compatible.

## Example

```rust
const PRIMES: Primes<5> = Primes::new();
assert_eq!(PRIMES[3], 7);
assert_eq!(PRIMES, [2, 3, 5, 7, 11]);
```