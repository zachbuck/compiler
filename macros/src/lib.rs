
//! This crate contains macros for use by the compiler. These will vary in 
//! purpose as needed for implementing the compiler effectively.

#![warn(missing_docs)]

use proc_macro::TokenStream;
use syn::{Error, ItemEnum};

mod string_enum;

/// This macro generates implementations of `TryFrom<&str>` and `Display`
/// for a given enum based on pairing each enum variant to a string literal
/// that should be treated as it's equivalent
/// 
/// ## Example
/// 
/// ```
/// # use macros::generate_string_enum;
/// generate_string_enum! {
/// 	#[derive(PartialEq)]
/// 	enum Fruit {
/// 		Apple = "apple!",
/// 		Orange = "orange?",
/// 	}
/// }
/// 
/// println!("{}", Fruit::Apple); // prints 'apple!'
/// 
/// let fruit: Fruit = "orange?".try_into().unwrap();
/// assert!(Fruit::Orange == fruit);
/// 
/// let fruit: Result<Fruit, _> = "bannana".try_into();
/// assert!(Err(()) == fruit);
/// ```
/// 
/// ## Notes
/// 
/// This crate automatically generates implementations of `Clone`, `Copy`, 
/// `Display`, and `TryFrom<&str>`. If you implement these yourself for your
/// enum it could cause errors. In addition, using function-like macros inside
/// of this macro will not work and will cause errors. This may be fixed at
/// a future point.
#[proc_macro]
pub fn generate_string_enum(token_stream: TokenStream) -> TokenStream {
	use string_enum::GenerateStringEnumError::*;

	match string_enum::generate_string_enum(syn::parse_macro_input!(token_stream as ItemEnum)) {
		Ok(tokens) => tokens.into(),
		Err(MissingDiscriminant { ident_span }) => Error::new(
			ident_span, 
			format!(
				"Variant '{}' is missing a paired string value", 
				ident_span.source_text().unwrap_or(String::new())
			)
		).to_compile_error().into(),
		Err(MissingString { ident_span, expr_span }) => Error::new(
			expr_span, 
			format!(
				"Value of variant '{}' must be a string literal\nInstead found '{}'",
				ident_span.source_text().unwrap_or(String::new()),
				expr_span.source_text().unwrap_or(String::new()),
			)
		).to_compile_error().into(),
		Err(NoReprAllowed { attr_span }) => Error::new(
			attr_span,
			format!(
				"enum may not have `repr` attribute. This attribute will be set automatically by the macro.",
			),
		).to_compile_error().into(),
		Err(NoCopyCloneAllowed { attr_span }) => Error::new(
			attr_span, 
			format!(
				"enum may not derive `Clone` or `Copy`. These are automatically derived by the macro.",
			)
		).to_compile_error().into(),
	}
}
