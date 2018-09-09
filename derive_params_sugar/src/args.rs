use std::collections::HashMap;
use syn::parse::Parser;
use syn::Expr;
use syn::{Attribute, Meta, MetaNameValue, NestedMeta};

pub fn parse_args(args: ::proc_macro::TokenStream) -> Args {
    let upper = format!("#[stuff({})]", args);
    let attr = Attribute::parse_outer
        .parse_str(&upper)
        .unwrap()
        .pop()
        .unwrap();
    if let Meta::List(list) = attr.interpret_meta().unwrap() {
        let mut args = Args {
            defaults: HashMap::new(),
        };
        for nested in list.nested {
            match nested {
                NestedMeta::Meta(value) => {
                    if format!("{}", &value.name()) == "defaults" {
                        args.defaults = process_defaults(value);
                    }
                }
                _ => panic!("Incorrect macro format! - bad nesting"),
            }
        }
        args
    } else {
        panic!("Incorrect macro format! - not a list in args");
    }
}

fn process_defaults(meta: Meta) -> HashMap<String, Expr> {
    if let Meta::List(list) = meta {
        list.nested
            .iter()
            .map(|value| match value {
                NestedMeta::Meta(value) => extract_default(value.clone()),
                _ => panic!("Incorrect macro format! - bad nesting"),
            }).collect()
    } else {
        panic!("Cannot parse defaults");
    }
}

pub fn extract_default(m: Meta) -> (String, Expr) {
    match m {
        Meta::NameValue(MetaNameValue { ident, lit, .. }) => {
            (format!("{}", ident), ::default::as_expr(lit))
        }
        _ => panic!("not a good format for the default value"),
    }
}

#[derive(Debug)]
pub struct Args {
    pub defaults: HashMap<String, Expr>,
}
