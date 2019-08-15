#![cfg_attr(feature = "try_trait", feature(try_trait))]
#![cfg_attr(feature = "trusted_len", feature(trusted_len))]
#![cfg_attr(feature = "exact_size_is_empty", feature(exact_size_is_empty))]
#![no_std]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

#[cfg(feature = "rayon")]
extern crate rayon_crate as rayon;

use iterator_enum::*;

#[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)]
#[cfg_attr(feature = "trusted_len", derive(TrustedLen))]
#[cfg_attr(feature = "rayon", derive(ParallelIterator, IndexedParallelIterator, ParallelExtend))]
enum Either<A, B> {
    A(A),
    B(B),
}
