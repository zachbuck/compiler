
use proc_macro::TokenStream;
use syn::{ItemEnum, parse_macro_input};

mod string_enum;

#[proc_macro]
pub fn generate_string_enum(tokens: TokenStream) -> TokenStream {
	// Parse syntax tree and pass it down
	match string_enum::generate_string_enum(parse_macro_input!(tokens as ItemEnum)) {
		Ok(tokens) => tokens.into()
	}
}
