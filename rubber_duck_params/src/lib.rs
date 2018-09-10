#![recursion_limit = "128"]
#![feature(proc_macro_gen)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use quote::quote;
use syn::parse_quote;
use syn::{parse_macro_input, Item, ItemFn};

mod args;
mod build;
mod parse_fn;
mod util;

#[proc_macro_attribute]
pub fn gen_struct_sugar(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut args = crate::args::parse_args(args);

    let generated_parts = {
        let mut parsed: Item = parse_macro_input!(input as Item);
        let (struct_dec, macro_dec) = match parsed {
            Item::Fn(ref mut fn_item) => {
                let structure = parse_fn::parse_field_decl(&mut args, fn_item);

                let struct_dec = build::gen_builder(&structure);

                let macro_dec = build::gen_macro(&structure);

                build::mod_block_add_defaults(fn_item, &structure);

                (struct_dec, macro_dec)
            }
            _ => {
                panic!("Can only be used on free-standing fn declarations");
            }
        };

        vec![struct_dec, quote!(#parsed), macro_dec]
    };

    let generated = quote!(#(#generated_parts)*);

//    println!("{}", &generated);

    generated.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
