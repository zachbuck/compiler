
//! This module contains the definitions for the output of the tokenizer. 
//! A Tokenizer which is set to collect will create a `TokenizerArtifact`. 
//! This is a stream of tokens collected into a `Vec<Token>`. The main type
//! of relevance is `Token`. See it's documentation for more information. 

use std::path::PathBuf;

use macros::string_enum::StringEnum;

/// This is the output of functions from tokenizer which collect.
/// Contains a collection of all of the tokens and the source on
/// which the tokenizer operated.
pub struct TokenizedFile {
	/// The name of the source tokenized from (i.e., the filename)
	pub path: 	PathBuf,
	/// Tokens returned in order by the tokenizer.
	pub tokens:	Vec<Token>,
}

/// The main type of interest for this module.
pub struct Token {
	/// What type of token it is. See `TokenType` for more information.
	pub token_type: TokenType,
	/// The location in the original source where the token originated from.
	/// Useful for eventual debug information. 
	pub span: 		Span,
}

/// A location in a source from which a token originates.
pub struct Span {
	/// The starting byte of the span
	pub start: 	usize,
	/// How many bytes long the span is
	pub len: 	usize,
}

/// The types of tokens that can exist. The beginning of the tree describing
/// what kind of token this token is.
#[derive(Clone)]
#[derive(Debug)]
pub enum TokenType {
	/// Some individual punctuation character
	Punct(Punct),
	/// A language reserved keyword (i.e., fn, return, etc.)
	Keyword(Keyword),
	/// A primative literal
	Literal(Literal),
	/// An identifier
	Ident(String),
}

/// An individual punctuation character
#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Punct {
	/// The type of punctuation
	pub punct_type: PunctType,
	/// If the punctuation does not have whitespace following it.
	/// Useful for compound punctuation (i.e., ">=" vs "> =").
	pub continues: 	bool,
}

#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[derive(Debug)]
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

#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(StringEnum)]
pub enum Keyword {
	#[str_mapping = "fn"]
	Fn,
	#[str_mapping = "return"]
	Return,
	#[str_mapping = "int"]
	Int,
}

/// An enum describing a primative literal.
#[derive(Clone)]
#[derive(Debug)]
pub enum Literal {
	/// A 32 bit integer
	Int(i32),
}

impl TryFrom<(LiteralType, &str)> for Literal {
	type Error = ();

	fn try_from(value: (LiteralType, &str)) -> Result<Self, Self::Error> {
		let (literal_type, text) = value;
		match literal_type {
			LiteralType::Int => Ok(Literal::Int(text.parse().map_err(|_| ())?)),
		}
	}
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub(crate) enum LiteralType {
	Int,
}
