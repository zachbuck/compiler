
use crate::output::{Span, Spanned};

#[derive(Debug)]
pub struct Ident {
	pub(crate) ident: 	String,
	pub(crate) span:	Span,
}

impl Ident {
	pub fn get_ident(&self) -> &str { return &self.ident }
}

impl Spanned for Ident {
	fn get_span(&self) -> &Span { &self.span }
}
