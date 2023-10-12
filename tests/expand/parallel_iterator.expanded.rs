extern crate rayon_crate as rayon;
use iter_enum::*;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B> ::rayon::iter::ParallelIterator for Enum<A, B>
where
    A: ::rayon::iter::ParallelIterator,
    B: ::rayon::iter::ParallelIterator<
        Item = <A as ::rayon::iter::ParallelIterator>::Item,
    >,
{
    type Item = <A as ::rayon::iter::ParallelIterator>::Item;
    #[inline]
    fn drive_unindexed<__C>(self, consumer: __C) -> __C::Result
    where
        __C: ::rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        match self {
            Enum::A(x) => ::rayon::iter::ParallelIterator::drive_unindexed(x, consumer),
            Enum::B(x) => ::rayon::iter::ParallelIterator::drive_unindexed(x, consumer),
        }
    }
    #[inline]
    fn opt_len(&self) -> ::core::option::Option<usize> {
        match self {
            Enum::A(x) => ::rayon::iter::ParallelIterator::opt_len(x),
            Enum::B(x) => ::rayon::iter::ParallelIterator::opt_len(x),
        }
    }
}
fn main() {}
