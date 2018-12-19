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
pub enum FieldRole {
    Named(NamedData),
    Positional,
}

#[derive(Clone)]
pub struct NamedData {
    pub default: Option<Expr>,
}


#[derive(Clone)]
pub struct Field<T> {
    pub name: Ident,
    pub ty: Type,
    pub extra: T,
}

#[derive(Clone)]
pub struct Structure {
    pub fields: Vec<Field<FieldRole>>,
    pub ident: Ident,
}

impl Field<FieldRole> {
    pub fn has_default(&self) -> bool {
        match self.extra {
            FieldRole::Named(ref def) => def.default.is_some(),
            FieldRole::Positional => false,
        }
    }

    pub fn default_expr(&self) -> Option<Expr> {
        match self.extra {
            FieldRole::Named(ref def) => def.default.clone(),
            FieldRole::Positional => None,
        }
    }

    pub fn is_positional(&self) -> bool {
        match self.extra {
            FieldRole::Named(_) => false,
            FieldRole::Positional => true,
        }
    }
}

#[allow(dead_code)]
impl Structure {
    pub fn names(&self) -> Vec<Ident> {
        self.fields.iter().map(|v| v.name.clone()).collect()
    }

    pub fn types(&self) -> Vec<Type> {
        self.fields.iter().map(|v| v.ty.clone()).collect()
    }

    pub fn positional(&self) -> impl Iterator<Item=&Field<FieldRole>> {
        self.fields.iter().filter(|&v| match v.extra {
            FieldRole::Positional => true,
            _ => false}
        )
    }

    pub fn named(&self) -> Vec<Field<NamedData>> {
        self.fields.iter().filter_map(|v| match &v.extra {
            FieldRole::Positional => None,
            FieldRole::Named(ref def) => Some(Field {
                name: v.name.clone(),
                ty: v.ty.clone(),
                extra: def.clone(),
            })
        }).collect()
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
            positional.push(Field {
                name: unvalidated.name,
                ty: unvalidated.ty,
                extra: FieldRole::Positional
            })
        } else {
            named.push(Field {
                name: unvalidated.name,
                ty: unvalidated.ty,
                extra: FieldRole::Named(NamedData { default: unvalidated.default}),
            })
        }
    }

    let fields = positional.into_iter().chain(named.into_iter()).collect();

    Structure {
        fields,
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
