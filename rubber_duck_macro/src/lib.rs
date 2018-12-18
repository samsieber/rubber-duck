#![recursion_limit = "128"]

extern crate proc_macro;

mod call;
mod builder;
mod util;
mod args;
mod build;
mod parse_fn;

#[proc_macro_attribute]
pub fn gen_struct_sugar(
  args: ::proc_macro::TokenStream,
  input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
  call::gen_struct_sugar(args, input)
}