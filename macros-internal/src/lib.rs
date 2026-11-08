
//! Due to rust's proc-macro crate limitations, it is not possible to export
//! other types (i.e., support functions, structs, traits, etc.) from crates
//! compiling as proc-macro crates. This crate is not meant to be used directly
//! rather it should be used through the upper level crate which republishes
//! it's macros and necessary supporting type definitions.

use proc_macro::TokenStream;
use syn::{Error, ItemEnum};

mod string_enum;

// Wrap the generate_string_enum, formatting errors and doing some pre-processing.
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
