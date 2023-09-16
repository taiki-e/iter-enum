// SPDX-License-Identifier: Apache-2.0 OR MIT

use iter_enum::*;

#[derive(ParallelIterator)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
