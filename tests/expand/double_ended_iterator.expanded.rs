use iter_enum::*;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::core::iter::DoubleEndedIterator for Enum<A, B>
where
    A: ::core::iter::DoubleEndedIterator,
    B: ::core::iter::DoubleEndedIterator<Item = <A as ::core::iter::Iterator>::Item>,
{
    #[inline]
    fn next_back(&mut self) -> ::core::option::Option<Self::Item> {
        match self {
            Enum::A(x) => ::core::iter::DoubleEndedIterator::next_back(x),
            Enum::B(x) => ::core::iter::DoubleEndedIterator::next_back(x),
        }
    }
    #[inline]
    fn rfold<__U, __F>(self, accum: __U, f: __F) -> __U
    where
        __F: ::core::ops::FnMut(__U, Self::Item) -> __U,
    {
        match self {
            Enum::A(x) => ::core::iter::DoubleEndedIterator::rfold(x, accum, f),
            Enum::B(x) => ::core::iter::DoubleEndedIterator::rfold(x, accum, f),
        }
    }
    #[inline]
    fn rfind<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
    where
        __P: ::core::ops::FnMut(&Self::Item) -> bool,
    {
        match self {
            Enum::A(x) => ::core::iter::DoubleEndedIterator::rfind(x, predicate),
            Enum::B(x) => ::core::iter::DoubleEndedIterator::rfind(x, predicate),
        }
    }
}
fn main() {}
