use crate::args::Args;
use proc_macro2::Ident;
use quote::quote;
use std::collections::HashMap;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Expr;
use syn::FnArg;
use syn::ItemFn;
use syn::Pat;
use syn::Type;

#[derive(Clone)]
pub struct PositionalField {
    pub name: Ident,
    pub ty: Type,
}

#[derive(Clone)]
pub struct NamedField {
    pub name: Ident,
    pub ty: Type,
    pub default: Option<Expr>,
}

#[derive(Clone)]
pub struct Structure {
    pub positional: Vec<PositionalField>,
    pub named: Vec<NamedField>,
    pub ident: Ident,
}

#[allow(dead_code)]
impl Structure {
    pub fn names(&self) -> Vec<Ident> {
        let mut p = self.p_names();
        let mut n = self.n_names();

        p.append(&mut n);

        p
    }

    pub fn p_names(&self) -> Vec<Ident> {
        self.positional.iter().map(|p| p.name.clone()).collect()
    }

    pub fn n_names(&self) -> Vec<Ident> {
        self.named.iter().map(|n| n.name.clone()).collect()
    }

    pub fn types(&self) -> Vec<Type> {
        let mut p = self.p_types();
        let mut n = self.n_types();

        p.append(&mut n);

        p
    }

    pub fn p_types(&self) -> Vec<Type> {
        self.positional.iter().map(|p| p.ty.clone()).collect()
    }

    pub fn n_types(&self) -> Vec<Type> {
        self.named.iter().map(|n| n.ty.clone()).collect()
    }
}

/// Parses the field declaration _and_ modifies signature of any parameters that are optional
pub fn parse_field_decl(args: &mut Args, fn_item: &mut ItemFn) -> Structure {
    let ident = fn_item.ident.clone();
    let decl = &mut fn_item.decl;
    let unvalidated_fields = parse_fields(&mut decl.inputs, &mut args.defaults);

    let mut positional = vec![];
    let mut named = vec![];
    let mut positional_iter = args.positional.iter();

    for unvalidated in unvalidated_fields {
        let name_string = format!("{}", unvalidated.name);
        if let Some(next_positional_name) = positional_iter.next() {
            if &name_string != next_positional_name {
                panic!(
                    "The positional field list must be in the same order as the declared fields"
                );
            }
            if unvalidated.default.is_some() {
                panic!("Cannot set a default value for a positional field");
            }
            positional.push(PositionalField {
                name: unvalidated.name,
                ty: unvalidated.ty,
            })
        } else {
            named.push(NamedField {
                name: unvalidated.name,
                ty: unvalidated.ty,
                default: unvalidated.default,
            })
        }
    }

    Structure {
        positional,
        named,
        ident,
    }
}

struct UnvalidatedField {
    pub name: Ident,
    pub ty: Type,
    pub default: Option<Expr>,
}

fn parse_fields(
    args: &mut Punctuated<FnArg, Comma>,
    defaults: &mut HashMap<String, Expr>,
) -> Vec<UnvalidatedField> {
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

                    UnvalidatedField {
                        name: pat.ident.clone(),
                        ty: arg.ty.clone(),
                        default,
                    }
                }
                _ => panic!(
                    "Non-ident patterns not supported in fn signature, {:?}",
                    arg
                ),
            },
            _ => panic!("Unsupported fn arg type!"),
        }).collect()
}
