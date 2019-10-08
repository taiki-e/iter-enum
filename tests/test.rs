#![no_std]
#![cfg_attr(feature = "trusted_len", feature(trusted_len))]
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
