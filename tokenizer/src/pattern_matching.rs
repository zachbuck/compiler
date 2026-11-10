
use std::sync::LazyLock;

use regex::{Regex, RegexBuilder};

use crate::output::LiteralType;

pub static IGNORE: LazyLock<Box<[Regex]>> = LazyLock::new(|| {
	static IGNORE_CLASSES: &[&[&str]] = &[
		// General whitespace
		&[r"\s+"],

		// Comments
		&[r"//.*?(?:\n|$)", r"/\*.*?\*/"],
	];

	IGNORE_CLASSES.into_iter()
		// Combine Classes of things that should be ignored by the tokenizer into a single regex
		.map(|class| class.join("|")) 
		// Format them so that they only match against the start. 
		.map(|pattern| format!("^(?:{pattern})"))
		// Create the state machines.
		.map(|pattern|
			RegexBuilder::new(&pattern)
				.dot_matches_new_line(true)
				.build()
				.unwrap()
		).collect::<Box<[_]>>()
});

pub static PUNCTUATION: LazyLock<Regex> = LazyLock::new(|| {
	static PUNCT_CLASSES: &[&str] = &[
		r"()\[\]{}<>", 	// Grouping symbols
		r".,:;", 		// Seperator symbols
		r"-><>=",		// Definition symbols
		r"~&|^<>",		// Bitwise operators
		r"+\-*/",		// Arithmetic operators
		r"!&|^",		// Boolean operators
		r"=!<>",		// Comparison operators
		r"*&",			// Pointer operators
		r"=",			// Assignment
	];

	let pattern = format!(
		// Collect the strings into a parent character class.
		// Make only match against the beginning.
		"^[{}]",
		PUNCT_CLASSES.into_iter()
			// collect the strings into a character class
			.map(|class| format!("[{class}]"))
			.collect::<Vec<_>>()
			.join("")
	);

	RegexBuilder::new(&pattern)
		.build()
		.unwrap()
});

pub static KEYWORD: LazyLock<Regex> = LazyLock::new(|| {
	static KW_CLASSES: &[&[&str]] = &[
		&["fn", "return"], 	// function-related keywords
		&["int"],			// primative types
	];

	let pattern = format!(
		// Collect the keywords, make them only match against the beginning.
		r"^(?:{})",
		KW_CLASSES.into_iter()
			// Join the keywords by or operators.
			.map(|kw_class| kw_class.join("|"))
			.collect::<Vec<_>>()
			// Join the keyword classes by or operators.
			.join("|")
	);

	RegexBuilder::new(&pattern)
		.build()
		.unwrap()
});

pub static LITERAL: LazyLock<Box<[(LiteralType, Regex)]>> = LazyLock::new(|| {
	static LIT_CLASSES: &[(LiteralType, &[&str])] = &[
		(LiteralType::Int, &[r"\d+"]),
	];

	LIT_CLASSES.into_iter()
		// Join the valid patterns that each `LiteralType` can have by or operators.
		.map(|(lit_type, class)| (*lit_type, class.join("|")))
		// Format them to only match the beginning
		.map(|(lit_type, class)| (lit_type, format!(r"^(?:{class})")))
		// Construct the state machine.
		.map(|(lit_type, pattern)| 
			(
				lit_type,
				RegexBuilder::new(&pattern)
					.build()
					.unwrap()
			)
		).collect::<Box<[_]>>()
});

pub static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
	static PATTERN: &str = r"[\w--[\d]][\w]*";

	RegexBuilder::new(PATTERN)
		.build()
		.unwrap()
});

#[cfg(test)]
mod test {
	use crate::pattern_matching::*;

	// To avoid problems, none of the patterns should ever match against 
	// empty strings.
	#[test]
	fn match_empty_string() {
		assert!(IGNORE.iter().any(|regex| regex.is_match("")) == false);
		assert!(PUNCTUATION.is_match("") == false);
		assert!(KEYWORD.is_match("") == false);
		assert!(LITERAL.iter().any(|(_, regex)| regex.is_match("")) == false);
		assert!(IDENTIFIER.is_match("") == false);
	}

	// The token matchers should only match at the beginning. Test that
	// this behavior is enforced.
	#[test]
	fn match_at_beginning() {
		assert!(IGNORE.iter().any(|regex| regex.is_match("asdf    // /* */")) == false);
		assert!(PUNCTUATION.is_match("match? .,:?<>=&!") == false)
	}

	#[test]
	fn match_empty_space() {
		assert!(IGNORE.iter().find_map(|regex| regex.find(" \t\r\n asdf")).unwrap().as_str() == " \t\r\n ");
	}

	#[test]
	fn match_comments() {
		assert!(IGNORE.iter().find_map(|regex| regex.find("//test \n asdf")).unwrap().as_str() == "//test \n");
		assert!(IGNORE.iter().find_map(|regex| regex.find("//test")).unwrap().as_str() == "//test");
		assert!(IGNORE.iter().find_map(|regex| regex.find("/* test */ asdfasdf")).unwrap().as_str() == "/* test */");
	}
}
