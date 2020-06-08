# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

## [Unreleased]

## [0.2.4] - 2020-06-02

* Updated `derive_utils` to 0.10.

## [0.2.3] - 2019-11-22

* Renamed from iterator-enum to iter-enum.

## [0.2.2] - 2019-10-08

* [Removed unstable `"exact_size_is_empty"` and `"try_trait"` features.][7]

[7]: https://github.com/taiki-e/iterator-enum/pull/7

## [0.2.1] - 2019-08-15

* Added support for `rayon::iter::{ParallelIterator, IndexedParallelIterator, ParallelExtend}`. These are disabled by default and can be used by enabling `"rayon"` feature.

* Added generated code examples.

* Updated `syn` and `quote` to 1.0.

* Updated `derive_utils` to 0.9.

## [0.2.0] - 2019-06-16

* Transition to Rust 2018. With this change, the minimum required version will go up to Rust 1.31.

* Removed `"std"` feature. This crate can generate accurate code without `"std"` feature.

* Updated minimum `derive_utils` version to 0.7.2.

## [0.1.2] - 2019-02-05

* Update minimum `derive_utils` version to 0.6.3.

* Update minimum `syn` version to 0.15.22.

## [0.1.1] - 2019-02-03

* Documentation improvements.

## [0.1.0] - 2019-02-03

Initial release

[Unreleased]: https://github.com/taiki-e/iterator-enum/compare/v0.2.4...HEAD
[0.2.4]: https://github.com/taiki-e/iterator-enum/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/taiki-e/iterator-enum/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/taiki-e/iterator-enum/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/iterator-enum/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/iterator-enum/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/taiki-e/iterator-enum/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/iterator-enum/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/iterator-enum/releases/tag/v0.1.0
