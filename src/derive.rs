use std::collections::HashMap;

use proc_macro2::Ident;
use proc_macro_error::{abort, abort_if_dirty, emit_error};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parenthesized,
    parse::Parse,
    punctuated::Punctuated,
    token::{self, Comma},
    DataEnum, DeriveInput, Expr, Token, Type, Variant, Visibility, LitStr,
};

use crate::{
    config::{Config, ConfigDeclarationList},
    opt::{extract_type_from_option, is_option_wrapped},
    value::{IdentValueAssignment, ValueAssignment},
};

macro_rules! error_duplicate {
    ($span1: expr, $error1: expr $(, $error1fragments: expr)*;
     $span2: expr, $error2: expr $(, $error2fragments: expr)*) => {
        cfg_if::cfg_if! {
            if #[cfg(help_span)] {
                emit_error!(
                    $span1, $error1 $(, $error1fragments)*;
                    help = $span2 => $error2 $(, $error2fragments)*
                );
            } else {
                emit_error!($span1, $error1 $(, $error1fragments)*);
                emit_error!($span2, $error2 $(, $error2fragments)*);
            }
        }
    };
}

pub(crate) use error_duplicate;

macro_rules! unwrap_opt_or_continue {
    ($expr: expr) => {{
        let res = $expr;
        if res.is_none() {
            continue;
        }
        res.unwrap()
    }};
}

struct ParenList<T> {
    _paren: token::Paren,
    elements: Punctuated<T, Comma>,
}

impl<T: Parse> Parse for ParenList<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            _paren: parenthesized!(content in input),
            elements: content.parse_terminated(T::parse)?,
        })
    }
}

struct AttributeDeclaration {
    attributes: Vec<ConfigDeclarationList>,
    vis: Visibility,
    ident: Ident,
    _colon: Token!(:),
    type_: Type,
    default_value: Option<ValueAssignment>,
}

impl Parse for AttributeDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attributes: input.call(ConfigDeclarationList::parse_all)?,
            vis: input.parse()?,
            ident: input.parse()?,
            _colon: input.parse()?,
            type_: {
                let res = input.parse();
                if let Err(e) = res {
                    let err = syn::Error::new(e.span(), "Expected a type.");
                    return Err(err);
                }
                res.unwrap()
            },
            default_value: input.parse()?,
        })
    }
}

struct AttributeValue<'f> {
    variant_ident: &'f Ident,
    value: Option<Expr>,
    type_state: TypeState,
}

impl<'f> AttributeValue<'f> {
    pub fn new(field_ident: &'f Ident, type_state: TypeState) -> Self {
        Self {
            variant_ident: field_ident,
            value: None,
            type_state,
        }
    }
}

impl<'f> ToTokens for AttributeValue<'f> {
    fn to_tokens(&self, tokens2: &mut proc_macro2::TokenStream) {
        if self.value.is_none() {
            return proc_macro2::TokenStream::new().to_tokens(tokens2);
        }

        let ident = &self.variant_ident;

        let value = self.value.as_ref().unwrap();

        let value = match self.type_state {
            TypeState::Required(_) => quote!(#value),
            TypeState::Optional(_) => {
                if is_option_wrapped(value) {
                    quote!(#value)
                } else {
                    quote!(Some(#value))
                }
            }
        };

        let tokens = quote! {
            if let Self::#ident = self {
                return #value
            }
        };

        tokens.to_tokens(tokens2)
    }
}

#[derive(Clone)]
enum TypeState {
    Required(Type),
    Optional(Type),
}

struct Attribute<'f> {
    vis: Visibility,
    ident: Ident,
    type_: TypeState,
    values: Vec<AttributeValue<'f>>,
    default: Option<Expr>,
    config: Config,
}

impl<'f> Attribute<'f> {
    fn new(declaration: AttributeDeclaration, variants: &'f Punctuated<Variant, Comma>) -> Self {
        let type_ = declaration.type_;
        let type_state = match extract_type_from_option(&type_) {
            Some(type_) => TypeState::Optional(type_.to_owned()),
            None => TypeState::Required(type_),
        };

        let values = variants
            .iter()
            .map(|f| AttributeValue::new(&f.ident, type_state.to_owned()))
            .collect();

        let config = Config::new(declaration.attributes);

        Self {
            vis: declaration.vis,
            ident: declaration.ident,
            type_: type_state,
            values,
            default: declaration
                .default_value
                .map(|default| default.value().to_owned()),
            config,
        }
    }

    fn set(&mut self, ident: &Ident, value: IdentValueAssignment) {
        let attr_value = self
            .values
            .iter_mut()
            .find(|p| p.variant_ident == ident)
            .expect("tried to set a value for a variant that doesn't exists.");

        match attr_value.value {
            None => attr_value.value = Some(value.value().to_owned()),
            Some(ref value2) => {
                error_duplicate!(
                    value, "The value is already set for this attribute.";
                    value2, "First value of `{}` is set here.", self.ident
                );
            }
        }
    }

    fn check(&self) {
        if self.default.is_some() {
            return;
        }
        if let TypeState::Optional(_) = self.type_ {
            return;
        }

        for value in &self.values {
            if value.value.is_none() {
                emit_error!(
                    value.variant_ident,
                    format!("Value not set for `{}`.", self.ident)
                );
            }
        }
    }
}

impl<'f> ToTokens for Attribute<'f> {
    fn to_tokens(&self, tokens2: &mut proc_macro2::TokenStream) {
        let function_name = self.config.function_name().unwrap_or(format_ident!("get_{}", self.ident));

        let vis = &self.vis;
        let type_ = match &self.type_ {
            TypeState::Required(type_) => quote!(#type_),
            TypeState::Optional(type_) => quote!(Option<#type_>),
        };
        let values = &self.values;

        let default = match &self.default {
            Some(value) => {
                let mut tokens = quote!(#value);
                if let TypeState::Optional(_) = self.type_ {
                    if !is_option_wrapped(value) {
                        tokens = quote!(Some(#value))
                    }
                }
                tokens
            }
            None => {
                if let TypeState::Optional(_) = self.type_ {
                    quote!(None)
                } else {
                    quote!(unreachable!())
                }
            }
        };

        let comment = self.config.comment();

        let tokens = quote! {
            #[doc = #comment]
            #vis fn #function_name(&self) -> #type_ {
                #(#values)*

                #default
            }
        };

        tokens.to_tokens(tokens2)
    }
}

fn parse_enum_attributes<'f>(
    attrs: &[syn::Attribute],
    data_enum: &'f DataEnum,
) -> Vec<Attribute<'f>> {
    let mut attribute_declarations = Vec::<AttributeDeclaration>::new();

    for attr in attrs.iter() {
        let attr_ident = unwrap_opt_or_continue!(attr.path.get_ident());

        match attr_ident.to_string().as_str() {
            "attr" => {
                let res = syn::parse2(attr.tokens.to_owned());
                if let Err(e) = res {
                    emit_error!(e.span(), e);
                    continue;
                }

                let declaration_list: ParenList<AttributeDeclaration> = res.unwrap();

                for declaration in declaration_list.elements {
                    let match_ = attribute_declarations
                        .iter()
                        .find(|attr2| declaration.ident == attr2.ident);

                    if let Some(declaration2) = match_ {
                        error_duplicate!(
                            declaration.ident, "This attribute is already declared.";
                            declaration2.ident, "`{}` is already declared here.", declaration2.ident
                        );

                        continue;
                    }

                    attribute_declarations.push(declaration);
                }
            }

            _ => continue,
        }
    }

    attribute_declarations
        .into_iter()
        .map(|decl| Attribute::new(decl, &data_enum.variants))
        .collect()
}

fn parse_variant_attributes(variant: &Variant) -> Vec<IdentValueAssignment> {
    let mut variant_attrs = Vec::new();

    for attr in &variant.attrs {
        let attr_ident = unwrap_opt_or_continue!(attr.path.get_ident());

        match attr_ident.to_string().as_str() {
            "attr" => {
                let res = syn::parse2(attr.tokens.to_owned());
                if let Err(e) = res {
                    emit_error!(e.span(), e);
                    continue;
                }

                let list: ParenList<IdentValueAssignment> = res.unwrap();

                variant_attrs.extend(list.elements.into_iter());
            }
            _ => continue,
        }
    }

    variant_attrs
}

fn check_for_conflicts(attrs: &[Attribute]) {
    let mut before = HashMap::<&LitStr, &Attribute>::new();
    for attr in attrs.iter().filter(|a| a.config.function_name_lit().is_some()) {
        let lit = attr.config.function_name_lit().unwrap();
        
        if let Some((lit2, attr2)) = before.get_key_value(lit) {
            error_duplicate!(
                lit, "The attribute `{}` already use this function name.", attr2.ident;
                lit2, "First use of `{}` here.", lit.value()
            );

            continue;
        }

        before.insert(lit, attr);
    }
}

pub fn derive_custom_attrs(input: DeriveInput) -> proc_macro2::TokenStream {
    let data_enum = match input.data {
        syn::Data::Struct(struct_) => abort!(struct_.struct_token, "Not implemented for structs."),
        syn::Data::Union(union_) => abort!(union_.union_token, "Not implemented for unions."),

        syn::Data::Enum(ref data_enum) => data_enum,
    };

    let mut attributes = parse_enum_attributes(&input.attrs, data_enum);

    abort_if_dirty();

    for variant in &data_enum.variants {
        let variant_attrs = parse_variant_attributes(variant);

        for attr in variant_attrs {
            let opt = attributes
                .iter_mut()
                .find(|attr2| &attr2.ident == attr.ident());

            if opt.is_none() {
                emit_error!(attr.ident(), "Unknown attribute.");
                continue;
            }

            opt.unwrap().set(&variant.ident, attr)
        }
    }

    for attr in attributes.iter() {
        attr.check();
    }

    abort_if_dirty();

    check_for_conflicts(&attributes);

    abort_if_dirty();

    let ident = &input.ident;
    let (impl_generics, generics, generic_where) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #ident #generics #generic_where {
            #(#attributes)*
        }
    }
}
