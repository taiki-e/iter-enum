//! \#\[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)\] for enums.
//!
//! ## Examples
//!
//! ```rust
//! # #![cfg_attr(feature = "try_trait", feature(try_trait))]
//! # #![cfg_attr(feature = "exact_size_is_empty", feature(exact_size_is_empty))]
//! # extern crate core;
//! # extern crate iterator_enum;
//! use iterator_enum::*;
//!
//! #[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, FusedIterator, Extend)]
//! enum Either<A, B> {
//!     A(A),
//!     B(B),
//! }
//!
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     if x > 0 {
//!         Either::A(x..=0)
//!     } else {
//!         Either::B(Some(x).into_iter())
//!     }
//! }
//! ```
//!
//! See [auto_enums](https://github.com/taiki-e/auto_enums) for how to automate patterns like this.
//!
//! ## Supported traits
//!
//! * [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
//! * [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html)
//! * [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html)
//! * [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html)
//! * [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html) (*requires `"trusted_len"` crate feature*)
//! * [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html)
//!
//! See [this issue](https://github.com/taiki-e/auto_enums/issues/11) for other traits.
//!
//! ## Crate Features
//!
//! * `std`
//!   * Enabled by default.
//!   * Generate code for `std` library.
//!   * Disable this feature to generate code for `no_std`.
//!
//! * `try_trait`
//!   * Disabled by default.
//!   * Make iterator implementation more effective.
//!   * This requires Rust Nightly and you need to enable the unstable [`try_trait`](https://github.com/rust-lang/rust/issues/42327) feature gate.
//!
//! * `trusted_len`
//!   * Disabled by default.
//!   * Use `#[derive(TrustedLen)]`.
//!   * This requires Rust Nightly and you need to enable the unstable [`trusted_len`](https://github.com/rust-lang/rust/issues/37572) feature gate.
//!
//! * `exact_size_is_empty`
//!   * Disabled by default.
//!   * Implements `ExactSizeIterator::is_empty`.
//!   * This requires Rust Nightly and you need to enable the unstable [`exact_size_is_empty`](https://github.com/rust-lang/rust/issues/35428) feature gate.
//!

#![crate_type = "proc-macro"]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/iterator-enum/0.1.2")]
#![deny(unsafe_code)]
#![deny(bare_trait_objects, elided_lifetimes_in_paths, unreachable_pub)]

extern crate derive_utils;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use derive_utils::{derive_trait, EnumData as Data, __rt::ident_call_site};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;

/// Returns standard library's root.
///
/// In default returns `::std`.
/// if disabled default crate feature, returned `::core`.
macro_rules! std_root {
    () => {{
        #[cfg(feature = "std")]
        {
            quote!(::std)
        }
        #[cfg(not(feature = "std"))]
        {
            quote!(::core)
        }
    }};
}

macro_rules! parse {
    ($input:expr) => {
        match syn::parse($input)
            .map_err(derive_utils::Error::from)
            .and_then(|item| Data::from_derive(&item))
        {
            Ok(data) => data,
            Err(err) => return TokenStream::from(err.to_compile_error()),
        }
    };
}

#[proc_macro_derive(Iterator)]
pub fn derive_iterator(input: TokenStream) -> TokenStream {
    let data = parse!(input);
    let root = std_root!();

    #[cfg(feature = "try_trait")]
    let try_trait = quote! {
        #[inline]
        fn try_fold<__U, __F, __R>(&mut self, init: __U, f: __F) -> __R
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __R,
            __R: #root::ops::Try<Ok = __U>;
    };
    // It is equally efficient if `try_fold` can be used.
    #[cfg(not(feature = "try_trait"))]
    let try_trait = quote! {
        #[inline]
        fn fold<__U, __F>(self, init: __U, f: __F) -> __U
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __U;
        #[inline]
        fn all<__F>(&mut self, f: __F) -> bool
        where
            __F: #root::ops::FnMut(Self::Item) -> bool;
        #[inline]
        fn any<__F>(&mut self, f: __F) -> bool
        where
            __F: #root::ops::FnMut(Self::Item) -> bool;
        #[inline]
        fn find<__P>(&mut self, predicate: __P) -> #root::option::Option<Self::Item>
        where
            __P: #root::ops::FnMut(&Self::Item) -> bool;
        #[inline]
        fn find_map<__U, __F>(&mut self, f: __F) -> #root::option::Option<__U>
        where
            __F: #root::ops::FnMut(Self::Item) -> #root::option::Option<__U>;
        #[inline]
        fn position<__P>(&mut self, predicate: __P) -> #root::option::Option<usize>
        where
            __P: #root::ops::FnMut(Self::Item) -> bool;
    };

    derive_trait!(
        data,
        parse_quote!(#root::iter::Iterator),
        parse_quote! {
            trait Iterator {
                type Item;
                #[inline]
                fn next(&mut self) -> #root::option::Option<Self::Item>;
                #[inline]
                fn size_hint(&self) -> (usize, #root::option::Option<usize>);
                #[inline]
                fn count(self) -> usize;
                #[inline]
                fn last(self) -> #root::option::Option<Self::Item>;
                #[inline]
                fn nth(&mut self, n: usize) -> #root::option::Option<Self::Item>;
                #[inline]
                #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
                fn collect<__U: #root::iter::FromIterator<Self::Item>>(self) -> __U;
                #[inline]
                fn partition<__U, __F>(self, f: __F) -> (__U, __U)
                where
                    __U: #root::default::Default + #root::iter::Extend<Self::Item>,
                    __F: #root::ops::FnMut(&Self::Item) -> bool;
                #try_trait
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(DoubleEndedIterator)]
pub fn derive_double_ended_iterator(input: TokenStream) -> TokenStream {
    let data = parse!(input);
    let root = std_root!();

    #[cfg(feature = "try_trait")]
    let try_trait = quote! {
        #[inline]
        fn try_rfold<__U, __F, __R>(&mut self, init: __U, f: __F) -> __R
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __R,
            __R: #root::ops::Try<Ok = __U>;
    };
    // It is equally efficient if `try_rfold` can be used.
    #[cfg(not(feature = "try_trait"))]
    let try_trait = quote! {
        #[inline]
        fn rfold<__U, __F>(self, accum: __U, f: __F) -> __U
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __U;
        #[inline]
        fn rfind<__P>(&mut self, predicate: __P) -> #root::option::Option<Self::Item>
        where
            __P: #root::ops::FnMut(&Self::Item) -> bool;
    };

    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        parse_quote!(#root::iter::DoubleEndedIterator),
        parse_quote! {
            trait DoubleEndedIterator: #root::iter::Iterator {
                #[inline]
                fn next_back(&mut self) -> #root::option::Option<Self::Item>;
                #try_trait
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(ExactSizeIterator)]
pub fn derive_exact_size_iterator(input: TokenStream) -> TokenStream {
    let data = parse!(input);
    let root = std_root!();

    #[cfg(not(feature = "exact_size_is_empty"))]
    let is_empty = quote!();
    #[cfg(feature = "exact_size_is_empty")]
    let is_empty = quote! {
        #[inline]
        fn is_empty(&self) -> bool;
    };

    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        parse_quote!(#root::iter::ExactSizeIterator),
        parse_quote! {
            trait ExactSizeIterator: #root::iter::Iterator {
                #[inline]
                fn len(&self) -> usize;
                #is_empty
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(FusedIterator)]
pub fn derive_fused_iterator(input: TokenStream) -> TokenStream {
    let data = parse!(input);
    let root = std_root!();

    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        parse_quote!(#root::iter::FusedIterator),
        parse_quote! {
            trait FusedIterator: #root::iter::Iterator {}
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[cfg(feature = "trusted_len")]
#[proc_macro_derive(TrustedLen)]
pub fn derive_trusted_len(input: TokenStream) -> TokenStream {
    let data = parse!(input);
    let root = std_root!();

    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        parse_quote!(#root::iter::TrustedLen),
        parse_quote! {
            unsafe trait TrustedLen: #root::iter::Iterator {}
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(Extend)]
pub fn derive_extend(input: TokenStream) -> TokenStream {
    let data = parse!(input);
    let root = std_root!();

    derive_trait!(
        data,
        parse_quote!(#root::iter::Extend),
        parse_quote! {
            trait Extend<__A> {
                #[inline]
                fn extend<__T: #root::iter::IntoIterator<Item = __A>>(&mut self, iter: __T);
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}
