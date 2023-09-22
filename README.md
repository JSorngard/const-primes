# const-primes

This is a small crate for creating arrays of prime numbers at compile time.  

This crate currently uses trial division to generate its arrays due to limitations imposed by Rust when evaluating expressions at compile time.
This renders it unsuitable for creating very large arrays.

`#![no_std]` compatible.

## Examples

Generate primes into a normal Rust array
```rust
const PRIMES: [u32; 5] = primes();
assert_eq!(PRIMES[3], 7);
assert_eq!(PRIMES, [2, 3, 5, 7, 11]);
```
or by using the type defined in the crate
```rust
const PRIMES: Primes<5> = Primes::new();
assert_eq!(PRIMES[3], 7);
assert_eq!(PRIMES, [2, 3, 5, 7, 11]);
```

## License

Licensed under either of

 * Apache License, Version 2.0
   [LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   [LICENSE-MIT](http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
