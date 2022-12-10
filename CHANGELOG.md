# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

- Update `derive_utils` to 0.12.

## [1.0.1] - 2021-04-06

- [Apply `doc(cfg(...))` on feature gated APIs.](https://github.com/taiki-e/iter-enum/pull/15)

- Documentation improvements.

## [1.0.0] - 2021-01-27

- [Remove unstable `trusted_len` feature.](https://github.com/taiki-e/iter-enum/pull/14)

## [0.2.7] - 2021-01-05

- Exclude unneeded files from crates.io.

## [0.2.6] - 2020-12-29

- Documentation improvements.

## [0.2.5] - 2020-11-06

- Update `derive_utils` to 0.11.

## [0.2.4] - 2020-06-02

- Update `derive_utils` to 0.10.

## [0.2.3] - 2019-11-22

- Rename from `iterator-enum` to `iter-enum`.

## [0.2.2] - 2019-10-08

- [Remove unstable `exact_size_is_empty` and `try_trait` features.](https://github.com/taiki-e/iter-enum/pull/7)

## [0.2.1] - 2019-08-15

- Add support for `rayon::iter::{ParallelIterator, IndexedParallelIterator, ParallelExtend}`. These are disabled by default and can be used by enabling `rayon` feature.

- Add generated code examples.

- Update `syn` and `quote` to 1.0.

- Update `derive_utils` to 0.9.

## [0.2.0] - 2019-06-16

- Transition to Rust 2018. With this change, the minimum required version will go up to Rust 1.31.

- Remove `std` feature. This crate can generate accurate code without `std` feature.

- Update minimum `derive_utils` version to 0.7.2.

## [0.1.2] - 2019-02-05

- Update minimum `derive_utils` version to 0.6.3.

- Update minimum `syn` version to 0.15.22.

## [0.1.1] - 2019-02-03

- Documentation improvements.

## [0.1.0] - 2019-02-03

Initial release

[Unreleased]: https://github.com/taiki-e/iter-enum/compare/v1.0.1...HEAD
[1.0.1]: https://github.com/taiki-e/iter-enum/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/taiki-e/iter-enum/compare/v0.2.7...v1.0.0
[0.2.7]: https://github.com/taiki-e/iter-enum/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/taiki-e/iter-enum/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/taiki-e/iter-enum/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/taiki-e/iter-enum/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/taiki-e/iter-enum/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/taiki-e/iter-enum/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/iter-enum/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/iter-enum/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/taiki-e/iter-enum/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/iter-enum/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/iter-enum/releases/tag/v0.1.0
