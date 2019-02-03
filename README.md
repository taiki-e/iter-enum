# iterator-enum

[![Build Status](https://travis-ci.com/taiki-e/iterator-enum.svg?branch=master)](https://travis-ci.com/taiki-e/iterator-enum)
[![version](https://img.shields.io/crates/v/iterator-enum.svg)](https://crates.io/crates/iterator-enum/)
[![documentation](https://docs.rs/iterator-enum/badge.svg)](https://docs.rs/iterator-enum/)
[![license](https://img.shields.io/crates/l/iterator-enum.svg)](https://crates.io/crates/iterator-enum/)
[![Rustc Version](https://img.shields.io/badge/rustc-1.30+-lightgray.svg)](https://blog.rust-lang.org/2018/10/25/Rust-1.30.0.html)

\#\[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
iterator-enum = "0.1"
```

Now, you can use iterator-enum:

```rust
use iterator_enum::*;
```

The current version of iterator-enum requires Rust 1.30 or later.

## Examples

```rust
use iterator_enum::*;

#[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)]
enum Either<A, B> {
    A(A),
    B(B),
}

fn foo(x: i32) -> impl Iterator<Item = i32> {
    if x > 0 {
        Either::A(x..=0)
    } else {
        Either::B(Some(x).into_iter())
    }
}
```

## Supported traits

* [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
* [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html)
* [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html)
* [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html)
* [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html) (*requires `"trusted_len"` crate feature*)
* [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html)

See [taiki-e/auto_enums#11](https://github.com/taiki-e/auto_enums/issues/11) for other traits.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
