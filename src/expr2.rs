use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, Expr, parse2};

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
        let (token, last_span) = get_token_stream(input);

        if token.is_empty() {
            let error = syn::Error::new(last_span, "Expected expression.");
            return Err(error)
        }

        let expr = parse2(token.to_owned())
            .map_err(|e| syn::Error::new_spanned(token.into_iter().last(), e))?;

        Ok(Self { expr })
    }
}

impl ToTokens for Expr2 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.expr.to_tokens(tokens)
    }
}

fn get_token_stream(input: &syn::parse::ParseBuffer) -> (TokenStream, proc_macro2::Span) {
    let (token, last_span) = input.step(|cursor| {
        let mut new_tokens = TokenStream::new();
        let mut last_span = input.span();

        let mut cursor = *cursor;
        while let Some((tt, next)) = cursor.token_tree() {
            match tt {
                proc_macro2::TokenTree::Punct(punct) if punct.as_char() == ',' => return Ok(((new_tokens, last_span), cursor)),
                _ => ()
            }

            tt.to_tokens(&mut new_tokens);

            cursor = next;
            if !cursor.eof() {
                last_span = cursor.span();
            }
        }

        return Ok(((new_tokens, last_span), cursor));
    }).unwrap();
    (token, last_span)
}
