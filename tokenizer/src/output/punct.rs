
use macros::string_enum::StringEnum;

use crate::output::{Span, Spanned};

#[derive(Debug)]
pub struct Punct {
	pub(crate) punct_type: 	PunctType,
	pub(crate) continues:	bool,
	pub(crate) span:		Span,
}

impl Punct {
	pub fn get_type(&self) -> PunctType { self.punct_type }
	pub fn get_continues(&self) -> bool { self.continues }
}

impl PartialEq for Punct {
	fn eq(&self, other: &Self) -> bool { self.punct_type == other.punct_type && self.continues == self.continues }
}

impl PartialEq<PunctType> for Punct {
	fn eq(&self, other: &PunctType) -> bool { self.punct_type == *other }
}

impl Spanned for Punct {
	fn get_span(&self) -> &Span { &self.span }
}

#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(StringEnum)]
pub enum PunctType {
	#[str_mapping = "("]
	OpenParen,

	#[str_mapping = ")"]
	CloseParen,

	#[str_mapping = "["]
	OpenBracket,

	#[str_mapping = "]"]
	CloseBracket,

	#[str_mapping = "{"]
	OpenBrace,

	#[str_mapping = "}"]
	CloseBrace,

	#[str_mapping = "."]
	Period,

	#[str_mapping = ","]
	Comma,

	#[str_mapping = ":"]
	Colon,

	#[str_mapping = ";"]
	Semicolon,

	#[str_mapping = "="]
	Equal,

	#[str_mapping = "~"]
	Tilde,

	#[str_mapping = "&"]
	And,

	#[str_mapping = "|"]
	Pipe,

	#[str_mapping = "^"]
	Caret,

	#[str_mapping = "<"]
	LessThan,

	#[str_mapping = ">"]
	GreaterThan,

	#[str_mapping = "!"]
	Exclaimation,

	#[str_mapping = "+"]
	Plus,

	#[str_mapping = "-"]
	Minus,

	#[str_mapping = "*"]
	Asterisk,

	#[str_mapping = "/"]
	Slash,
}
