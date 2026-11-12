
//! See crate macros for definitions of the members of this crate,
//! This crate is not designed to be directly consumed.

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{DeriveInput, Error, parse_macro_input};

use crate::string_enum::GenerateStringEnumError::*;

mod string_enum;

#[proc_macro_derive(StringEnum, attributes(str_mapping))]
pub fn generate_string_enum(token_stream: TokenStream) -> TokenStream {
	let input = parse_macro_input!(token_stream as DeriveInput);
	
	match string_enum::generate_string_enum(input) {
		Ok(tokens) => tokens.into(),

		Err(NotAnEnum) => Error::new(
			Span::call_site(), 
			"#[derive(StringEnum)]` should only be called on enum definitions."
		).to_compile_error().into(),

		Err(MissingAttribute(span)) => Error::new(
			span, 
			format!(
				"Missing str_mapping declaration for {}",
				span.source_text().unwrap_or("ERROR GETTING BACKING TEXT".to_string()),
			),
		).to_compile_error().into(),

		Err(MalformedAttribute(span)) => Error::new(
			span, 
			format!(
				"str_mapping attribute is malformed. Should be of form '#[str_mapping = \"name\"]'\nInstead saw '{}'",
				span.source_text().unwrap_or("ERROR GETTING BACKING TEXT".to_string()),
			)
		).to_compile_error().into(),

		Err(MissingStringLit(span)) => Error::new(
			span, 
			format!(
				"str_mapping attribute is malformed. Value should be a string literal or resolve to a string literal.\nInstead saw '{}'",
				span.source_text().unwrap_or("ERROR GETTING BACKING TEXT".to_string()),
			)
		).into_compile_error().into(),
	}
}
