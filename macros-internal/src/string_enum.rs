
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Expr, Ident, Lit, LitStr};
use syn::spanned::Spanned;

use crate::string_enum::GenerateStringEnumError::*;

pub enum GenerateStringEnumError {
	NotAnEnum,
	MissingAttribute(Span),
	MalformedAttribute(Span),
	MissingStringLit(Span),
}

pub fn generate_string_enum(input: DeriveInput) -> Result<TokenStream, GenerateStringEnumError> {

	// Make sure that the input is an enum.
	let data = match input.data {
		Data::Enum(enum_data) => enum_data,
		_ => return Err(NotAnEnum),
	};

	let enum_ident = &input.ident;

	// Verify required formatting and data and collect.
	struct EnumMapping<'a> {
		ident: &'a Ident,
		value: &'a LitStr,
	}
	let mut enum_mapping = Vec::with_capacity(data.variants.len());
	for variant in &data.variants {
		let ident = &variant.ident;

		let attribute = variant.attrs.iter()
			.find(|attr| attr.path().is_ident("str_mapping"))
			.ok_or(MissingAttribute(variant.span()))?;

		let meta = attribute.meta.require_name_value()
			.map_err(|_| MalformedAttribute(attribute.path().span()))?;

		let literal = match &meta.value {
			Expr::Lit(lit) => Some(lit),
			_ => None,
		}.ok_or(MissingAttribute(meta.value.span()))?;

		let value = match &literal.lit {
			Lit::Str(value) => Some(value),
			_ => None,
		}.ok_or(MissingStringLit(literal.span()))?;

		enum_mapping.push(EnumMapping { ident, value });
	}

	let string_enum_def = quote! {
		impl StringEnum<'_> for #enum_ident {}
	};

	let try_from_iter = enum_mapping.iter()
		.map(|mapping| (mapping.ident, mapping.value))
		.map(|(ident, value)| quote! { #value => Ok(#enum_ident::#ident), });

	let try_from_str_def = quote! {
		impl TryFrom<&str> for #enum_ident {
			type Error = ();

			fn try_from(value: &str) -> Result<Self, Self::Error> {
				match value {
					#( #try_from_iter )*
					_ => Err(()),
				}
			}
		}
	};

	let display_iter = enum_mapping.iter()
		.map(|mapping| (mapping.ident, mapping.value))
		.map(|(ident, value)| quote! { #enum_ident::#ident => #value, });

	let display_def = quote! {
		impl std::fmt::Display for #enum_ident {
			fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				formatter.write_str(match self {
					#( #display_iter )*
				})
			}
		}
	};

	let out = quote! {
		#string_enum_def
		#try_from_str_def
		#display_def
	};

	Ok(out)
}
