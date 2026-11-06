
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::ItemEnum;

pub enum GenerateStringEnumError {

}

pub fn generate_string_enum(syntax_tree: ItemEnum) -> Result<TokenStream, GenerateStringEnumError> {
	use GenerateStringEnumError::*;

	let enum_name = syntax_tree.ident;

	let mut enum_variants = Vec::with_capacity(syntax_tree.variants.len());
	for variant in syntax_tree.variants {
		enum_variants.push(variant.ident);
	}

	let out = quote_spanned! {Span::call_site()=>
		enum #enum_name {
			#( #enum_variants ),*
		}
	};

	return Ok(out);
}
