use quote::ToTokens;
use syn::{parse::Parse, token::CustomToken, Expr, Ident, Path, Token};

pub type IdentValueAssignment = IdentValueAssignmentGeneric<Ident>;
pub type PathOptionalValueAssignment = IdentOptionalValueAssignmentGeneric<Path>;

pub struct IdentValueAssignmentGeneric<I> {
    ident: I,
    value: ValueAssignment,
}

impl<I> IdentValueAssignmentGeneric<I> {
    pub fn value(&self) -> &Expr {
        &self.value.value
    }

    pub fn ident(&self) -> &I {
        &self.ident
    }
}

impl<I: Parse> Parse for IdentValueAssignmentGeneric<I> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl<I: ToTokens> ToTokens for IdentValueAssignmentGeneric<I> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.ident.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

pub struct IdentOptionalValueAssignmentGeneric<I> {
    ident: I,
    value: Option<ValueAssignment>,
}

impl<I> IdentOptionalValueAssignmentGeneric<I> {
    pub fn value(&self) -> Option<&Expr> {
        self.value.as_ref().map(|v| &v.value)
    }

    pub fn ident(&self) -> &I {
        &self.ident
    }
}

impl<I: Parse> Parse for IdentOptionalValueAssignmentGeneric<I> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl<I: ToTokens> ToTokens for IdentOptionalValueAssignmentGeneric<I> {
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
