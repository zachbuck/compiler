
use proc_macro2::{Span, TokenStream};
use syn::ItemEnum;
use quote::{quote, quote_spanned};

pub enum StringEnumError {

}

pub fn generate_string_enum(ast: ItemEnum) -> Result<TokenStream, StringEnumError> {
	// Parse the AST of the enum.
	let enum_name = ast.ident;

	// Construct the output
	let out = quote! {
		enum #enum_name {

		}

		impl macros_helper::string_enum::StringEnum for #enum_name {

		}

		impl std::fmt::Display for #enum_name {

		}
	};
	let out = quote_spanned! {Span::call_site()=> #out};

	return Ok(out)
}
