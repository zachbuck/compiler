
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use syn::{Expr, ItemEnum, Lit, Meta, spanned::Spanned};

pub enum GenerateStringEnumError {
	MissingDiscriminant { ident_span: Span },
	MissingString { ident_span: Span, expr_span: Span },
	NoReprAllowed { attr_span: Span },
	NoCopyCloneAllowed { attr_span: Span },
}

pub fn generate_string_enum(syntax_tree: ItemEnum) -> Result<TokenStream, GenerateStringEnumError> {
	use GenerateStringEnumError::*;

	// Get the name of the enum.
	let enum_name = syntax_tree.ident;
	
	// Get the visibility of the enum.
	let visibility = syntax_tree.vis;

	// Get attributes.
	let attributes = syntax_tree.attrs;

	// Validate that there is no `repr` attribute.
	if let Some(attribute) = attributes.iter().find(|a| a.path().is_ident("repr")) {
		return Err(NoReprAllowed { attr_span: attribute.span(), })
	}

	// Validate that there is no `#[derive(Clone, Copy)]` attribute.
	for meta in attributes.iter()
		.filter(|a| a.path().is_ident("derive"))
		.filter_map(|a| if let Meta::List(list) = &a.meta { Some(list) } else { None } ) {
		for ident in meta.tokens.clone().into_iter()
			.filter_map(|t| if let TokenTree::Ident(ident) = t { Some(ident) } else { None } ) {
			if ident == "Clone" { return Err(NoCopyCloneAllowed { attr_span: meta.span() }) }
			if ident == "Copy"  { return Err(NoCopyCloneAllowed { attr_span: meta.span() }) }
		}
	}

	// Parse the pairs of identifiers and strings.
	let mut enum_pairs = Vec::with_capacity(syntax_tree.variants.len());
	for variant in &syntax_tree.variants {
		// Get the identifier of the enum pair.
		let ident = &variant.ident;

		// Get if there is a value assigned.
		let (_, expr) = &variant.discriminant.as_ref().ok_or(MissingDiscriminant{ ident_span: variant.span(), })?;
		
		// Make sure that it is a string literal.
		let literal = match expr {
			Expr::Lit(literal) => {
				match &literal.lit {
					Lit::Str(str) => Ok(str.value()),
					_ => Err(MissingString { ident_span: ident.span(), expr_span: expr.span() }),
				}
			},
			_ => Err(MissingString { ident_span: ident.span(), expr_span: expr.span() }),
		}?;

		let attributes = &variant.attrs[..];

		enum_pairs.push((ident, literal, attributes));
	}

	// Create the constant that will store all of the lables.
	let enum_count = enum_pairs.len();
	let literals = enum_pairs.iter()
		.map(|(_, literal, _)| literal);
	let label_list = quote! {
		const LABELS: [&'static str; #enum_count] = [ #( #literals ),* ]
	};

	// Redefine the enum in terms of usize.
	let variants = enum_pairs.iter()
		.enumerate()
		.map(|(i, (ident, _, attributes))| quote_spanned! {ident.span()=> #( #attributes )* #ident = #i });
	let enum_definition = quote! {
		#( #attributes )*
		#[repr(usize)]
		#[derive(Clone, Copy)]
		#visibility enum #enum_name {
			#( #variants ),*
		}
	};

	// Define Display for the enum
	let display_definition = quote! {
		#[automatically_derived]
		#[doc(hidden)]
		impl std::fmt::Display for #enum_name {
			fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				#label_list;
				formatter.write_str(LABELS[*self as usize])
			}
		}
	};

	// Define TryFrom<&str> for the enum
	let match_arms = enum_pairs.iter()
		.enumerate()
		.map(|(i, (_, label, _))| quote! { #label => Ok(unsafe { std::mem::transmute(#i) }) });
	let try_from_str_definition = quote! {
		#[automatically_derived]
		impl TryFrom<&str> for #enum_name {
			type Error = ();

			fn try_from(label: &str) -> Result<Self, Self::Error> {
				match label {
					#( #match_arms, )*
					_ => Err(()),
				}
			}
		}
	};

	let out = quote_spanned! {Span::call_site()=>
		#enum_definition
		#display_definition
		#try_from_str_definition
	};

	return Ok(out);
}
