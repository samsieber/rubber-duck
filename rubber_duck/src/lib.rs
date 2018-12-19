#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

//! # Overview
//! This crate provides a psuedo-implementation of one of many named argument RFCs.
//! It's not exactly like the real thing, given that macros are used instead of real compiler support.
//! This crate provides an attribute-like proc macro to annotate a function with to generate named argument support.
//!
//! That annotation does the following:
//!  * Generates a struct with the same arguments as the function
//!     * along with an associated builder() function
//!     * and this struct is hidden from docs by default
//!  * Generates a builder for said struct method
//!     * It's a type-safe builder, and requires that each method without a default value must be called
//!     * The build function converts the builder back into the argument struct
//!     * This is strcut is also doc hidden
//!  * Default values can be specified in the annotation for the fields, for each
//!     * The field type is wrapped in an Option (e.g. T changes from T to Option<T>)
//!     * A default value of None is specified on the builder
//!     * And with type of Option<T>, the builder setter for the field accepts T or Option<T>
//!  * The positional argumentss can be specified as well (no defaults allowed for those)
//!  * A macro is generated with the same name as the function, which accepts named parameters
//!     * creates the builder
//!     * sets whatever arguments are passed in
//!     * calls build (which will be a compile error if not all named values are provided if the lack defaults)
//!
//!  So how this all works out is that we overload the method name in several namespaces (fn, struct and macro),
//!  such that the consumer can just us the plain fn or the macro
//!
//! ## Limitations
//!
//! This implementation punts on the declaration side - e.g. how to declare the parameters as being named, and how to specify
//! parameter defaults. It instead specifies all those in the attribute to avoid parsing issues. A future version could switch
//! from an attribute-like proc macro to a function-like proc macro to actually experiment with delcaration syntax
//!
//! This version doesn't deal well with generics. That seems like a solvable limitation in this approach
//!
//! This version only works with stand alone functions, not functions in impl blocks. That should be solveable as well.
//!
//! This version requires nightly (for decl. macro 2.0 and proc_macro_gen) and the 2018 edition (due to macro paths).
//! While proc_macro_gen might be stabalized soon, decl. macros 2.0 won't (I think). Decl. Macros 2.0 were used instead of
//! macro_rules to get proper module namespacing support for the generated macros
//!
//! ### Probably bug imposed
//!
//! The macros exported from rubber_duck don't appear in the rubber_duck docs
//!
//! The macros generated for the api declarer don't get shown in the correct module in the generated docs
//! [Github issue #54112](https://github.com/rust-lang/rust/issues/54112)
//!
//! # Using
//! ## Prerequisites:
//! * Use edition 2018 (macro paths between editions interact very poorly with what I'm doing)
//! * Use nightly
//!
//! ### Writing an API
//! #### Setup
//! Add this at the top of your lib.rs if you're implementing an API and want to expose
//!
//! ```
//! // Allows us to generate macros from macros
//! #![feature(proc_macro_hygiene)]
//! // Allows the use of declarative macros 2.0 (which are generated from the proc macro
//! #![feature(decl_macro)]
//!
//! // Hide the base items the macro internals use
//! #[doc(hidden)]
//! // Import the base items the macro internals use
//! // This must be at the base of the crate
//! pub use rubber_duck::core::*;
//! ```
//!
//! #### Annotating Functions
//! To make a function callable in macro named syntax, in the same mod as the function, import the macros:
//!
//! ```
//! use ::rubber_duck::macros::*;
//! ```
//!
//! Then annotate a function with `#[gen_struct_sugar]` - that will be enough to generate a macro with
//! that can be called with `named => value` syntax.
//!
//! Furthermore, you can declare a) positional parameters and b) default values for named parameters
//!
//! ```
//! #[gen_struct_sugar(
//!        defaults( // You don't have to set defaults for all named parameters, but here we do
//!            read = "false",
//!            write = "false",
//!            append = "false",
//!            truncate = "false",
//!            create = "false",
//!            create_new = "false",
//!        ),
//!        positionals(path), // Here we list the positional parameters in the order they appear
//!    )]
//!    pub fn open_file(
//!        path: impl AsRef<Path>,
//!        read: bool,
//!        write: bool,
//!        append: bool,
//!        truncate: bool,
//!        create: bool,
//!        create_new: bool,
//!    ) -> std::io::Result<File> {
//!        OpenOptions::new()
//!            .read(read)
//!            .write(write)
//!            .append(append)
//!            .truncate(truncate)
//!            .create(create)
//!            .create_new(create_new)
//!            .open(path)
//!    }
//! ```
//!
//! ### Consuming an API
//! To consume an API, one can
//!  a) call the method, like normal - any methods that have default values of type T will actually
//!      be of type Option<T>
//!  b) call the method with a bang `!` - any positional parameters must go first in proper order without names,
//!     the named parameters go next in the form of `name => value`, where name is the publish arg name in the docs
//!
//! Furthermore, consumer is in a separate crate from the writer, they don't need any feature flags enabled
//!
//! They still need to be on nightly though and probably 2018 (because of paths...)
//!
//! #### Example:
//!
//! Given api declaration:
//!
//! ```
//! mod module {
//!   >    #[gen_struct_sugar(defaults(name = r#""Bob".to_owned()"#))]
//!   >    pub fn is_a_test(name: String, message: String) -> String {
//!       >        let i = 0;
//!       >        let i = i + 1;
//!       >        format!("{}) Hello {}, {} The end.", i, &name, &message)
//!           >    }
//!   > }
//! ```
//!
//! One can call the function in a variety of ways
//!
//! ```
//! {
//!     use crate::module::is_a_test
//!      // Named form requires a macro
//!     is_a_test!(message=> "Hi.".to_owned(), name=>"George".to_owned());  // 1) Hello George. Hi. The end.
//!      // and lets you use defaults
//!     is_a_test!(message=> "Hi.".to_owned());                             // 1) Hello Bob. Hi. The end.
//!      // Positional form doesn't (need a macro or let you do defaults)
//!     is_a_test("bob".to_owned(), "Hi.".to_owned());                      // 1) Hello Bob. Hi. The end.
//! }
//! // You don't even have to import it!
//! crate::module::is_a_test!(message=> "there".to_owned(), name=>"hi".to_owned());
//! ```

pub mod macros {
    pub use rubber_duck_macro::*;
}

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