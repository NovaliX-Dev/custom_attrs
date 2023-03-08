use proc_macro2::{Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    token,
};

fn get_tokens(input: &ParseStream) -> (TokenStream, bool) {
    input
        .step(|cursor| {
            let mut tokens = TokenStream::new();
            let mut add_comma = true;

            let mut cursor = *cursor;
            while let Some((tt, next)) = cursor.token_tree() {
                if let TokenTree::Punct(punct) = &tt {
                    if punct.as_char() == ',' {
                        add_comma = false;
                        break;
                    }
                }

                tt.to_tokens(&mut tokens);

                cursor = next;
            }

            Ok(((tokens, add_comma), cursor))
        })
        .unwrap()
}

pub struct ReferenceTokens {
    tokens: TokenStream,
    last_span: Option<Span>,
}

impl Parse for ReferenceTokens {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mut tokens, add_comma) = get_tokens(&input);

        // trick to have the correct span for the expression error
        if tokens.is_empty() {
            let comma = token::Comma {
                spans: [input.span()],
            };
            comma.to_tokens(&mut tokens);
        }

        let last_span = if add_comma { Some(input.span()) } else { None };

        Ok(Self { tokens, last_span })
    }
}

impl ToTokens for ReferenceTokens {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.tokens.to_tokens(tokens);

        if let Some(span) = self.last_span {
            // trick to have the correct span for the expression error
            let comma = token::Comma { spans: [span] };
            comma.to_tokens(tokens);
        }
    }
}
