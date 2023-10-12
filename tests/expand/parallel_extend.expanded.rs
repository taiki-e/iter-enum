extern crate rayon_crate as rayon;
use iter_enum::*;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B, __T: ::core::marker::Send> ::rayon::iter::ParallelExtend<__T> for Enum<A, B>
where
    A: ::rayon::iter::ParallelExtend<__T>,
    B: ::rayon::iter::ParallelExtend<__T>,
{
    #[inline]
    fn par_extend<__I>(&mut self, par_iter: __I)
    where
        __I: ::rayon::iter::IntoParallelIterator<Item = __T>,
    {
        match self {
            Enum::A(x) => ::rayon::iter::ParallelExtend::par_extend(x, par_iter),
            Enum::B(x) => ::rayon::iter::ParallelExtend::par_extend(x, par_iter),
        }
    }
}
fn main() {}
