# const-primes

This is a small crate for creating arrays of prime numbers at compile time.

## Example

```rust
const PRIMES: [u32; 5] = Primes::new().into_array();
assert_eq!(PRIMES, [2, 3, 5, 7, 11]);
```