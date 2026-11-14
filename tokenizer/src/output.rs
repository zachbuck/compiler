
use std::{ops::Range, path::{Path, PathBuf}};

pub use crate::output::{
	ident::Ident,
	keyword::{Keyword, KeywordType},
	literal::{Literal, LiteralData, LiteralType},
	punct::{Punct, PunctType},
};

mod ident;
mod keyword;
mod literal;
mod punct;

pub struct File {
	pub(crate) filename: 	PathBuf,
	pub(crate) tokens: 		Vec<Token>,
}

impl File {
	pub fn get_filename(&self) -> &Path { &self.filename }
	pub fn get_tokens(&self) -> &[Token] { &self.tokens }
}

#[derive(Debug)]
pub struct Span {
	pub(crate) beg_byte: usize,
	pub(crate) len_byte: usize,
}

impl Span {
	pub fn first_byte(&self) -> usize { self.beg_byte }
	pub fn byte_count(&self) -> usize { self.len_byte }
	pub fn byte_num_iter(&self) -> Range<usize> { self.beg_byte..(self.beg_byte + self.len_byte) }
}

pub trait Spanned {
	fn get_span(&self) -> &Span;
}

#[derive(Debug)]
pub enum Token {
	Punct(Punct),
	Keyword(Keyword),
	Literal(Literal),
	Ident(Ident),
}

#[derive(PartialEq, Eq)]
pub enum TokenType {
	Punct,
	Keyword,
	Literal,
	Ident,
}

impl Token {
	pub fn get_type(&self) -> TokenType {
		match self {
			Token::Punct(_) 	=> TokenType::Punct,
			Token::Keyword(_) 	=> TokenType::Keyword,
			Token::Literal(_) 	=> TokenType::Literal,
			Token::Ident(_) 	=> TokenType::Ident,
		}
	}
}
