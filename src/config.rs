use proc_macro2::Ident;
use proc_macro_error::emit_error;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{self, Comma},
    LitStr, Token,
};

use crate::{derive::error_duplicate, value::PathOptionalValueAssignment};

macro_rules! unwrap_as {
    ($expr: expr, as $type: path, $error: expr) => {
        match $expr {
            $type(value) => value,
            _ => {
                emit_error!($expr, $error);
                return;
            }
        }
    };
}

pub struct ConfigDeclarationList {
    _pound: Token!(#),
    _bracket: token::Bracket,
    declarations: Punctuated<PathOptionalValueAssignment, Comma>,
}

impl ConfigDeclarationList {
    pub fn parse_all(input: ParseStream) -> syn::Result<Vec<Self>> {
        let mut all_declarations = Vec::new();
        while input.peek(Token!(#)) {
            all_declarations.push(input.call(ConfigDeclarationList::parse)?)
        }

        Ok(all_declarations)
    }
}

impl Parse for ConfigDeclarationList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _pound: input.parse()?,
            _bracket: bracketed!(content in input),
            declarations: content.parse_terminated(PathOptionalValueAssignment::parse)?,
        })
    }
}

#[derive(Default)]
pub struct Config {
    comment: String,
    function_name: Option<LitStr>,
}

impl Config {
    pub fn new(attributes: Vec<ConfigDeclarationList>) -> Self {
        let mut self_ = Self::default();

        for list in attributes {
            for config in list.declarations {
                let path = config
                    .ident()
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>();

                let path = path.iter().map(|s| s.as_str()).collect::<Vec<_>>();

                match path.as_slice() {
                    &["doc"] => self_.parse_documentation(config),
                    &["function"] => self_.parse_function(config, path),

                    _ => emit_error!(config.ident(), "Unknown config."),
                }
            }
        }

        self_
    }

    fn parse_documentation(&mut self, attr: PathOptionalValueAssignment) {
        if attr.value().is_none() {
            emit_error!(attr.ident(), "Expected `doc = ...`");
            return;
        }

        let lit =
            unwrap_as!(attr.value().unwrap(), as syn::Expr::Lit, "Expected a literal expression.");
        let str = unwrap_as!(&lit.lit, as syn::Lit::Str, "Expected a string literal.");

        if !self.comment.is_empty() {
            self.comment.push('\n');
        }
        self.comment += str.value().as_str();
    }

    fn parse_function(&mut self, attr: PathOptionalValueAssignment, path_str: Vec<&str>) {
        if attr.value().is_none() {
            emit_error!(attr.ident(), "Expected `function = ...`");
            return;
        }

        let lit =
            unwrap_as!(attr.value().unwrap(), as syn::Expr::Lit, "Expected a literal expression.");
        let str = unwrap_as!(&lit.lit, as syn::Lit::Str, "Expected a string literal.");

        match &self.function_name {
            Some(str2) => {
                error_duplicate!(
                    attr, "This config is already set." ;
                    str2, "Value for config `{}` is already set here.", path_str.join("::")
                );
            }
            None => {
                if str.value().is_empty() {
                    emit_error!(str, "The function name must not be empty.");
                    return;
                }

                self.function_name = Some(str.to_owned())
            },
        }
    }

    pub fn comment(&self) -> &str {
        self.comment.as_ref()
    }

    pub fn function_name(&self) -> Option<Ident> {
        self.function_name.as_ref().map(|l| Ident::new(&l.value(), l.span()))
    }

    pub fn function_name_lit(&self) -> Option<&LitStr> {
        self.function_name.as_ref()
    }
}
