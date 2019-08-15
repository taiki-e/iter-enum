# Unreleased

* Added support for `rayon::iter::{ParallelIterator, IndexedParallelIterator, ParallelExtend}`. These are disabled by default and can be used by enabling `"rayon"` crate feature.

* Added generated code examples.

* Updated `syn` and `quote` to 1.0.

* Updated `derive_utils` to 0.9.

# 0.2.0 - 2019-06-16

* Transition to Rust 2018. With this change, the minimum required version will go up to Rust 1.31.

* Removed `"std"` crate feature. This crate can generate accurate code without `"std"` feature.

* Updated minimum `derive_utils` version to 0.7.2.

# 0.1.2 - 2019-02-05

* Update minimum `derive_utils` version to 0.6.3.

* Update minimum `syn` version to 0.15.22.

# 0.1.1 - 2019-02-03

* Improve documentations.

# 0.1.0 - 2019-02-03

Initial release
