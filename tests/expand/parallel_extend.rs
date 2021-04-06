use iter_enum::*;

#[derive(ParallelExtend)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
