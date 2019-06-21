## [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html)

When deriving for enum like the following:

```rust
#[derive(Extend)]
enum Enum<A, B> {
    A(A),
    B(B),
}
```

Code like this will be generated:

```rust
enum Enum<A, B> {
    A(A),
    B(B),
}

impl<A, B> ::core::iter::Extend for Enum<A, B>
where
    A: ::core::iter::Extend,
    B: ::core::iter::Extend,
{
    #[inline]
    fn extend<__T: ::core::iter::IntoIterator<Item = __A>>(&mut self, iter: __T) {
        match self {
            Enum::A(x) => ::core::iter::Extend::extend(x, iter),
            Enum::B(x) => ::core::iter::Extend::extend(x, iter),
        }
    }
}
```
