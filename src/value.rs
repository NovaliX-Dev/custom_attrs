use quote::ToTokens;
use syn::{parse::Parse, token::CustomToken, Expr, Ident, Token};

pub type AttributeValueAssignment = AttributeValueAssignmentGeneric<Ident>;

pub struct AttributeValueAssignmentGeneric<I> {
    ident: I,
    value: ValueAssignment,
}

impl<I> AttributeValueAssignmentGeneric<I> {
    pub fn value(&self) -> &Expr {
        &self.value.value
    }

    pub fn ident(&self) -> &I {
        &self.ident
    }
}

impl<I: Parse> Parse for AttributeValueAssignmentGeneric<I> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl<I: ToTokens> ToTokens for AttributeValueAssignmentGeneric<I> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.ident.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

pub struct ValueAssignment {
    _equal: Token!(=),
    value: Expr,
}

impl ValueAssignment {
    pub fn value(&self) -> &Expr {
        &self.value
    }
}

impl Parse for ValueAssignment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _equal: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl CustomToken for ValueAssignment {
    fn peek(cursor: syn::buffer::Cursor) -> bool {
        if let Some((punct, _)) = cursor.punct() {
            if punct.as_char() == '=' {
                return true;
            }
        }
        false
    }

    fn display() -> &'static str {
        todo!()
    }
}

impl ToTokens for ValueAssignment {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self._equal.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}
