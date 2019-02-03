#![cfg_attr(feature = "try_trait", feature(try_trait))]
#![cfg_attr(feature = "trusted_len", feature(trusted_len))]
#![cfg_attr(feature = "exact_size_is_empty", feature(exact_size_is_empty))]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(warnings)]
#![allow(dead_code)]

extern crate iterator_enum;

use iterator_enum::*;

#[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)]
#[cfg_attr(feature = "trusted_len", derive(TrustedLen))]
enum Either<A, B> {
    A(A),
    B(B),
}
