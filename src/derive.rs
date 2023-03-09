use std::collections::HashMap;

use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_if_dirty, emit_error, SpanRange};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parenthesized,
    parse::Parse,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{self, Comma},
    DeriveInput, Expr, Ident, LitStr, Token, Type, Variant, Visibility,
};

use crate::{
    config::{Config, ConfigDeclarationList},
    opt::{extract_type_from_option, is_option_wrapped},
    reference::{Reference, ReferenceProcessor},
    value::{
        AttributeValueAssignment, AttributeValueAssignmentTokenStream,
        AttributeValueAssignmentTokens, ValueAssignment,
    },
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

macro_rules! unwrap_or_continue {
    ($expr: expr) => {{
        match $expr {
            Ok(value) => value,
            Err(e) => {
                emit_error!(e.span(), e);
                continue;
            }
        }
    }};

    (no_emit $expr: expr) => {{
        match $expr {
            Ok(value) => value,
            Err(_) => {
                continue;
            }
        }
    }};
}

macro_rules! unwrap_ {
    ($expr: expr) => {{
        match $expr {
            Ok(value) => value,
            Err(e) => return Err(e),
        }
    }};
    ($expr: expr, $error: expr) => {{
        match $expr {
            Ok(value) => value,
            Err(e) => {
                let error = syn::Error::new(e.span(), $error);
                return Err(error);
            }
        }
    }};
}

struct ParenList<T> {
    _paren: token::Paren,
    elements: Punctuated<T, Comma>,
}

impl<T> ParenList<T> {
    fn into_parts(self) -> (token::Paren, Punctuated<T, Comma>) {
        (self._paren, self.elements)
    }

    fn from_parts(paren: token::Paren, elements: Vec<T>) -> Self {
        Self {
            _paren: paren,
            elements: Punctuated::from_iter(elements.into_iter()),
        }
    }
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

impl<T: ToTokens> ToTokens for ParenList<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self._paren
            .surround(tokens, |tokens2| self.elements.to_tokens(tokens2));
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
            type_: unwrap_!(input.parse(), "Expected a type."),
            default_value: input.parse()?,
        })
    }
}

struct AttributeValue {
    variant: Ident,
    value: Expr,
    self_references: Option<AttributeMetadata>,
    required: bool,
}

impl AttributeValue {
    fn new(
        variant: Ident,
        required: bool,
        value: Expr,
        metadata: Option<AttributeMetadata>,
    ) -> Self {
        Self {
            variant,
            value,
            self_references: metadata,
            required,
        }
    }

    fn get_span(&self) -> SpanRange {
        self.self_references
            .as_ref()
            .map(|r| r.value_real_span)
            .unwrap_or(SpanRange::single_span(self.value.span()))
    }

    fn to_tokens(&self, variant: &Variant) -> TokenStream {
        let ident = &variant.ident;
        let fields = match variant.fields {
            syn::Fields::Named(ref named) => {
                let new_named = named.named.iter().map(|n| n.ident.as_ref().unwrap());

                if let Some(metadata) = &self.self_references {
                    let new_named = new_named.map(|field_ident| {
                        let match_ = metadata
                            .references
                            .iter()
                            .find(|ref_| ref_.name() == field_ident.to_string().as_str());

                        if match_.is_some() {
                            quote!(#field_ident)
                        } else {
                            quote!(#field_ident: _)
                        }
                    });

                    quote!({#(#new_named),*})
                } else {
                    quote!({#(#new_named: _),*})
                }
            }
            syn::Fields::Unnamed(ref unnamed) => {
                let new_named = unnamed
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| Some(format_ident!("_{}", i)));

                quote!((#(#new_named),*))
            }
            syn::Fields::Unit => quote!(),
        };

        let value = &self.value;

        let value = if !self.required && !is_option_wrapped(value) {
            quote!(Some(#value))
        } else {
            quote!(#value)
        };

        quote! {
            if let Self::#ident #fields = self {
                return #value
            }
        }
    }
}

struct Attribute {
    vis: Visibility,
    ident: Ident,
    required: bool,
    type_: Type,
    values: Vec<AttributeValue>,
    default: Option<Expr>,
    config: Config,
}

impl Attribute {
    fn new(declaration: AttributeDeclaration) -> Self {
        let type_ = declaration.type_;
        let required = extract_type_from_option(&type_).is_none();

        let config = Config::new(declaration.attributes);

        Self {
            vis: declaration.vis,
            ident: declaration.ident,
            required,
            type_,
            values: Vec::new(),
            default: declaration
                .default_value
                .map(|default| default.into_value()),
            config,
        }
    }

    fn set(
        &mut self,
        variant: &Variant,
        value: AttributeValueAssignment,
        metadata: Option<AttributeMetadata>,
    ) {
        let match_ = self.values.iter().find(|v| v.variant == variant.ident);

        if let Some(value2) = match_ {
            error_duplicate!(
                value, "The value is already set for this attribute.";
                value2.get_span(), "First value of `{}` is set here.", self.ident
            );

            return;
        }

        self.values.push(AttributeValue::new(
            variant.ident.to_owned(),
            self.required,
            value.into_value(),
            metadata,
        ));
    }

    fn validate(&self, all_variants: &Punctuated<Variant, Comma>) {
        for variant in all_variants {
            let match_ = self.values.iter().find(|v| v.variant == variant.ident);

            if match_.is_none() {
                if self.default.is_some() {
                    continue;
                }

                if self.required {
                    emit_error!(
                        variant.ident,
                        format!("Value not set for `{}`.", self.ident)
                    );
                }
            }
        }
    }

    fn to_tokens(&self, all_variants: &Punctuated<Variant, Comma>) -> TokenStream {
        let function_name = self
            .config
            .function_name()
            .unwrap_or(format_ident!("get_{}", self.ident));

        let vis = &self.vis;
        let type_ = &self.type_;
        let values = self.values.iter().map(|v| {
            all_variants
                .iter()
                .find(|var| var.ident == v.variant)
                .map(|var| v.to_tokens(var))
        });

        let default = match &self.default {
            Some(value) => {
                if !is_option_wrapped(value) && !self.required {
                    quote!(Some(#value))
                } else {
                    quote!(#value)
                }
            }
            None => {
                if !self.required {
                    quote!(None)
                } else {
                    quote!(unreachable!())
                }
            }
        };

        let comment = self.config.comment();

        quote! {
            #[doc = #comment]
            #vis fn #function_name(&self) -> #type_ {
                #(#values)*

                #default
            }
        }
    }
}

fn parse_enum_attributes(attrs: &[syn::Attribute]) -> Vec<Attribute> {
    let mut attribute_declarations = Vec::<AttributeDeclaration>::new();

    for attr in attrs.iter() {
        let attr_ident = unwrap_opt_or_continue!(attr.path.get_ident());

        match attr_ident.to_string().as_str() {
            "attr" => {
                let declaration_list: ParenList<AttributeDeclaration> =
                    unwrap_or_continue!(syn::parse2(attr.tokens.to_owned()));

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
        .map(Attribute::new)
        .collect()
}

fn check_for_conflicts(attrs: &[Attribute]) {
    let mut before = HashMap::<&LitStr, &Attribute>::new();
    for attr in attrs
        .iter()
        .filter(|a| a.config.function_name_lit().is_some())
    {
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

fn parse_variant_attributes(variant: &Variant) -> Vec<AttributeValueAssignment> {
    let mut variant_attrs = Vec::new();

    for attr in &variant.attrs {
        let attr_ident = unwrap_opt_or_continue!(attr.path.get_ident());

        match attr_ident.to_string().as_str() {
            "attr" => {
                let list: ParenList<AttributeValueAssignment> =
                    unwrap_or_continue!(syn::parse2(attr.tokens.to_owned()));

                variant_attrs.extend(list.elements.into_iter());
            }
            _ => continue,
        }
    }

    variant_attrs
}

struct AttributeMetadata {
    attribute_ident: String,
    value_real_span: SpanRange,
    references: Vec<Reference>,
}

impl AttributeMetadata {
    fn new(
        attribute_ident: String,
        references: Vec<Reference>,
        value_real_span: SpanRange,
    ) -> Self {
        Self {
            attribute_ident,
            references,
            value_real_span,
        }
    }
}

fn expand_variant_attributes(variant: &mut Variant) -> Vec<AttributeMetadata> {
    let mut self_references = Vec::new();

    for attr in variant.attrs.iter_mut() {
        let attr_ident = unwrap_opt_or_continue!(attr.path.get_ident());

        match attr_ident.to_string().as_str() {
            "attr" => {
                let list: ParenList<AttributeValueAssignmentTokens> =
                    unwrap_or_continue!(syn::parse2(attr.tokens.to_owned()));

                let (list_span, elements) = list.into_parts();

                let mut new_elements = Vec::new();
                for element in elements {
                    let (ident, equal, value) = element.into_parts();

                    let processor = unwrap_or_continue!(no_emit ReferenceProcessor::parse(value.into_token_stream()));
                    let (tokens, reference_lists, real_span) = processor.into_parts();

                    for reference_list in reference_lists {
                        match reference_list.name().name() {
                            "self" => self_references.push(AttributeMetadata::new(
                                ident.to_string(),
                                reference_list.into_list(),
                                real_span,
                            )),
                            _ => reference_list.name().emit_error("Unknown reference."),
                        }
                    }

                    new_elements.push(AttributeValueAssignmentTokenStream::from_parts(
                        ident, equal, tokens,
                    ))
                }

                let new_list = ParenList::<AttributeValueAssignmentTokenStream>::from_parts(
                    list_span,
                    new_elements,
                );

                attr.tokens = new_list.to_token_stream();
            }
            _ => continue,
        }
    }

    self_references
}

pub fn derive_custom_attrs(input: DeriveInput) -> proc_macro2::TokenStream {
    let mut data_enum = match input.data {
        syn::Data::Struct(struct_) => abort!(struct_.struct_token, "Not implemented for structs."),
        syn::Data::Union(union_) => abort!(union_.union_token, "Not implemented for unions."),

        syn::Data::Enum(data_enum) => data_enum,
    };

    let mut attributes = parse_enum_attributes(&input.attrs);

    abort_if_dirty();

    for variant in data_enum.variants.iter_mut() {
        let mut self_references = expand_variant_attributes(variant);
        let variant_attrs = parse_variant_attributes(variant);

        for attr in variant_attrs {
            let opt = attributes
                .iter_mut()
                .find(|attr2| &attr2.ident == attr.ident());

            if opt.is_none() {
                emit_error!(attr.ident(), "Unknown attribute.");
                continue;
            }

            #[allow(clippy::cmp_owned)]
            let match_ = self_references
                .iter()
                .enumerate()
                .find(|(_, r)| r.attribute_ident == attr.ident().to_string())
                .map(|(i, _)| i);

            let metadata = match_.map(|i| self_references.swap_remove(i));
            opt.unwrap().set(variant, attr, metadata)
        }
    }

    for attr in attributes.iter() {
        attr.validate(&data_enum.variants);
    }

    abort_if_dirty();

    check_for_conflicts(&attributes);

    abort_if_dirty();

    let ident = &input.ident;
    let (impl_generics, generics, generic_where) = input.generics.split_for_impl();

    let tokens = attributes.iter().map(|a| a.to_tokens(&data_enum.variants));

    quote! {
        impl #impl_generics #ident #generics #generic_where {
            #(#tokens)*
        }
    }
}
