#![feature(proc_macro_gen)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
use proc_macro::TokenStream;
use syn::export::Span;
use syn::export::ToTokens;
use syn::{parse_macro_input, Expr, Field, Fields, Ident, Item, Lit, Meta, MetaNameValue, Type, GenericArgument, PathArguments};
use syn::PathSegment;
use syn::AngleBracketedGenericArguments;

mod util;

enum IsOption {
  True(Type),
  False,
}

fn extractOptionType(wrapped_type: &AngleBracketedGenericArguments) -> Type{
  let value = wrapped_type.args.iter().next().unwrap();
  match value {
    GenericArgument::Type(ty) => ty.clone(),
    _ => panic!("Unhandled type"),
  }
}

fn getOptionType(ty: Type) -> IsOption{
  match ty {
    Type::Path(ty) => {
      match ty.path.segments.iter().next() {
        Some(pp) => {
          let PathSegment {
             ident, ref arguments
          } = pp;
          if format!("{}", &ident) == "Option" {
            IsOption::True(match arguments {
              PathArguments::AngleBracketed(args) => {
                extractOptionType(args)
              },
              _ => panic!("Can't handle this type of option"),
            })
          } else {
            IsOption::False
          }
        },
        _ => IsOption::False,
      }
    },
    _ => IsOption::False,
  }
}

#[proc_macro_attribute]
pub fn gen_typesafe_builder(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let generated_parts = {
        let mut parsed: Item = parse_macro_input!(input as Item);
        match parsed {
            Item::Struct(mut struct_item) => {
                let parsed_fields = parse_fields(&mut struct_item.fields);
                let mut parts = vec![quote!(pub struct Unset;), quote!(#struct_item)];

                let unset = Ident::new("Unset", Span::call_site());

                let base_types = || -> Vec<Box<dyn ToTokens>> {
                    parsed_fields
                        .iter()
                        .map(|v| Box::new(v.name.clone()) as Box<ToTokens>)
                        .collect()
                };

                let struct_name = &struct_item.ident;
                let builder_name =
                    Ident::new(&format!("{}Builder", &struct_name), Span::call_site());

                let value = Ident::new("value", Span::call_site());

                parts.push({
                    let idents = parsed_fields.iter().map(|v| &v.name);
                    let field_types = parsed_fields.iter().map(|v| &v.name);
                    let struct_types = parsed_fields.iter().map(|v| &v.name);

                    quote!(
                    pub struct #builder_name<#(#struct_types),*>{
                      #(#idents: #field_types,)*
                    }
                  )
                });

                parts.push ({
                  let field_types = parsed_fields.iter().map(|v| {
                    if v.default_value.is_some() {
                      let ty = &v.field.ty;
                      quote!(#ty)
                    } else {
                      quote!(#unset)
                    }
                  });

                  quote!(
                    impl #struct_name {
                      pub fn builder() -> #builder_name<#(#field_types),*> {
                        #builder_name::new()
                      }
                    }
                  )
                });

                parts.push({
                    let field_types = parsed_fields.iter().map(|v| {
                        if v.default_value.is_some() {
                            let ty = &v.field.ty;
                            quote!(#ty)
                        } else {
                            quote!(#unset)
                        }
                    });
                    let unsets = parsed_fields.iter().map(|v| &unset);
                    let field_decs = parsed_fields.iter().map(|f| {
                        let ident = &f.name;
                        let typ = &f.field.ty;
                        if let Some(ref default_value) = f.default_value {
                            quote!(#ident : #default_value,)
                        } else {
                            quote!(#ident : #unset,)
                        }
                    });
                    quote!(
                      impl #builder_name<#(#unsets),*>{
                        pub fn new() -> #builder_name<#(#field_types),*> {
                          #builder_name {
                            #(#field_decs)*
                          }
                        }
                      }
                    )
                });

                parts.push({
                    let idents = parsed_fields.iter().map(|v| &v.name);
                    let field_names = parsed_fields.iter().map(|v| &v.name);
                    let field_names_2 = parsed_fields.iter().map(|v| &v.name);
                    let struct_types = parsed_fields.iter().map(|v| &v.field.ty);

                    quote!(
                      impl #builder_name<#(#struct_types),*>{
                        pub fn build(self) -> #struct_name {
                          #struct_name {
                            #(#field_names : self.#field_names_2, )*
                          }
                        }
                      }
                    )
                });

                let mut quoted_impls = parsed_fields
                    .iter()
                    .enumerate()
                    .map(|(idx, field)| {
                        let mut impl_types = base_types();
                        if let Some(ref df) = field.default_value {
                        } else {
                            impl_types.remove(idx);
                        }
                        let struct_types = parsed_fields
                            .iter()
                            .enumerate()
                            .map(|(inner_idx, inner_field)| {
                                let t: Type = if inner_idx != idx || field.default_value.is_some() {
                                    syn::parse_str(&format!("{}", inner_field.name)).unwrap()
                                } else {
                                    syn::parse_str("Unset").unwrap()
                                };
                                t
                            }).collect::<Vec<_>>();

                        let mut fn_types = base_types();
                        fn_types[idx] = Box::new(field.field.ty.clone());

                        let fn_name = &field.name;

                        let value_type = &field.field.ty;

                        let field_names = parsed_fields.iter().map(|v| &v.name);
                        let assignments = parsed_fields.iter().enumerate().map(|(i, v)|
                            if i == idx {
                              quote!(#value)
                            } else {
                              let n = &v.name;
                              quote!(self.#n)
                            });

                        println!("{} - {}", impl_types.len(), struct_types.len());

                        let optionType = getOptionType(value_type.clone());

                      match optionType {
                        IsOption::False => {
                          quote!(
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
                            impl <#(#impl_types),*> #builder_name<#(#struct_types),*> {
                                pub fn #fn_name<VALUE: AsOption<#wrapped>>(self, value: VALUE) -> #builder_name<#(#fn_types),*> {
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
                //        let fn_types = parsed_fields.iter().enumerate().map(|(i,v)| {
                //          let mut ret = base_types();
                //          ret[i] = Box::new(v.field.ty.clone());
                //          ret
                //        });
                //        let value_types = parsed_fields.iter().enumerate().map(|(i,v)| {
                //          &v.field.ty
                //        });
                //        let lefts_once = || parsed_fields.iter().map(|v| v.name.clone());
                //        let lefts = parsed_fields.iter().map(|v| lefts_once());
                //        let fn_names = parsed_fields.iter().map(|v| &v.name);

                //        let rights = parsed_fields.iter().enumerate().map(|(i, _)| {
                //          let right_loop = parsed_fields.iter().enumerate().map(|(i_i, f)| {
                //            let ret: Expr = if i_i == i {
                //              let ident = Ident::new("value", Span::call_site());
                //              let quoted: proc_macro::TokenStream = quote!(#ident).into();
                //              let expr = parse_macro_input!(quoted as Expr);
                //              expr
                //            } else {
                //              let ident = f.name;
                //              let quoted: proc_macro::TokenStream = quote!(self.#ident).into();
                //              let expr = parse_macro_input!(quoted as Expr);
                //              expr
                //            };
                //            ret
                //          }).collect::<Vec<Expr>>();
                //          right_loop
                //        }).collect::<Vec<Vec<Expr>>>();

                parts.append(&mut quoted_impls);

                parts
                // 2) Build a struct for said arguments
                // 3) .. now what?
            }
            _ => {
                panic!("Arg");
            }
        }
    };

    let generated = quote!(#(#generated_parts)*);

    println!("{}", &generated);

    generated.into()
}

#[derive(Debug, Clone)]
struct MyField {
    field: Field,
    name: Ident,
    default_value: Option<Expr>,
}

fn as_expr(lit: Lit) -> Expr {
    if let Lit::Str(lit_str) = lit {
        lit_str.parse::<Expr>().unwrap()
    } else {
        panic!("Not a string literal!");
    }
}

fn as_wrapped_expr(lit: Lit) -> Expr {
    if let Lit::Str(lit_str) = lit {
        lit_str.parse::<Expr>().unwrap()
    } else {
        panic!("Not a string literal!");
    }
}

fn parse_fields(fields: &mut Fields) -> Vec<MyField> {
    let ret = fields
        .iter_mut()
        .map(|f_ref| {
            let mut plucked_default_values = ::util::drain_map(&mut f_ref.attrs, |a| {
                println!("{:?}", a.interpret_meta());
                a.interpret_meta().and_then(|m| match m {
                    Meta::Word(_) => None,
                    Meta::List(_) => None,
                    Meta::NameValue(MetaNameValue { ident, lit, .. }) => {
                        if ident.to_string() == "default" {
                            Some(as_expr(lit))
                        } else if ident.to_string() == "default_str" {
                            Some(as_wrapped_expr(lit))
                        } else {
                            None
                        }
                    }
                })
            });
            if plucked_default_values.len() > 1 {
                panic!("Too many default values!")
            }
            MyField {
                field: f_ref.clone(),
                name: f_ref.ident.clone().unwrap(),
                default_value: plucked_default_values.pop(),
            }
        }).collect::<Vec<_>>();
    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
