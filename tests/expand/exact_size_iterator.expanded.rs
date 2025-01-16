use iter_enum::*;
enum Enum<A, B> {
    A(A),
    B(B),
}
#[automatically_derived]
impl<A, B> ::core::iter::ExactSizeIterator for Enum<A, B>
where
    A: ::core::iter::ExactSizeIterator,
    B: ::core::iter::ExactSizeIterator<Item = <A as ::core::iter::Iterator>::Item>,
{
    #[inline]
    fn len(&self) -> usize {
        match self {
            Enum::A(x) => <A as ::core::iter::ExactSizeIterator>::len(x),
            Enum::B(x) => <B as ::core::iter::ExactSizeIterator>::len(x),
        }
    }
}
fn main() {}
