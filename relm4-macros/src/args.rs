use proc_macro2::TokenStream as TokenStream2;
use quote::{quote_spanned, ToTokens};
use syn::{spanned::Spanned, punctuated::Punctuated, Expr, Token, parse::{Parse, ParseStream}, Result};

#[derive(Debug)]
pub struct Args {
    exprs: Vec<Expr>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let punct: Punctuated<Expr, Token![,]> = input.call(Punctuated::parse_separated_nonempty)?;
        let exprs = punct.into_pairs().map(|pair| pair.into_value()).collect();

        Ok(Args {
            exprs
        })
    }
}

impl ToTokens for Args {
    fn to_tokens(&self, out: &mut TokenStream2) {
        let mut iter = self.exprs.iter();

        let first = iter.next().unwrap();
        out.extend(quote_spanned! {
                first.span() => #first
            });

        for expr in iter {
            out.extend(quote_spanned! {
                expr.span() => ,#expr
            });
        }
    }
}
