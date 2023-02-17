use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro_error::{abort, emit_error, proc_macro_error, abort_if_dirty};
use quote::{quote, ToTokens, format_ident};
use syn::{
    parse::Parse, punctuated::Punctuated, token::{Comma, CustomToken}, DataEnum, DeriveInput, Expr, Token, Type,
    Variant, Visibility,
};

macro_rules! unwrap_opt_or_continue {
    ($expr: expr) => {{
        let res = $expr;
        if res.is_none() {
            continue;
        }
        res.unwrap()
    }};
}

struct AttributeDefaultValue {
    _equal: Token!(=),
    value: Expr
}

impl Parse for AttributeDefaultValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _equal: input.parse()?,
            value: input.parse()?
        })
    }
}

impl CustomToken for AttributeDefaultValue {
    fn peek(cursor: syn::buffer::Cursor) -> bool {
        if let Some((punct, _)) = cursor.punct() {
            if punct.as_char() == '=' {
                return true
            }
        }
        return false
    }

    fn display() -> &'static str {
        todo!()
    }
}

struct AttributeDeclaration {
    vis: Visibility,
    ident: Ident,
    _colon: Token!(:),
    type_: Type,
    default_value: Option<AttributeDefaultValue>
}

impl Parse for AttributeDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
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
            default_value: input.parse()?
        })
    }
}

struct AttributeValueAssignment {
    ident: Ident,
    _equal: Token!(=),
    value: Expr,
}

impl Parse for AttributeValueAssignment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            _equal: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl ToTokens for AttributeValueAssignment {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.ident.to_tokens(tokens);
        self._equal.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

enum AttributeState {
    NotSet,
    Set(Expr),
}

struct AttributeValue<'f> {
    variant_ident: &'f Ident,
    value: AttributeState,
}

impl<'f> AttributeValue<'f> {
    pub fn new(field_ident: &'f Ident) -> Self {
        Self {
            variant_ident: field_ident,
            value: AttributeState::NotSet,
        }
    }
}

impl<'f> ToTokens for AttributeValue<'f> {
    fn to_tokens(&self, tokens2: &mut proc_macro2::TokenStream) {
        if let AttributeState::NotSet = &self.value {
            return proc_macro2::TokenStream::new().to_tokens(tokens2)
        }
        
        let ident = &self.variant_ident;
        let value = match &self.value {
            AttributeState::Set(value) => value,
            _ => unreachable!()
        };

        let tokens = quote! {
            if let Self::#ident = self {
                return #value
            }
        };

        tokens.to_tokens(tokens2)
    }
}

struct Attribute<'f> {
    vis: Visibility,
    ident: Ident,
    type_: Type,
    values: Vec<AttributeValue<'f>>,
    default: Option<Expr>
}

impl<'f> Attribute<'f> {
    fn new(declaration: AttributeDeclaration, variants: &'f Punctuated<Variant, Comma>) -> Self {
        let values = variants
            .iter()
            .map(|f| AttributeValue::new(&f.ident))
            .collect();

        Self {
            vis: declaration.vis,
            ident: declaration.ident,
            type_: declaration.type_,
            values,
            default: declaration.default_value.map(|default| default.value)
        }
    }

    fn set(&mut self, ident: &Ident, value: AttributeValueAssignment) {
        let attr_value = self
            .values
            .iter_mut()
            .find(|p| p.variant_ident == ident)
            .expect("tried to set a value for a variant that doesn't exists.");

        match attr_value.value {
            AttributeState::NotSet => attr_value.value = AttributeState::Set(value.value),
            AttributeState::Set(ref value2) => {
                emit_error!(value2, format!("First value of `{}` is set here.", self.ident));
                emit_error!(value, "The value is already set for this attribute.")
            }
        }
    }

    fn check(&self) {
        if self.default.is_some() {
            return
        }

        for value in &self.values {
            if let AttributeState::NotSet = value.value {
                emit_error!(value.variant_ident, format!("Value not set for `{}`.", self.ident));
            }
        }
    }
}

impl<'f> ToTokens for Attribute<'f> {
    fn to_tokens(&self, tokens2: &mut proc_macro2::TokenStream) {
        let function_name = format_ident!("get_{}", self.ident);
        
        let vis = &self.vis;
        let type_ = &self.type_;
        let values = &self.values;

        let default = match &self.default {
            Some(value) => quote!(#value),
            None => quote!( unreachable!() ),
        };

        let tokens = quote! {
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
                let res = attr.parse_args();
                if let Err(e) = res {
                    emit_error!(e.span(), e);
                    continue;
                }

                let attr: AttributeDeclaration = res.unwrap();

                let match_ = attribute_declarations.iter()
                    .find(|attr2| attr.ident == attr2.ident);

                if match_.is_some() {
                    emit_error!(attr.ident, "This attribute is already declared.");
                    continue;
                }

                attribute_declarations.push(attr);
            }

            _ => continue,
        }
    }

    attribute_declarations
        .into_iter()
        .map(|decl| Attribute::new(decl, &data_enum.variants))
        .collect()
}

fn parse_variant_attributes(variant: &Variant) -> Vec<AttributeValueAssignment> {
    let mut variant_attrs = Vec::new();
    
    for attr in &variant.attrs {
        let attr_ident = unwrap_opt_or_continue!(attr.path.get_ident());

        match attr_ident.to_string().as_str() {
            "attr" => {
                let res = attr.parse_args::<AttributeValueAssignment>();
                if let Err(e) = res {
                    emit_error!(e.span(), e);
                    continue;
                }

                variant_attrs.push(res.unwrap());
            }
            _ => continue
        }
    }

    variant_attrs
}

fn _derive_custom_attrs(input: DeriveInput) -> proc_macro2::TokenStream {
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
            let opt = attributes.iter_mut()
                .find(|attr2| attr2.ident == attr.ident);

            if opt.is_none() {
                emit_error!(attr.ident, "Unknown attribute.");
                continue;
            }

            opt.unwrap().set(&variant.ident, attr)
        }
    }

    for attr in attributes.iter() {
        attr.check();
    }

    abort_if_dirty();

    let ident = &input.ident;
    let (impl_generics, generics, generic_where) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #ident #generics #generic_where {
            #(#attributes)*
        }
    }
}

#[proc_macro_derive(CustomAttrs, attributes(attr))]
#[proc_macro_error]
pub fn derive_custom_attrs(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);

    _derive_custom_attrs(derive_input).into()
}
