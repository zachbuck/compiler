
//! This contains definitions for the pattern matching for the tokenizer.

use std::sync::LazyLock;

use regex::Regex;
use trie_rs::{Trie, TrieBuilder};

/// Enum specifying what systems used to match against categories
/// Different implementations for different matching systems
pub enum Matcher {
	Trie(Trie<u8>),
	Regex(Box<[Regex]>),
}

impl Matcher {
	pub fn matches<'a>(&self, input: &'a str) -> Option<&'a str> {
		match self {
			Matcher::Trie(trie) => {
				trie.longest_prefix(input).map(|s: String| &input[0..s.len()])
			},
			Matcher::Regex(regexes) => {
				regexes.iter()
					.map(|regex| regex.captures(input))
					.filter_map(|c| c.map(|c| c.get_match().as_str()))
					.max_by_key(|s| s.len())
			},
		}
	}

	fn from_iter_trie<T>(iter: T) -> Self
	where T: IntoIterator<Item = &'static &'static str>
	{
		let mut builder = TrieBuilder::new();
		iter.into_iter().for_each(|pattern| builder.push(pattern));
		let trie = builder.build();

		Matcher::Trie(trie)
	}

	fn from_iter_regex<T>(iter: T) -> Self
	where T: IntoIterator<Item = &'static &'static str>
	{
		let regexes = iter.into_iter()
			// make sure that the regex only matches the start of the input.
			.map(|pattern| format!("^{pattern}"))
			.map(|pattern| Regex::new(&pattern).unwrap())
			.collect();

		Matcher::Regex(regexes)
	}
}

macro_rules! from_static_str_slice {
	(trie, $slice: expr) => {
		{
			LazyLock::new(|| Matcher::from_iter_trie($slice))
		}
	};
	(regex, $slice: expr) => {
		{
			LazyLock::new(|| Matcher::from_iter_regex($slice))
		}
	}
}

/// Regexes to check, if found ignore them immediately.
pub static IGNORE: LazyLock<Matcher> = from_static_str_slice!(regex, &[
	// Ignore leading whitespace
	"\\S+",
]);

/// Special punctuation characters that have special meaning.
/// Does not support regex for added speed.
pub static PUNCTUATION: LazyLock<Matcher> = from_static_str_slice!(trie, &[
	// seperator characters
	".", ",", ";",
	// grouping characters
	"(", ")", "[", "]", "{", "}",
	// specific to function/struct definitions
	"->", "<", ">",
]);

/// Operators for the language.
/// Does not support regex for added speed.
static OPERATORS: LazyLock<Matcher> = from_static_str_slice!(trie, &[
	// numerical operators
	"-", "+", "-", "*", "/", "**",
	// binary bitwise operators
	"~", "&", "|", "^", "<<", ">>",
	// logical operators
	"!", "&&", "||", "^^",
	// pointer operators
	"*", "&",
	// comparison operators
	"==", "!=", ">", ">=", "<", "<=",
]);

/// Keywords for the language.
/// Does not support regex for added speed.
static KEYWORDS: LazyLock<Matcher> = from_static_str_slice!(trie, &[
	// generic keywords
	"fn",
	"return",
	// in-built types
	"int",
]); 

/// Regexes identifying valid literals in the language.
static LITERALS: LazyLock<Matcher> = from_static_str_slice!(regex, &[
	// Integer Literals
	"[0-9]+",
	// String literals
	"\".*?[^\\]\"",
]);

/// Regexes identifying valid identifiers in the language.
static IDENTIFIER: LazyLock<Matcher> = from_static_str_slice!(regex, &[
	"[a-zA-Z_][a-zA-Z0-9_]*",
]);
