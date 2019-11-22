//! \#\[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)\] for enums.
//!
//! ## Examples
//!
//! ```rust
//! use iter_enum::*;
//!
//! #[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)]
//! enum Either<A, B> {
//!     A(A),
//!     B(B),
//! }
//!
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     if x > 0 { Either::A(0..=x) } else { Either::B(std::iter::empty()) }
//! }
//! ```
//!
//! See [auto_enums](https://github.com/taiki-e/auto_enums) crate for how to automate patterns like this.
//!
//! ## Supported traits
//!
//! * [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) - [generated code](https://github.com/taiki-e/iter-enum/blob/master/doc/iterator.md)
//! * [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html) - [generated code](https://github.com/taiki-e/iter-enum/blob/master/doc/double_ended_iterator.md)
//! * [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html) - [generated code](https://github.com/taiki-e/iter-enum/blob/master/doc/exact_size_iterator.md)
//! * [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) - [generated code](https://github.com/taiki-e/iter-enum/blob/master/doc/fused_iterator.md)
//! * [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html) - [generated code](https://github.com/taiki-e/iter-enum/blob/master/doc/extend.md)
//! * [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html) - [generated code](https://github.com/taiki-e/iter-enum/blob/master/doc/trusted_len.md) (*requires `"trusted_len"` feature*)
//! * [`ParallelIterator`](https://docs.rs/rayon/1/rayon/iter/trait.ParallelIterator.html) - [generated code](https://github.com/taiki-e/iter-enum/blob/master/doc/parallel_iterator.md) (*requires `"rayon"` feature*)
//! * [`IndexedParallelIterator`](https://docs.rs/rayon/1/rayon/iter/trait.IndexedParallelIterator.html) - [generated code](https://github.com/taiki-e/iter-enum/blob/master/doc/indexed_parallel_iterator.md) (*requires `"rayon"` feature*)
//! * [`ParallelExtend`](https://docs.rs/rayon/1/rayon/iter/trait.ParallelExtend.html) - [generated code](https://github.com/taiki-e/iter-enum/blob/master/doc/parallel_extend.md) (*requires `"rayon"` feature*)
//!
//! ## Optional features
//!
//! * `rayon`
//!   * Disabled by default.
//!   * Enable to use `#[derive(ParallelIterator, IndexedParallelIterator, ParallelExtend)]`.
//!
//! * `trusted_len`
//!   * Disabled by default.
//!   * Enable to use `#[derive(TrustedLen)]`.
//!   * This requires Rust Nightly and you need to enable the unstable [`trusted_len`](https://github.com/rust-lang/rust/issues/37572) feature gate.

#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/iter-enum/0.2.3")]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::all)]

extern crate proc_macro;

use derive_utils::{derive_trait, EnumData as Data};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_quote;

macro_rules! parse {
    ($input:expr) => {
        match syn::parse($input).and_then(|item: syn::DeriveInput| Data::new(&item)) {
            Ok(data) => data,
            Err(e) => return e.to_compile_error().into(),
        }
    };
}

#[proc_macro_derive(Iterator)]
pub fn derive_iterator(input: TokenStream) -> TokenStream {
    // TODO: When `try_trait` stabilized, add `try_fold` and remove `fold`, `find` etc. conditionally.

    // It is equally efficient if `try_fold` can be used.
    let try_trait = quote! {
        #[inline]
        fn fold<__U, __F>(self, init: __U, f: __F) -> __U
        where
            __F: ::core::ops::FnMut(__U, Self::Item) -> __U;
        #[inline]
        fn all<__F>(&mut self, f: __F) -> bool
        where
            __F: ::core::ops::FnMut(Self::Item) -> bool;
        #[inline]
        fn any<__F>(&mut self, f: __F) -> bool
        where
            __F: ::core::ops::FnMut(Self::Item) -> bool;
        #[inline]
        fn find<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
        where
            __P: ::core::ops::FnMut(&Self::Item) -> bool;
        #[inline]
        fn find_map<__U, __F>(&mut self, f: __F) -> ::core::option::Option<__U>
        where
            __F: ::core::ops::FnMut(Self::Item) -> ::core::option::Option<__U>;
        #[inline]
        fn position<__P>(&mut self, predicate: __P) -> ::core::option::Option<usize>
        where
            __P: ::core::ops::FnMut(Self::Item) -> bool;
    };

    derive_trait!(
        parse!(input),
        parse_quote!(::core::iter::Iterator),
        parse_quote! {
            trait Iterator {
                type Item;
                #[inline]
                fn next(&mut self) -> ::core::option::Option<Self::Item>;
                #[inline]
                fn size_hint(&self) -> (usize, ::core::option::Option<usize>);
                #[inline]
                fn count(self) -> usize;
                #[inline]
                fn last(self) -> ::core::option::Option<Self::Item>;
                #[inline]
                fn nth(&mut self, n: usize) -> ::core::option::Option<Self::Item>;
                #[inline]
                #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
                fn collect<__U: ::core::iter::FromIterator<Self::Item>>(self) -> __U;
                #[inline]
                fn partition<__U, __F>(self, f: __F) -> (__U, __U)
                where
                    __U: ::core::default::Default + ::core::iter::Extend<Self::Item>,
                    __F: ::core::ops::FnMut(&Self::Item) -> bool;
                #try_trait
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(DoubleEndedIterator)]
pub fn derive_double_ended_iterator(input: TokenStream) -> TokenStream {
    // TODO: When `try_trait` stabilized, add `try_rfold` and remove `rfold` and `rfind` conditionally.

    // It is equally efficient if `try_rfold` can be used.
    let try_trait = quote! {
        #[inline]
        fn rfold<__U, __F>(self, accum: __U, f: __F) -> __U
        where
            __F: ::core::ops::FnMut(__U, Self::Item) -> __U;
        #[inline]
        fn rfind<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
        where
            __P: ::core::ops::FnMut(&Self::Item) -> bool;
    };

    derive_trait!(
        parse!(input),
        Some(format_ident!("Item")),
        parse_quote!(::core::iter::DoubleEndedIterator),
        parse_quote! {
            trait DoubleEndedIterator: ::core::iter::Iterator {
                #[inline]
                fn next_back(&mut self) -> ::core::option::Option<Self::Item>;
                #try_trait
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(ExactSizeIterator)]
pub fn derive_exact_size_iterator(input: TokenStream) -> TokenStream {
    // TODO: When `exact_size_is_empty` stabilized, add `is_empty` conditionally.

    derive_trait!(
        parse!(input),
        Some(format_ident!("Item")),
        parse_quote!(::core::iter::ExactSizeIterator),
        parse_quote! {
            trait ExactSizeIterator: ::core::iter::Iterator {
                #[inline]
                fn len(&self) -> usize;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(FusedIterator)]
pub fn derive_fused_iterator(input: TokenStream) -> TokenStream {
    derive_trait!(
        parse!(input),
        Some(format_ident!("Item")),
        parse_quote!(::core::iter::FusedIterator),
        parse_quote! {
            trait FusedIterator: ::core::iter::Iterator {}
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[cfg(feature = "trusted_len")]
#[proc_macro_derive(TrustedLen)]
pub fn derive_trusted_len(input: TokenStream) -> TokenStream {
    derive_trait!(
        parse!(input),
        Some(format_ident!("Item")),
        parse_quote!(::core::iter::TrustedLen),
        parse_quote! {
            unsafe trait TrustedLen: ::core::iter::Iterator {}
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(Extend)]
pub fn derive_extend(input: TokenStream) -> TokenStream {
    derive_trait!(
        parse!(input),
        parse_quote!(::core::iter::Extend),
        parse_quote! {
            trait Extend<__A> {
                #[inline]
                fn extend<__T: ::core::iter::IntoIterator<Item = __A>>(&mut self, iter: __T);
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[cfg(feature = "rayon")]
#[proc_macro_derive(ParallelIterator)]
pub fn derive_parallel_iterator(input: TokenStream) -> TokenStream {
    derive_trait!(
        parse!(input),
        parse_quote!(::rayon::iter::ParallelIterator),
        parse_quote! {
            trait ParallelIterator {
                type Item;
                #[inline]
                fn drive_unindexed<__C>(self, consumer: __C) -> __C::Result
                where
                    __C: ::rayon::iter::plumbing::UnindexedConsumer<Self::Item>;
                #[inline]
                fn opt_len(&self) -> ::core::option::Option<usize>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[cfg(feature = "rayon")]
#[proc_macro_derive(IndexedParallelIterator)]
pub fn derive_indexed_parallel_iterator(input: TokenStream) -> TokenStream {
    derive_trait!(
        parse!(input),
        Some(format_ident!("Item")),
        parse_quote!(::rayon::iter::IndexedParallelIterator),
        parse_quote! {
            trait IndexedParallelIterator: ::rayon::iter::ParallelIterator {
                #[inline]
                fn drive<__C>(self, consumer: __C) -> __C::Result
                where
                    __C: ::rayon::iter::plumbing::Consumer<Self::Item>;
                #[inline]
                fn len(&self) -> usize;
                #[inline]
                fn with_producer<__CB>(self, callback: __CB) -> __CB::Output
                where
                    __CB: ::rayon::iter::plumbing::ProducerCallback<Self::Item>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[cfg(feature = "rayon")]
#[proc_macro_derive(ParallelExtend)]
pub fn derive_parallel_extend(input: TokenStream) -> TokenStream {
    derive_trait!(
        parse!(input),
        parse_quote!(::rayon::iter::ParallelExtend),
        parse_quote! {
            trait ParallelExtend<__T: Send> {
                #[inline]
                fn par_extend<__I>(&mut self, par_iter: __I)
                where
                    __I: ::rayon::iter::IntoParallelIterator<Item = __T>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}
