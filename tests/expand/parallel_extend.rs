// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate rayon_crate as rayon;

use iter_enum::*;

#[derive(ParallelExtend)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
