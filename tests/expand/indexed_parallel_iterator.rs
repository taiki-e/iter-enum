use iter_enum::*;

#[derive(IndexedParallelIterator)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
