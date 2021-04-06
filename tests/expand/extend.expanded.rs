use iter_enum::*;
enum Enum<A, B> {
    A(A),
    B(B),
}
impl<A, B, __A> ::core::iter::Extend<__A> for Enum<A, B>
where
    A: ::core::iter::Extend<__A>,
    B: ::core::iter::Extend<__A>,
{
    #[inline]
    fn extend<__T: ::core::iter::IntoIterator<Item = __A>>(&mut self, iter: __T) {
        match self {
            Enum::A(x) => ::core::iter::Extend::extend(x, iter),
            Enum::B(x) => ::core::iter::Extend::extend(x, iter),
        }
    }
}
fn main() {}
