
//! This module contains the definitions for the output of the tokenizer. 
//! A Tokenizer which is set to collect will create a `TokenizerArtifact`. 
//! This is a stream of tokens collected into a `Vec<Token>`. The main type
//! of relevance is `Token`. See it's documentation for more information. 

use std::path::PathBuf;

use macros::string_enum::generate_string_enum;

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

generate_string_enum! {
	/// A punctuation type
	#[allow(missing_docs)]
	#[derive(Debug)]
	pub enum PunctType {
		OpenParen 		= "(",
		CloseParen 		= ")",
		OpenBracket 	= "[",
		CloseBracket 	= "]",
		OpenBrace 		= "{",
		CloseBrace 		= "}",
		Period 			= ".",
		Comma 			= ",",
		Colon 			= ":",
		Semicolon 		= ";",
		Equal 			= "=",
		Tilde 			= "~",
		And 			= "&",
		Pipe 			= "|",
		Caret 			= "^",
		LessThan 		= "<",
		GreaterThan 	= ">",
		Exclaimation 	= "!",
		Plus 			= "+",
		Minus 			= "-",
		Asterisk 		= "*",
		Slash 			= "/",
	}
}

generate_string_enum! {
	/// A language reserved keyword
	#[allow(missing_docs)]
	#[derive(Debug)]
	pub enum Keyword {
		Fn 		= "fn",
		Return 	= "return",
		Int		= "int",
	}
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
