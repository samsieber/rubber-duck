#![feature(proc_macro_gen)]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
use proc_macro::TokenStream;
use std::collections::HashMap;
use syn::export::Span;
use syn::export::ToTokens;
use syn::parse::Parser;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, Attribute, Expr, Field, Fields, FnArg, Ident, Item, ItemFn, Lit, Meta,
    MetaNameValue, NestedMeta, Pat, Type,
};

mod args;
mod default;
mod util;

#[proc_macro_attribute]
pub fn gen_struct_sugar(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut args = ::args::parse_args(args);

    println!("{:#?}", &args);

    let generated_parts = {
        let mut parsed: Item = parse_macro_input!(input as Item);
        let mut parts = match parsed {
            Item::Fn(ref mut fn_item) => {
                //                let mut parsed_meta = parse_attrs(fn_item);
                let mut parsed_fields = parse_fields(&mut fn_item.decl.inputs, &mut args.defaults);

                let mut parts = vec![{
                    let struct_name = Ident::new(
                        &::util::uppercase(&format!("{}", fn_item.ident)),
                        fn_item.ident.span(),
                    );

                    let field_decs = parsed_fields.iter().map(|f| {
                        let name = &f.name;
                        let ty = &f.ty;
                        let attr = if let Some(ref default) = f.default {
                            quote!(#[default = "None"])
                        } else {
                            quote!()
                        };

                        quote!(
                            #attr
                            pub #name: #ty,
                            )
                    });

                    quote!(
                        #[gen_typesafe_builder]
                        pub struct #struct_name {
                            #(#field_decs)*
                        }
                    )
                }];

                parts.push({
                    let types = parsed_fields.iter().map(|f| &f.ty);
                    let names = parsed_fields.iter().map(|f| &f.name);
                    let name = &fn_item.ident;
                    let struct_name = Ident::new(
                        &::util::uppercase(&format!("{}", fn_item.ident)),
                        fn_item.ident.span(),
                    );

                    let args = parsed_fields.iter().map(|f| &f.name);


                    quote!(
                        /// Calls [is_a_test](fn.is_a_test.html), but uses a different name format

                        pub macro #name( $($name:ident => $value:expr),*) {
                            {{
                                let __temp = #struct_name::builder()
                                    $(.$name($value))*
                                    .build();
                                #name(
                                    #(__temp.#args),*
                                )
                            }}
                        }
                    )
                });

                let statements = fn_item.block.stmts.clone();
                let has_defaults: Vec<_> = parsed_fields
                    .iter()
                    .filter(|&f| f.default.is_some())
                    .map(|v| v.clone())
                    .collect();
                let names1 = has_defaults.iter().map(|v| &v.name);
                let names2 = has_defaults.iter().map(|v| &v.name);
                let expr = has_defaults.iter().map(|v| v.default.as_ref().unwrap());
                //                 let #name = if #name.is_some() {
                //                     #name
                //                 } else {
                //                     #expr
                //                 }
                let block = parse_quote!(
                    {
                        #(let #names1 = if let Some(__var) = #names2{
                             __var
                         } else {
                             #expr
                         };)*
                        #(#statements)*
                    }
                );

                fn_item.block = Box::new(block);

                parts
            }
            _ => {
                panic!("Can only be used on free-standing fn declarations");
            }
        };
        parts.push(quote!(#parsed));
        parts
    };

    let generated = quote!(#(#generated_parts)*);

    println!("{}", &generated);

    generated.into()
}

#[derive(Clone)]
struct FieldDec {
    ty: Type,
    name: Ident,
    default: Option<Expr>,
}

enum AttrLocation {
    Struct,
    Field(String),
}

struct ParsedAttr {
    location: AttrLocation,
    content: Meta,
}

fn parse_meta(meta: &Meta) -> ParsedAttr {
    let res: ParsedAttr = match meta {
        Meta::List(meta) => {
            if "for_field" == format!("{}", meta.ident) {
                let mut iter = meta.nested.iter();
                if let Some(first) = iter.next() {
                    let field_name = match first {
                        NestedMeta::Literal(first) => format!("{:?}", first),
                        NestedMeta::Meta(first) => match first {
                            Meta::Word(ident) => format!("{:?}", first),
                            _ => panic!("Expected a string literal or single identifier"),
                        },
                    };
                    if let Some(second) = iter.next() {
                        let parsed = ParsedAttr {
                            location: AttrLocation::Field(field_name),
                            content: match second {
                                NestedMeta::Literal(_) => panic!(
                                    "Cannot use a string literal for the annotation: {:?}",
                                    second
                                ),
                                NestedMeta::Meta(second) => second.clone(),
                            },
                        };
                        if iter.next().is_some() {
                            panic!("Too many items in the attribute list: {:?}", meta)
                        }
                        parsed
                    } else {
                        panic!("Not enough items in attribute list");
                    }
                } else {
                    panic!("Not enough values passed to 'for_field'")
                }
            } else if "for_struct" == format!("{}", meta.ident) {
                let mut iter = meta.nested.iter();
                if let Some(first) = iter.next() {
                    let parsed = ParsedAttr {
                        location: AttrLocation::Struct,
                        content: match first {
                            NestedMeta::Literal(_) => panic!(
                                "Cannot use a string literal for the annotation: {:?}",
                                first
                            ),
                            NestedMeta::Meta(nested_meta) => nested_meta.clone(),
                        },
                    };
                    if iter.next().is_some() {
                        panic!("Too many items in the attribute list: {:?}", &meta)
                    } else {
                        parsed
                    }
                } else {
                    panic!("Not enough items in the attribute list: {:?}", &meta)
                }
            } else {
                panic!("Unknown attribute! {:?}", &meta.ident);
            }
        }
        _ => panic!("Unknown field {:?}", &meta),
    };
    return res;
}

fn parse_attrs(args: &mut ItemFn) -> Vec<ParsedAttr> {
    ::util::drain_map(&mut args.attrs, |attr| {
        let meta = attr.interpret_meta();
        if let Some(meta) = meta {
            Some(parse_meta(&meta))
        } else {
            panic!("Not a parseable meta: {:?}", meta);
        }
    })
}

fn parse_fields(
    args: &mut Punctuated<FnArg, Comma>,
    defaults: &mut HashMap<String, Expr>,
) -> Vec<FieldDec> {
    args.iter_mut()
        .map(|arg| match arg {
            FnArg::Captured(ref mut arg) => match arg.pat {
                Pat::Ident(ref mut pat) => {
                    if pat.by_ref.is_some() {
                        panic!("By ref not handled yet for argument")
                    }
                    if pat.mutability.is_some() {
                        panic!("Mut not handled yet for argument")
                    }
                    if pat.subpat.is_some() {
                        panic!("Subpattern not supported yet for argument")
                    }

                    let default = defaults.remove(&format!("{}", &pat.ident));

                    if default.is_some() {
                        let new_ty = {
                            let ty = arg.ty.clone();
                            let new_ty: Type = parse_quote!(Option<#ty>);
                            new_ty
                        };
                        arg.ty = new_ty;
                    };

                    FieldDec {
                        name: pat.ident.clone(),
                        ty: arg.ty.clone(),
                        default: default,
                    }
                }
                _ => panic!("Non-ident patterns not supported in fn signature"),
            },
            _ => panic!("Unsupported fn arg type!"),
        }).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
