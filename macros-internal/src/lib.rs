
use proc_macro::TokenStream;
use syn::ItemEnum;

mod string_enum;

#[proc_macro]
pub fn generate_string_enum(token_stream: TokenStream) -> TokenStream {
	match string_enum::generate_string_enum(syn::parse_macro_input!(token_stream as ItemEnum)) {
		Ok(tokens) => tokens.into(),
	}
}
