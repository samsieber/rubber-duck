use quote::quote;
use std::collections::HashMap;
use std::collections::HashSet;
use syn::parse::Parser;
use syn::{Attribute, Expr, Lit, Meta, MetaNameValue, NestedMeta};

pub fn parse_args(args: ::proc_macro::TokenStream) -> Args {
    let upper = format!("#[parsing_wrapper({})]", args);
    let attr = Attribute::parse_outer
        .parse_str(&upper)
        .unwrap()
        .pop()
        .unwrap();

    if let Meta::List(list) = attr.interpret_meta().unwrap() {
        let mut args = Args {
            defaults: HashMap::new(),
            positional: vec![],
        };
        let mut processed: HashSet<String> = HashSet::new();
        for nested in list.nested {
            match nested {
                NestedMeta::Meta(value) => {
                    let raw_name = format!("{}", &value.name());
                    if processed.contains(&raw_name) {
                        panic!("Double annotations for {}", raw_name);
                    } else {
                        processed.insert(raw_name.clone());
                    }
                    if raw_name == "defaults" {
                        args.defaults = process_defaults(value);
                    } else if raw_name == "on_struct" {
                        unimplemented!()
                    } else if raw_name == "on_fields" {
                        unimplemented!()
                    } else if raw_name == "positionals" {
                        args.positional = process_positionals(value);
                    } else {
                        panic!()
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

fn process_positionals(meta: Meta) -> Vec<String> {
    if let Meta::List(list) = meta {
        list.nested
            .iter()
            .map(|value| match value {
                NestedMeta::Meta(value) => extract_positional_name(value.clone()),
                _ => panic!("Incorrect macro format! - bad nesting"),
            }).collect()
    } else {
        panic!("Cannot parse defaults");
    }
}

fn extract_positional_name(meta: Meta) -> String {
    match meta {
        Meta::Word(word) => format!("{}", word),
        _ => panic!(
            "Wrong format for specifying positional name. Expected an identifier, got '{}'",
            quote!(meta)
        ),
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
        Meta::NameValue(MetaNameValue { ident, lit, .. }) => (format!("{}", ident), as_expr(lit)),
        _ => panic!("not a good format for the default value"),
    }
}

pub fn as_expr(lit: Lit) -> Expr {
    if let Lit::Str(lit_str) = lit {
        lit_str.parse::<Expr>().unwrap()
    } else {
        panic!("Not a string literal!");
    }
}

#[derive(Debug)]
pub struct Args {
    pub defaults: HashMap<String, Expr>,
    pub positional: Vec<String>,
}
