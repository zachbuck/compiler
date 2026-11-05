
use proc_macro::TokenStream;
use syn::Error;

mod string_enum;

#[proc_macro]
pub fn generate_string_enum(tokens: TokenStream) -> TokenStream {
	match string_enum::generate_string_enum(tokens) {
		Ok(tokens) 	=> tokens,
		Err((err, span)) 	=> Error::new(span.into(), format!("{:?}", err)).to_compile_error().into(),
	}
}
