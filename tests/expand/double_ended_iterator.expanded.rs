use iter_enum::*;
enum Enum<A, B> {
    A(A),
    B(B),
}
#[automatically_derived]
impl<A, B> ::core::iter::DoubleEndedIterator for Enum<A, B>
where
    A: ::core::iter::DoubleEndedIterator,
    B: ::core::iter::DoubleEndedIterator<Item = <A as ::core::iter::Iterator>::Item>,
{
    #[inline]
    fn next_back(&mut self) -> ::core::option::Option<Self::Item> {
        match self {
            Enum::A(x) => <A as ::core::iter::DoubleEndedIterator>::next_back(x),
            Enum::B(x) => <B as ::core::iter::DoubleEndedIterator>::next_back(x),
        }
    }
    #[inline]
    fn nth_back(&mut self, n: usize) -> ::core::option::Option<Self::Item> {
        match self {
            Enum::A(x) => <A as ::core::iter::DoubleEndedIterator>::nth_back(x, n),
            Enum::B(x) => <B as ::core::iter::DoubleEndedIterator>::nth_back(x, n),
        }
    }
    #[inline]
    fn rfold<__U, __F>(self, init: __U, f: __F) -> __U
    where
        __F: ::core::ops::FnMut(__U, Self::Item) -> __U,
    {
        match self {
            Enum::A(x) => <A as ::core::iter::DoubleEndedIterator>::rfold(x, init, f),
            Enum::B(x) => <B as ::core::iter::DoubleEndedIterator>::rfold(x, init, f),
        }
    }
    #[inline]
    fn rfind<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
    where
        __P: ::core::ops::FnMut(&Self::Item) -> bool,
    {
        match self {
            Enum::A(x) => <A as ::core::iter::DoubleEndedIterator>::rfind(x, predicate),
            Enum::B(x) => <B as ::core::iter::DoubleEndedIterator>::rfind(x, predicate),
        }
    }
}
fn main() {}
