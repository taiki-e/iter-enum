# iter-enum

[![crates.io](https://img.shields.io/crates/v/iter-enum?style=flat-square&logo=rust)](https://crates.io/crates/iter-enum)
[![docs.rs](https://img.shields.io/badge/docs.rs-iter--enum-blue?style=flat-square&logo=docs.rs)](https://docs.rs/iter-enum)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![msrv](https://img.shields.io/badge/msrv-1.61-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![github actions](https://img.shields.io/github/actions/workflow/status/taiki-e/iter-enum/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/iter-enum/actions)

<!-- tidy:sync-markdown-to-rustdoc:start:src/lib.rs -->

\#\[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
iter-enum = "1"
```

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

- [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) - [example](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/iterator.rs) | [generated code](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/iterator.expanded.rs)
- [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html) - [example](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/double_ended_iterator.rs) | [generated code](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/double_ended_iterator.expanded.rs)
- [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html) - [example](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/exact_size_iterator.rs) | [generated code](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/exact_size_iterator.expanded.rs)
- [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) - [example](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/fused_iterator.rs) | [generated code](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/fused_iterator.expanded.rs)
- [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html) - [example](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/extend.rs) | [generated code](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/extend.expanded.rs)
- [`ParallelIterator`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html) (*requires `"rayon"` feature*) - [example](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/parallel_iterator.rs) | [generated code](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/parallel_iterator.expanded.rs)
- [`IndexedParallelIterator`](https://docs.rs/rayon/latest/rayon/iter/trait.IndexedParallelIterator.html) (*requires `"rayon"` feature*) - [example](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/indexed_parallel_iterator.rs) | [generated code](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/indexed_parallel_iterator.expanded.rs)
- [`ParallelExtend`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelExtend.html) (*requires `"rayon"` feature*) - [example](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/parallel_extend.rs) | [generated code](https://github.com/taiki-e/iter-enum/blob/HEAD/tests/expand/parallel_extend.expanded.rs)

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

<!-- tidy:sync-markdown-to-rustdoc:end -->

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
