
use proc_macro::{Delimiter, Span, TokenStream, TokenTree};
use proc_macro2::{
	TokenStream as TokenStream2,
};
use quote::quote;

#[derive(Debug)]
pub enum StringEnumError {
	MalformedEnum,
	MalformedPair,
}

pub fn generate_string_enum(tokens: TokenStream) -> Result<TokenStream, (StringEnumError, Span)> {
	let mut tokens = tokens.into_iter();

	// Check the first few tokens
	// First token is `enum`
	let _ = match tokens.next() {
		Some(TokenTree::Ident(ident)) => {
			if ident.span().source_text().map(|str| str != "enum").unwrap_or(false) { return Err((StringEnumError::MalformedEnum, Span::call_site())) }
		},
		_ => return Err((StringEnumError::MalformedEnum, Span::call_site())),
	};
	// Second token is an enum name and extract it
	let enum_name = match tokens.next() {
		Some(TokenTree::Ident(ident)) => {
			let mut stream = TokenStream::new();
			stream.extend([ident]);
			TokenStream2::from(stream)
		},
		_ => return Err((StringEnumError::MalformedEnum, Span::call_site())),
	};
	// Third token is a group delimited by `{` and `}`
	let group = match tokens.next() {
		Some(TokenTree::Group(group)) => {
			if group.delimiter() != Delimiter::Brace { return Err((StringEnumError::MalformedEnum, Span::call_site())) }
			group.stream()
		},
		_ => return Err((StringEnumError::MalformedEnum, Span::call_site())),
	}.into_iter().collect::<Box<[_]>>();
	// There is not an extra token at the end
	let _ = match tokens.next() {
		None => (),
		Some(_) => return Err((StringEnumError::MalformedEnum, Span::call_site())),
	};

	// Split elements by comma
	let elements = group.split(|token| {
		if let TokenTree::Punct(punct) = token { 
			punct.as_char() == ','
		} else { 
			false 
		}
	});

	// Fill in a HashMap of EnumValue, String pairs
	let mut enum_map = Vec::new();
	for row in elements {
		if row.len() == 0 { continue; }
		if row.len() != 3 { return Err((StringEnumError::MalformedPair, row[0].span())); }

		let key = match &row[0] {
			TokenTree::Ident(ident) => {
				let mut stream = TokenStream::new();
				stream.extend([ident.clone()]);
				TokenStream2::from(stream)
			},
			_ => return Err((StringEnumError::MalformedPair, row[0].span())),
		};
		let _ = match &row[1] {
			TokenTree::Punct(punct) => {
				if punct.as_char() != '=' { return Err((StringEnumError::MalformedPair, row[1].span())) }
			},
			_ => return Err((StringEnumError::MalformedPair, row[1].span())),
		};
		let value = match &row[2] {
			TokenTree::Literal(literal) => {
				if let Some(text) = literal.span().source_text() {
					if !(text.starts_with("\"") && text.ends_with("\"")) { return Err((StringEnumError::MalformedPair, row[2].span())) }
				}

				let mut stream = TokenStream::new();
				stream.extend([literal.clone()]);
				TokenStream2::from(stream)
			},
			_ => return Err((StringEnumError::MalformedPair, row[2].span())),
		};

		enum_map.push((key, value));
	}

	let enum_variants = enum_map.iter()
		.map(|(key, _)| key)
		.enumerate()
		.map(|(i, key)| quote! { #key = #i });

	let all = enum_map.iter()
		.map(|(key, _)| key)
		.map(|ident| quote! { #enum_name::#ident });

	let labels = enum_map.iter()
		.map(|(_, value)| value);

	// Begin generating the new tokens
	let token_stream = quote! {
		#[derive(Clone, Copy)]
		#[repr(usize)]
		pub enum #enum_name {
			#(#enum_variants),*
		}

		impl #enum_name {
			pub const ALL: &[#enum_name] = &[ #(#all),* ];
			pub const LABELS: &[&str] = &[ #(#labels),* ];
		}

		impl std::fmt::Display for #enum_name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str(Self::LABELS[*self as usize])
			}
		}
	};

	return Ok(token_stream.into())
}
