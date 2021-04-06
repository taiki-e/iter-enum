use iter_enum::*;

#[derive(ParallelIterator)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
