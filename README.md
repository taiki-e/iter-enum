# iter-enum

[![crates.io](https://img.shields.io/crates/v/iter-enum?style=flat-square&logo=rust)](https://crates.io/crates/iter-enum)
[![docs.rs](https://img.shields.io/badge/docs.rs-iter--enum-blue?style=flat-square&logo=docs.rs)](https://docs.rs/iter-enum)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.31+-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/workflow/status/taiki-e/iter-enum/CI/main?style=flat-square&logo=github)](https://github.com/taiki-e/iter-enum/actions)

\#\[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, Extend)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
iter-enum = "1"
```

*Compiler support: requires rustc 1.31+*

## Examples

```rust
use iter_enum::*;

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

See [auto_enums] crate for how to automate patterns like this.

## Supported traits

- [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) - [example](tests/expand/iterator.rs) | [generated code](tests/expand/iterator.expanded.rs)
- [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html) - [example](tests/expand/double_ended_iterator.rs) | [generated code](tests/expand/double_ended_iterator.expanded.rs)
- [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html) - [example](tests/expand/exact_size_iterator.rs) | [generated code](tests/expand/exact_size_iterator.expanded.rs)
- [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) - [example](tests/expand/fused_iterator.rs) | [generated code](tests/expand/fused_iterator.expanded.rs)
- [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html) - [example](tests/expand/extend.rs) | [generated code](tests/expand/extend.expanded.rs)
- [`ParallelIterator`](https://docs.rs/rayon/1/rayon/iter/trait.ParallelIterator.html) (*requires `"rayon"` feature*) - [example](tests/expand/parallel_iterator.rs) | [generated code](tests/expand/parallel_iterator.expanded.rs)
- [`IndexedParallelIterator`](https://docs.rs/rayon/1/rayon/iter/trait.IndexedParallelIterator.html) (*requires `"rayon"` feature*) - [example](tests/expand/indexed_parallel_iterator.rs) | [generated code](tests/expand/indexed_parallel_iterator.expanded.rs)
- [`ParallelExtend`](https://docs.rs/rayon/1/rayon/iter/trait.ParallelExtend.html) (*requires `"rayon"` feature*) - [example](tests/expand/parallel_extend.rs) | [generated code](tests/expand/parallel_extend.expanded.rs)

## Optional features

- **`rayon`**
  - Enable to use `#[derive(ParallelIterator, IndexedParallelIterator, ParallelExtend)]`.

## Related Projects

- [auto_enums]: A library for to allow multiple return types by automatically generated enum.
- [derive_utils]: A procedural macro helper for easily writing [derives macros][proc-macro-derive] for enums.
- [io-enum]: \#\[derive(Read, Write, Seek, BufRead)\] for enums.

[auto_enums]: https://github.com/taiki-e/auto_enums
[derive_utils]: https://github.com/taiki-e/derive_utils
[io-enum]: https://github.com/taiki-e/io-enum
[proc-macro-derive]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
