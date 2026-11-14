
use crate::output::{Span, Spanned};

#[derive(Debug)]
pub struct Literal {
	pub(crate) data: LiteralData,
	pub(crate) span: Span,
}

impl Literal {
	pub fn get_type(&self) -> LiteralType { self.data.get_type() }
}

impl PartialEq for Literal {
	fn eq(&self, other: &Self) -> bool { self.data == other.data }
}

impl PartialEq<LiteralType> for Literal {
	fn eq(&self, other: &LiteralType) -> bool { self.data.get_type() == *other }
}

impl Spanned for Literal {
	fn get_span(&self) -> &Span { &self.span }
}

impl TryFrom<(LiteralType, &str)> for LiteralData {
	type Error = ();

	fn try_from(value: (LiteralType, &str)) -> Result<Self, Self::Error> {
		let (literal_type, string) = value;
		match literal_type {
			LiteralType::Int => string.parse().map(|i| LiteralData::Int(i)).map_err(|_| ()),
		}
	}
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum LiteralData {
	Int(i32),
}

impl LiteralData {
	fn get_type(&self) -> LiteralType {
		match self {
			LiteralData::Int(_) => LiteralType::Int,
		}
	}
}

#[non_exhaustive]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum LiteralType {
	Int,
}

