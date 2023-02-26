use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, Expr};

pub struct Expr2 {
    expr: Expr,
}

impl Expr2 {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Parse for Expr2 {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let opt = input.step(|cursor| {
            let mut new_tokens = TokenStream::new();

            let mut cursor = *cursor;
            while let Some((tt, next)) = cursor.token_tree() {
                match tt {
                    proc_macro2::TokenTree::Punct(punct) if punct.as_char() == ',' => return Ok((new_tokens, cursor)),
                    _ => ()
                }

                tt.to_tokens(&mut new_tokens);

                cursor = next
            }

            return Ok((new_tokens, cursor));
        }).unwrap();

        let expr = syn::parse2(opt)?;

        Ok(Self { expr })
    }
}

impl ToTokens for Expr2 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.expr.to_tokens(tokens)
    }
}
