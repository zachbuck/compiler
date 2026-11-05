
//! This contains definitions for the pattern matching for this library.

use std::sync::LazyLock;

use regex::Regex;
use trie_rs::map::{Trie, TrieBuilder};

use crate::token::Token;

/// Enum specifying what systems used to match against categories
/// Different implementations for different matching systems

pub enum Match {
	Trie {
		internal: Trie<u8, Token>,
	},
	Regex {
		regexes: Box<[Regex]>, 
		transform: fn(&str) -> Token
	}
}

impl Match {
	pub fn try_match(&self, input: &str) -> Option<Token> {
		match self {
			Match::Trie { internal } => {
				internal.common_prefix_search::<String, _>(input).last().map(|(_, token)| token.clone())
			},
			Match::Regex { regexes, transform } => {
				regexes.iter()
					.map(|regex| regex.captures(input))
					.filter_map(|s| s.map(|c| c.get_match().as_str()))
					.max_by_key(|s| s.len())
					.map(transform)
			}
		}
	}

	fn from_iter_trie<T, O>(iter: T) -> Self 
	where 	T: IntoIterator<Item = (&'static str, O)>,
			O: Into<Token>,
	{
		let mut builder = TrieBuilder::new();
		iter.into_iter().for_each(|(key, value)| builder.push(key, value.into()));
		let trie = builder.build();

		Match::Trie {
			internal: trie,
		}
	}

	fn from_iter_regex<T>(iter: T, transform: fn(&str) -> Token) -> Self 
	where T: IntoIterator<Item = &'static str>
	{
		let regexes = iter.into_iter()
			.map(|pattern| format!("^{pattern}"))
			.map(|pattern| Regex::new(&pattern).expect(&format!("Improper Regex '{pattern}'")))
			.collect::<Box<[_]>>();

		Match::Regex { 
			regexes, 
			transform,
		}
	}
}

/// Regexes to check, if found ignore them immediately.
pub static IGNORE: LazyLock<Match> = LazyLock::new( || {
	fn transform(input: &str) -> Token {
		Token::Literal(input.to_string())
	}

	Match::from_iter_regex(
		[
			// Ignore leading whitespace
			"\\s+",
			// Ignore comments
			"\\/\\/.*?(?:\r\n|\n)", "/\\*.*?\\*/",
		],
		transform,
	)
});

/// Special punctuation characters that have special meaning.
/// Does not support regex for added speed.
pub static PUNCTUATION: LazyLock<Match> = LazyLock::new( || {
	use crate::token::Punctuation::*;

	Match::from_iter_trie(
		[
			// seperator characters
			(".", Period), (",", Comma), (";", Semicolon), (":", Colon),
			// Assignment
			("=", Equal),
			// grouping characters
			("(", OpenParenthesis), (")", CloseParenthesis), 
			("[", OpenBracket), ("]", CloseBracket), 
			("{", OpenBrace), ("}", CloseBrace),
			// specific to function/struct definitions
			("->", Arrow), ("<", LessThan), (">", GreaterThan),
			// arithmetic operators
			("-", Minus), ("+", Plus), ("*", Asterisk), ("/", Slash), 
			("**", DoubleAsterisk),
			// bitwise operators
			("~", Tilde), ("&", And), ("|", Pipe), ("^", Caret), 
			("<<", DoubleLessThan), (">>", DoubleGreaterThan),
			// logical operators
			("!", Exclamation), ("&&", DoubleAnd), ("||", DoublePipe), 
			("^^", DoubleCaret),
			// pointer operators
			("*", Asterisk), ("&", And),
			// comparison operators (the double counting of '<' and '>' does not matter).
			("==", DoubleEqual), ("!=", ExclaimEqual), (">", GreaterThan), 
			(">=", GreaterEqual), ("<", LessThan), ("<=", LessEqual),
		],
	)
});

/// Keywords for the language.
/// Does not support regex for added speed.
pub static KEYWORDS: LazyLock<Match> = LazyLock::new( || {
	use crate::token::Keyword::*;

	Match::from_iter_trie(
		[
			// generic keywords
			("fn", Fn),
			("return", Return),
			// in-built types
			("int", Int),
		],
	)
});

/// Regexes identifying valid literals in the language.
pub static LITERALS: LazyLock<Match> = LazyLock::new( || {
	fn transform(input: &str) -> Token {
		Token::Literal(input.to_string())
	}

	Match::from_iter_regex(
		[
			// Integer Literals
			"[0-9]+",
			// String literals
			"\".*?[^\\\\]\"",
		], 
		transform
	)
});

/// Regexes identifying valid identifiers in the language.
pub static IDENTIFIER: LazyLock<Match> = LazyLock::new( || {
	fn transform(input: &str) -> Token {
		Token::Identifier(input.to_string())
	}

	Match::from_iter_regex(
		[
			"[a-zA-Z_][a-zA-Z0-9_]*",
		], 
		transform,
	)
});

pub static ALL: &[&'static LazyLock<Match>] = &[
	&IGNORE,
	&PUNCTUATION,
	&KEYWORDS,
	&LITERALS,
	&IDENTIFIER,
];
