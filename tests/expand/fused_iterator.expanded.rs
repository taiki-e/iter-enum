use iter_enum::*;
enum Enum<A, B> {
    A(A),
    B(B),
}
#[automatically_derived]
impl<A, B> ::core::iter::FusedIterator for Enum<A, B>
where
    A: ::core::iter::FusedIterator,
    B: ::core::iter::FusedIterator<Item = <A as ::core::iter::Iterator>::Item>,
{}
fn main() {}
