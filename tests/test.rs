#![no_std]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

#[cfg(feature = "rayon")]
extern crate rayon_crate as rayon;

use core::iter::FusedIterator;

use iter_enum::{DoubleEndedIterator, ExactSizeIterator, Extend, FusedIterator, Iterator};
#[cfg(feature = "rayon")]
use iter_enum::{IndexedParallelIterator, ParallelExtend, ParallelIterator};
#[cfg(feature = "rayon")]
use rayon::iter::{IndexedParallelIterator, ParallelExtend, ParallelIterator};

#[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)]
#[cfg_attr(feature = "rayon", derive(ParallelIterator, IndexedParallelIterator, ParallelExtend))]
enum Either<A, B> {
    A(A),
    B(B),
}

fn _assert_impl<
    T: Iterator + DoubleEndedIterator + ExactSizeIterator + FusedIterator + Extend<()>,
>() {
    fn __assert_impl<
        T: Iterator + DoubleEndedIterator + ExactSizeIterator + FusedIterator + Extend<()>,
    >() {
    }
    __assert_impl::<Either<T, T>>();
}
#[cfg(feature = "rayon")]
fn _assert_impl_rayon<T: ParallelIterator + IndexedParallelIterator + ParallelExtend<()>>() {
    fn __assert_impl_rayon<T: ParallelIterator + IndexedParallelIterator + ParallelExtend<()>>() {}
    __assert_impl_rayon::<Either<T, T>>();
}
