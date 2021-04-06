use iter_enum::*;

#[derive(DoubleEndedIterator)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
