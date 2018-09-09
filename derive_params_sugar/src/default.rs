use syn::{Expr, Lit, Meta, MetaNameValue};

pub fn as_expr(lit: Lit) -> Expr {
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

pub fn extract_default(m: Meta) -> Option<Expr> {
    match m {
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
    }
}
