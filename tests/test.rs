#![no_std]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

#[cfg(feature = "rayon")]
extern crate rayon_crate as rayon;

use iter_enum::{DoubleEndedIterator, ExactSizeIterator, Extend, FusedIterator, Iterator};
#[cfg(feature = "rayon")]
use iter_enum::{IndexedParallelIterator, ParallelExtend, ParallelIterator};

#[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)]
#[cfg_attr(feature = "rayon", derive(ParallelIterator, IndexedParallelIterator, ParallelExtend))]
enum Either<A, B> {
    A(A),
    B(B),
}
