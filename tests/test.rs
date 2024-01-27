// SPDX-License-Identifier: Apache-2.0 OR MIT

#![no_std]
#![allow(dead_code)]

extern crate alloc;
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

// https://github.com/taiki-e/derive_utils/issues/47
#[derive(Iterator)]
enum A {
    A(alloc::boxed::Box<B>),
}
#[derive(Iterator)]
enum B {
    C(alloc::vec::IntoIter<i32>),
    B(A),
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
