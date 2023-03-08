use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, token::CustomToken, Expr, Ident, Path, Token};

use crate::reference::ReferenceTokens;

pub type AttributeValueAssignment = IdentValueAssignmentGeneric<Ident, Expr>;
pub type AttributeValueAssignmentTokens = IdentValueAssignmentGeneric<Ident, ReferenceTokens>;
pub type AttributeValueAssignmentTokenStream = IdentValueAssignmentGeneric<Ident, TokenStream>;
pub type ConfigValueAssignment = IdentOptionalValueAssignmentGeneric<Path, Expr>;

pub struct IdentValueAssignmentGeneric<I, V> {
    ident: I,
    value: ValueAssignmentGeneric<V>,
}

impl<I, V> IdentValueAssignmentGeneric<I, V> {
    pub fn into_value(self) -> V {
        self.value.value
    }

    pub fn from_parts(ident: I, equal: Token!(=), value: V) -> Self {
        Self {
            ident,
            value: ValueAssignmentGeneric::from_parts(equal, value),
        }
    }

    pub fn into_parts(self) -> (I, Token!(=), V) {
        (self.ident, self.value._equal, self.value.value)
    }

    pub fn ident(&self) -> &I {
        &self.ident
    }
}

impl<I: Parse, V: Parse> Parse for IdentValueAssignmentGeneric<I, V> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl<I: ToTokens, V: ToTokens> ToTokens for IdentValueAssignmentGeneric<I, V> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.ident.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

pub struct IdentOptionalValueAssignmentGeneric<I, V> {
    ident: I,
    value: Option<ValueAssignmentGeneric<V>>,
}

impl<I, V> IdentOptionalValueAssignmentGeneric<I, V> {
    pub fn value(&self) -> Option<&V> {
        self.value.as_ref().map(|v| &v.value)
    }

    pub fn ident(&self) -> &I {
        &self.ident
    }
}

impl<I: Parse, V: Parse> Parse for IdentOptionalValueAssignmentGeneric<I, V> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl<I: ToTokens, V: ToTokens> ToTokens for IdentOptionalValueAssignmentGeneric<I, V> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.ident.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

pub type ValueAssignment = ValueAssignmentGeneric<Expr>;

pub struct ValueAssignmentGeneric<V> {
    _equal: Token!(=),
    value: V,
}

impl<V> ValueAssignmentGeneric<V> {
    pub fn from_parts(equal: Token!(=), value: V) -> Self {
        Self {
            _equal: equal,
            value,
        }
    }

    pub fn into_value(self) -> V {
        self.value
    }
}

impl<V: Parse> Parse for ValueAssignmentGeneric<V> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _equal: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl<V> CustomToken for ValueAssignmentGeneric<V> {
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

impl<V: ToTokens> ToTokens for ValueAssignmentGeneric<V> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self._equal.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}
