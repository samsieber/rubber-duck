#![recursion_limit = "128"]

extern crate proc_macro;

mod call;
mod builder;
mod util;
mod args;
mod build;
mod parse_fn;

use quote::quote;
use syn::{parse_macro_input, Item};
use proc_macro_hack::proc_macro_hack;

#[proc_macro_attribute]
pub fn gen_struct_sugar(
  args: ::proc_macro::TokenStream,
  input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
  let mut args = crate::args::parse_args(args);

  let generated_parts = {
    let mut parsed: Item = parse_macro_input!(input as Item);
    let structure = match parsed {
      Item::Fn(ref mut fn_item) => {
        let structure = parse_fn::parse_field_decl(&mut args, fn_item);
        build::mod_block_add_defaults(fn_item, &structure);
        structure
      }
      _ => {
        panic!("Can only be used on free-standing fn declarations");
      }
    };

    let builder_impl = builder::create_typesafe_builder(&structure);

    let mut generated_parts = vec![builder_impl, quote!(#parsed)];

    if cfg!(feature = "nightly") {
      let macro_dec = build::gen_macro(&structure);
      generated_parts.push(macro_dec)
    }

    generated_parts
  };

  let generated = quote!(#(#generated_parts)*);

  generated.into()
}

/// Add one to an expression.
#[proc_macro_hack]
pub fn n(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  call::n(input)
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
