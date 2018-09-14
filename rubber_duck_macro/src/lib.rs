#![recursion_limit = "128"]
#![feature(proc_macro_gen)]

mod call;
mod builder;

#[proc_macro_attribute]
pub fn gen_typesafe_builder(
  args: ::proc_macro::TokenStream,
  input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
  builder::gen_typesafe_builder(args, input)
}

#[proc_macro_attribute]
pub fn gen_struct_sugar(
  args: ::proc_macro::TokenStream,
  input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
  call::gen_struct_sugar(args, input)
}