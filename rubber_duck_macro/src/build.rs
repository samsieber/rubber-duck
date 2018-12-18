use crate::parse_fn;
use ::syn::parse_quote;
use proc_macro2::Ident;
use proc_macro2::Span;
use quote::quote;
use syn::ItemFn;
use crate::parse_fn::FieldRole;

pub fn gen_builder(structure: &parse_fn::Structure) -> ::proc_macro2::TokenStream {
    let struct_name = Ident::new(
        &crate::util::uppercase(&format!("{}", &structure.ident)),
        structure.ident.span(),
    );

    let field_decs = structure.fields.iter().map(|f| {
        let name = &f.name;
        let ty = &f.ty;
        let attr = if let FieldRole::Named(ref named_data) = f.extra {
            if let Some(ref _def) = named_data.default {
                quote!(#[default = "None"])
            } else {
                quote!()
            }
        } else {
            quote!()
        };

        quote!(
        #attr
        pub #name: #ty,
      )
    });

    let struct_decl = quote!(
        #[doc(hidden)]
        pub struct #struct_name {
            #(#field_decs)*
        }
    );

    crate::builder::create_typesafe_builder(structure)
}

pub fn gen_macro(structure: &parse_fn::Structure) -> ::proc_macro2::TokenStream {
    let field_names = structure.names();
    let name = &structure.ident;
    let struct_name = Ident::new(
        &crate::util::uppercase(&format!("{}", &structure.ident)),
        Span::call_site(),
    );

    //    println!("{}", quote!($test));

    let p_expr_matchers = structure.positional().enumerate().map(|(i, _p)| {
        let p_num = Ident::new(&format!("p{}", i), Span::call_site());
        quote!($#p_num:expr)
    });
    let p_expr_expanders = structure.positional().enumerate().map(|(i, p)| {
        let p_num = Ident::new(&format!("p{}", i), Span::call_site());
        let p_ident = &p.name;
        let res = quote!(.#p_ident($#p_num));
        eprintln!("{}", res);
        res
    });

    let doc_string = format!("Executes [{}](fn.{}.html) with name paramters as appropriate", &name, &name);

    let quoted = quote!(
        #[doc = #doc_string]
        pub macro #name(#(#p_expr_matchers,)* $($names:ident => $value:expr),*) {{
            let __temp = #struct_name::builder()
                    #(#p_expr_expanders)*
                    $(.$names($value))*
                    .build();
                #name(
                    #(__temp.#field_names,)*
                )
        }}
    );

    eprintln!("{}", &quoted);

    quoted
}

pub fn mod_block_add_defaults(fn_item: &mut ItemFn, structure: &parse_fn::Structure) {
    let statements = fn_item.block.stmts.clone();
    let has_defaults: Vec<_> = structure
        .named()
        .iter()
        .filter(|&f| f.extra.default.is_some())
        .map(|v| v.clone())
        .collect();
    let names1 = has_defaults.iter().map(|v| &v.name);
    let names2 = has_defaults.iter().map(|v| &v.name);
    let expr = has_defaults.iter().map(|v| v.extra.default.as_ref().unwrap());
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
}
