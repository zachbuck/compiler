
use proc_macro::TokenStream;
use syn::{Error, ItemEnum};

mod string_enum;

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
	}
}
