use iter_enum::*;

#[derive(FusedIterator)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
