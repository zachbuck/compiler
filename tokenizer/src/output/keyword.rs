
use macros::string_enum::StringEnum;

use crate::output::{Span, Spanned};

#[derive(Debug)]
pub struct Keyword {
	pub(crate) keyword_type: 	KeywordType,
	pub(crate) span: 			Span,
}

#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(StringEnum)]
pub enum KeywordType {
	#[str_mapping = "fn"]
	Fn,

	#[str_mapping = "return"]
	Return,

	#[str_mapping = "int"]
	Int,
}

impl Keyword {
	pub fn get_type(&self) -> KeywordType { self.keyword_type }
}

impl PartialEq for Keyword {
	fn eq(&self, other: &Self) -> bool { self.keyword_type == other.keyword_type }
}

impl PartialEq<KeywordType> for Keyword {
	fn eq(&self, other: &KeywordType) -> bool { self.keyword_type == *other }
}

impl Spanned for Keyword {
	fn get_span(&self) -> &Span { &self.span }
}
