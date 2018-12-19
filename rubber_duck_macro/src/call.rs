use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr, Ident, Token, Type, ExprMethodCall};
use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::token::{Comma, Brace};
use syn::{braced, bracketed, parenthesized, parse_quote};
use syn::parse::ParseBuffer;
use syn::TypePath;
use syn::ExprCall;


struct NamedArgCall {
    ty: TypePath,
    pos_fields: Vec<Expr>,
    named_fields: Vec<NamedField>,
}

struct NamedField {
    pub ident: Ident,
    pub expr: Expr,
}

struct ExpressionList {
    pub items: Vec<Expr>,
    pub has_comma: bool,
}

impl Parse for NamedField {
    fn parse(input: ParseStream) -> Result<Self> {
        eprintln!("Parsing named field");
        let ident = input.parse()?;
        if input.peek(Token![:]) {
            eprintln!("Found a colon");
            input.parse::<Token![:]>()?;
            Ok(NamedField {
                ident,
                expr: input.parse()?,
            })
        } else {
            eprintln!("No colon found");
            let expr: Expr = {
                let id = &ident;
                parse_quote!(#id)
            };
            Ok( NamedField { ident, expr, })
        }
    }
}

enum ParsingPositional {
    Positional(Expr),
    NamedArgs(Punctuated<NamedField, Comma>),
    Empty,
}

impl Parse for ParsingPositional {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Ok(ParsingPositional::Empty)
        } else if input.peek(syn::token::Brace){
            let braced_content : ParseBuffer;
            let brace : Brace = braced!(braced_content in input);
            eprintln!(
                "Ident:{}\nColon:{}\nComma:{}",
                braced_content.peek(Ident),
                braced_content.peek2(Token![:]),
                braced_content.peek2(Token![,]),
                );
            if braced_content.peek(Ident) && (braced_content.peek2(Token![:]) || braced_content.peek2(Token![,])) {
                Ok(ParsingPositional::NamedArgs(
                    braced_content.parse_terminated(NamedField::parse)?
                ))
            } else {
                Ok(ParsingPositional::Positional({
                    let mut braced_tokens = braced_content.cursor().token_stream();
                    brace.surround(&mut braced_tokens, |v|{});
                    syn::parse_macro_input::parse(braced_tokens.into())?
                }))
            }
        } else {
            Ok(ParsingPositional::Positional(
                input.parse()?
            ))
        }
    }
}

impl Parse for NamedArgCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let arg_content : ParseBuffer;
        let ty = input.parse()?;
        let mut pos_fields = vec!();
        let mut punctuated_names : Option<Punctuated<NamedField, Comma>> = None;
        braced!(arg_content in input);
        eprintln!("Parsed type: {:?}", &ty);
        loop {
            let next = arg_content.parse::<ParsingPositional>()?;
            match next {
                ParsingPositional::Positional(expr) => {
                    pos_fields.push(expr);
                    if arg_content.peek(Token![,]) {
                        arg_content.parse::<Token![,]>()?;
                    }
                },
                ParsingPositional::NamedArgs(args) => {
                    punctuated_names = Some(args);
                    break;
                },
                ParsingPositional::Empty => {
                    break;
                }
            };
        }
        eprintln!("Done parsing");

        let named_fields =
            punctuated_names
                .map(|pn|
                    pn
                    .into_pairs()
                    .map(|v| v.into_value())
                    .collect()
                )
                .unwrap_or_else(|| Vec::new());

        Ok(NamedArgCall { ty, pos_fields, named_fields, })
    }
}

pub fn n(input: TokenStream) -> TokenStream {
    let named_arg_call : NamedArgCall = parse_macro_input!(input as NamedArgCall);
    eprintln!("Done parsing stream");
    let call = &named_arg_call.ty;
    let pos_args = named_arg_call.pos_fields.iter().map(|expr| {
        quote!(.next(#expr))
    });
    let name_args = named_arg_call.named_fields.iter().map(|named| {
        let name = &named.ident;
        let expr = &named.expr;
        quote!(.#name(#expr))
    });
    quote!({
        use $crate::{Deconstruct, Call};
        let mut built = #call::builder()
            #(#pos_args)*
            #(#name_args)*;
        let deco = built.deconstruct();
        #call.apply(deco)
        }
    ).into()
}