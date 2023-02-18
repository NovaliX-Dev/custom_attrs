//! # Custom Attrs
//! 
//! A library that allows you to configure values specific to each variants of an enum.
//! 
//! ## Installation and Usage
//! 
//! Add this to your `Cargo.toml` file :
//! ```toml
//! [dependencies]
//! custom_attrs = "1.2.2"
//! ```
//! 
//! Then you can use the `derive` attribute to use the library.
//! 
//! ```rust
//! use custom_attrs::CustomAttrs;
//! 
//! #[derive(CustomAttrs)]
//! 
//! // all attributes declarations will be here.
//! 
//! enum Enum {
//!     // ...
//! }
//! ```
//! 
//! ### Attribute declaration.
//! 
//! By default, an attribute declaration is composed of two parts : the attribute's name and it's type.
//! 
//! ```rust, ignore
//! #[attr(name: u32)]
//! ```
//! 
//! Optionally, you can add more components.
//! 
//! ### Optional components
//! 
//! You can set the visibility of the attribute. This will change the visibility of the getter function.
//! 
//! ```rust, ignore
//! #[attr(pub name: u32)]
//! enum Enum {}
//! ```
//! 
//! By default, each attribute declared require a value to be set for each variant.
//! If this requirement is not set, this library will produce an error.
//! 
//! You can disable this behavior by making it optional, by writing the type into an `Option`, or by adding a default value behind the attribute declaration. See the example below.
//! 
//! ```rust, ignore
//! #[attr(name: Option<u32>)]
//! enum Enum {}
//! ```
//! 
//! ```rust, ignore
//! #[attr(name: u32 = 3)]
//! enum Enum {}
//! ```
//! 
//! ### Setting a value
//! 
//! To set a value for a variant, just add the name of the attribute followed by the value you want to assign it.
//! 
//! 
//! ```rust, ignore
//! enum Enum {
//!     #[attr(name = 4)]
//!     VariantA
//! }
//! ```
//! 
//! If the attribute is optional, you don't have to wrap you value into a `Some`. `custom_attrs` will do it for you. If you want the value to be `None`, just put `None` behind the value.
//! 
//! ```rust, ignore
//! enum Enum {
//!     #[attr(name = 4)]
//!     VariantA,
//! 
//!     #[attr(name = None)]
//!     VariantA,
//! }
//! ```
//! 
//! ## Examples
//! 
//! ```rust
//! use custom_attrs::CustomAttrs;
//! 
//! // ...
//! 
//! #[derive(CustomAttrs)]
//! 
//! // set the attributes
//! #[attr(pub a: usize)]
//! #[attr(b: Option<usize>)] // attributes can be optional
//! #[attr(c: &'static str = "Hello world!")] // and can also have default values
//! enum Enum {
//!     #[attr(a = 5)]
//!     #[attr(b = 3)]
//!     Variant1,
//! 
//!     #[attr(a = 3)]
//!     #[attr(c = "Hello again !")]
//!     Variant2,
//! }
//! 
//! fn main() {
//!     let a = Enum::Variant1.get_a();
//! }
//! ```
//! 
//! See the examples directory for more details.
//! 
//! # License
//! 
//! Licensed under the MIT license.


use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::DeriveInput;

mod derive;

/// The main trait of the library.
///
/// ## Syntax
/// 
/// First, use the `derive` attribute to use the library.
/// 
/// ```rust
/// use custom_attrs::CustomAttrs;
/// 
/// #[derive(CustomAttrs)]
/// 
/// // all attributes declarations will be here.
/// 
/// enum Enum {
///     // ...
/// }
/// ```
/// 
/// ### Attribute declaration.
/// 
/// By default, an attribute declaration is composed of two parts : the attribute's name and it's type.
/// 
/// ```rust, ignore
/// #[attr(name: u32)]
/// ```
/// 
/// Optionally, you can add more components.
/// 
/// ### Optional components
/// 
/// You can set the visibility of the attribute. This will change the visibility of the getter function.
/// 
/// ```rust, ignore
/// #[attr(pub name: u32)]
/// enum Enum {}
/// ```
/// 
/// By default, each attribute declared require a value to be set for each variant.
/// If this requirement is not set, this library will produce an error.
/// 
/// You can disable this behavior by making it optional, by writing the type into an `Option`, or by adding a default value behind the attribute declaration. See the example below.
/// 
/// ```rust, ignore
/// #[attr(name: Option<u32>)]
/// enum Enum {}
/// ```
/// 
/// ```rust, ignore
/// #[attr(name: u32 = 3)]
/// enum Enum {}
/// ```
/// 
/// ### Setting a value
/// 
/// To set a value for a variant, just add the name of the attribute followed by the value you want to assign it.
/// 
/// 
/// ```rust, ignore
/// enum Enum {
///     #[attr(name = 4)]
///     VariantA
/// }
/// ```
/// 
/// If the attribute is optional, you don't have to wrap you value into a `Some`. `custom_attrs` will do it for you. If you want the value to be `None`, just put `None` behind the value.
/// 
/// ```rust, ignore
/// enum Enum {
///     #[attr(name = 4)]
///     VariantA,
/// 
///     #[attr(name = None)]
///     VariantA,
/// }
/// ```
/// 
/// ## Examples
///
/// ```
/// use custom_attrs::CustomAttrs;
///
/// // ...
///
/// #[derive(CustomAttrs)]
///
/// // set the attributes
/// #[attr(pub a: usize)]
/// #[attr(b: Option<usize>)] // attributes can be optional
/// #[attr(c: &'static str = "Hello world!")] // and can also have default values
/// enum Enum {
///     #[attr(a = 5)]
///     #[attr(b = 3)]
///     Variant1,
///
///     #[attr(a = 3)]
///     #[attr(c = "Hello again !")]
///     Variant2,
/// }
///
/// fn main() {
///     let a = Enum::Variant1.get_a();
/// }
/// ```
///
/// See the examples directory for more details.
#[proc_macro_derive(CustomAttrs, attributes(attr))]
#[proc_macro_error]
pub fn derive_custom_attrs(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);

    derive::derive_custom_attrs(derive_input).into()
}
