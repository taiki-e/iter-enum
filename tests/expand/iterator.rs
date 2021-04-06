use iter_enum::*;

#[derive(Iterator)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
