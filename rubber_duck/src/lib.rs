#![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]
#![cfg_attr(feature = "nightly", feature(decl_macro))]

/// You'll want to glob import this in whatever module you're defining functions to be callable in named/default arg syntax
pub mod macros {
    pub use rubber_duck_macro::*;
}

/// You'll need to glob import this in the root of the crate where you're defining functions to be callable.
pub mod core{
    pub trait AsOption<T> {
        fn as_option(self) -> ::std::option::Option<T>;
    }
    impl<T> AsOption<T> for ::std::option::Option<T> {
        fn as_option(self) -> ::std::option::Option<T> {
            self
        }
    }
    impl<T> AsOption<T> for T {
        fn as_option(self) -> ::std::option::Option<T> {
            Some(self)
        }
    }

    pub struct Unset;

    pub use crate::{Call, Deconstruct};
}

pub trait Call<Args, Res> {
    fn apply(&self, args: Args) -> Res;
}

pub trait Deconstruct<Args> {
    fn deconstruct(self) -> Args;
}

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
#[doc = "This allows one to call a function in named/default arg syntax (if support for it was derived)"]
pub use rubber_duck_macro::n;

macro_rules! impl_call {
    ($($TT:ident),*) => {
        #[allow(non_snake_case)]
        impl <$($TT,)* R, FN> Call<($($TT,)*), R> for FN where FN: Fn($($TT,)*) -> R {

            fn apply(&self, ($($TT,)*): ($($TT,)*)) -> R {
                self($($TT,)*)
            }
        }
    }
}

impl_call!(A);
impl_call!(A,B);
impl_call!(A,B,C);
impl_call!(A,B,C,D);
impl_call!(A,B,C,D,E);
impl_call!(A,B,C,D,E,F);
impl_call!(A,B,C,D,E,F,G);
impl_call!(A,B,C,D,E,F,G,H);
impl_call!(A,B,C,D,E,F,G,H,I);

#[cfg(test)]
mod test {
    use crate::*;

    fn test_call(name: String) -> String {
        name
    }

    #[test]
    fn test() {
        let tt = test_call;
        tt.apply(("Hello".to_string(),));
    }
}