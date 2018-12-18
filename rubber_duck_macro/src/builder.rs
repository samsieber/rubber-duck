use quote::quote;
use syn::export::Span;
use syn::export::ToTokens;
use syn::AngleBracketedGenericArguments;
use syn::PathSegment;
use syn::{
    Expr, Field, Fields, GenericArgument, Ident, Item, Lit, Meta, MetaNameValue,
    PathArguments, Type,
};
use crate::parse_fn::Structure;
use crate::parse_fn::FieldRole;
use proc_macro2::TokenStream;

enum IsOption {
    True(Type),
    False,
}

fn extract_option_type(wrapped_type: &AngleBracketedGenericArguments) -> Type {
    let value = wrapped_type.args.iter().next().unwrap();
    match value {
        GenericArgument::Type(ty) => ty.clone(),
        _ => panic!("Unhandled type"),
    }
}

fn get_option_type(ty: Type) -> IsOption {
    match ty {
        Type::Path(ty) => match ty.path.segments.iter().next() {
            Some(pp) => {
                let PathSegment {
                    ident,
                    ref arguments,
                } = pp;
                if format!("{}", &ident) == "Option" {
                    IsOption::True(match arguments {
                        PathArguments::AngleBracketed(args) => extract_option_type(args),
                        _ => panic!("Can't handle this type of option"),
                    })
                } else {
                    IsOption::False
                }
            }
            _ => IsOption::False,
        },
        _ => IsOption::False,
    }
}

pub fn create_typesafe_builder(structure: &Structure) -> proc_macro2::TokenStream {
    let struct_name = Ident::new(
        &crate::util::uppercase(&format!("{}", &structure.ident)),
        structure.ident.span(),
    );

    // struct Base
    let struct_decl = {
        let field_decs = structure.fields.iter().map(|f| {
            let name = &f.name;
            let ty = &f.ty;

            quote!(
                pub #name: #ty,
            )
        });
        quote!(
            #[doc(hidden)]
            pub struct #struct_name {
                #(#field_decs)*
            }
        )
    };

    let mut parts = vec![struct_decl];

    let unset = Ident::new("Unset", Span::call_site());

    let base_types = || -> Vec<TokenStream> {
        structure.fields
            .iter()
            .map(|v| &v.name)
            .map(|name| quote!(#name))
            .collect()
    };

    let struct_name = &structure.ident;
    let builder_name = Ident::new(&format!("{}Builder", &struct_name), Span::call_site());
    let value = Ident::new("value", Span::call_site());

    // struct Builder
    parts.push({
        let idents = structure.fields.iter().map(|v| &v.name);
        let field_types =  structure.fields.iter().map(|v| &v.name);
        let struct_types =  structure.fields.iter().map(|v| &v.name);

        quote!(
            #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            pub struct #builder_name<#(#struct_types),*>{
              #(#idents: #field_types,)*
            }
          )
    });

    // impl Plain Struct builder() -> Builder
    parts.push({
        let field_types =  structure.fields.iter().map(|v| {
            if v.has_default() {
                let ty = &v.ty;
                quote!(#ty)
            } else {
                quote!(#unset)
            }
        });

        quote!(
            impl #struct_name {
              #[allow(non_camel_case_types)]
              pub fn builder() -> #builder_name<#(#field_types),*> {
                #builder_name::new()
              }
            }
        )
    });

    // impl Builder new() -> Builder
    parts.push({
        let field_types = structure.fields.iter().map(|v| {
            if v.has_default() {
                let ty = &v.ty;
                quote!(#ty)
            } else {
                quote!(#unset)
            }
        });
        let unsets = structure.fields.iter().map(|_v| &unset);
        let field_decs = structure.fields.iter().map(|f| {
            let ident = &f.name;
            if let Some(ref default_value) = f.default_expr() {
                quote!(#ident : Some(#default_value),)
            } else {
                quote!(#ident : #unset,)
            }
        });
        quote!(
          impl #builder_name<#(#unsets),*>{
            #[allow(non_camel_case_types)]
            pub fn new() -> #builder_name<#(#field_types),*> {
              #builder_name {
                #(#field_decs)*
              }
            }
          }
        )
    });


    // impl Builder Setters

    let mut quoted_impls = structure.fields
        .iter()
        .enumerate()
        .map(|(idx, field)| {
            let mut impl_types = base_types();
            if !field.has_default() {
                impl_types.remove(idx);
            }
            let struct_types = structure.fields
                .iter()
                .enumerate()
                .map(|(inner_idx, inner_field)| {
                    let t: Type = if inner_idx != idx || field.has_default() {
                        syn::parse_str(&format!("{}", inner_field.name)).unwrap()
                    } else {
                        syn::parse_str("Unset").unwrap()
                    };
                    t
                }).collect::<Vec<_>>();

            let mut fn_types = base_types();
            fn_types[idx] = { let ty = &field.ty; quote!(#ty) };

            let fn_name = &field.name;

            let value_type = &field.ty;

            let field_names = structure.fields.iter().map(|v| &v.name);
            let assignments = structure.fields.iter().enumerate().map(|(i, v)|
                if i == idx {
                    quote!(#value)
                } else {
                    let n = &v.name;
                    quote!(self.#n)
                });

//                        println!("{} - {}", impl_types.len(), struct_types.len());

            let option_type = get_option_type(value_type.clone());

            match option_type {
                IsOption::False => {
                    quote!(
                            #[allow(non_camel_case_types)]
                            impl <#(#impl_types),*> #builder_name<#(#struct_types),*> {
                                pub fn #fn_name(self, value: #value_type) -> #builder_name<#(#fn_types),*> {
                                  #builder_name {
                                    #(#field_names : #assignments,)*
                                  }
                                }
                            }
                            )
                },
                IsOption::True(wrapped) => {
                    quote!(
                            #[allow(non_camel_case_types)]
                            impl <#(#impl_types),*> #builder_name<#(#struct_types),*> {
                                pub fn #fn_name<VALUE: crate::AsOption<#wrapped>>(self, value: VALUE) -> #builder_name<#(#fn_types),*> {
                                  let value : #value_type = value.as_option();
                                  #builder_name {
                                    #(#field_names : #assignments,)*
                                  }
                                }
                            }
                          )
                },
            }
        }).collect::<Vec<_>>();

    parts.append(&mut quoted_impls);


    // impl Builder Struct build() -> Plain
    parts.push({
        let field_names = structure.fields.iter().map(|v| &v.name);
        let field_names_2 = structure.fields.iter().map(|v| &v.name);
        let struct_types = structure.fields.iter().map(|v| &v.ty);

        quote!(
          #[allow(non_camel_case_types)]
          impl #builder_name<#(#struct_types),*>{
            pub fn build(self) -> #struct_name {
              #struct_name {
                #(#field_names : self.#field_names_2, )*
              }
            }
          }
        )
    });


    // impl Deconstruct<Args> for Builder

    quote!(#(#parts)*)
}
