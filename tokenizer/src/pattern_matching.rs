
use std::sync::LazyLock;

use regex::{Regex, RegexBuilder};
use trie_rs::Trie;

pub static IGNORE: LazyLock<Box<[Regex]>> = LazyLock::new(|| {
	const PATTERN_COUNT: usize = 3;

	const PATTERNS: [&str; PATTERN_COUNT] = [
		// General whitespace
		r"\s+",

		// Comments
		r"//.*?(\n|$)",
		r"/\*.*?\*/",
	];

	let regexes = PATTERNS
		.map(|pattern| format!("^{pattern}"))
		.map(|regex| 
			RegexBuilder::new(&regex)
				.dot_matches_new_line(true)
				.build()
				.unwrap()
		);

	Box::new(regexes)
});

pub static PUNCTUATION: LazyLock<Regex> = LazyLock::new(|| {
	const CLASS_COUNT: usize = 9;

	// These will all be combined into a character class. They will be combined into
	// multi character symbols or interpreted into their syntactic purpose later.
	// Some will be defined more than once, this should be optimized out by the regex engine.
	// This is to make sure that all symbols are where they should be in case the grammer changes
	// at some later point
	const CLASSES: [&str; CLASS_COUNT] = [
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
		"^[{}]",
		CLASSES
			.map(|class| format!("[{class}]"))
			.join("")
	);

	RegexBuilder::new(&pattern)
		.build()
		.unwrap()
});

pub static KEYWORD: LazyLock<Trie<u8>> = LazyLock::new(|| {
	static CLASS_COUNT: usize = 2;

	static KEYWORDS: [&[&str]; CLASS_COUNT] = [
		&["fn", "return"], // function-related keywords
		&["int"], // primitive types
	];

	let keywords = KEYWORDS.into_iter().flatten();

	Trie::from_iter(keywords)
});

pub static LITERAL: LazyLock<Box<[Regex]>> = LazyLock::new(|| {
	static CLASS_COUNT: usize = 1;

	static CLASSES: [&[&str]; CLASS_COUNT] = [
		&[r"\d+"], // Integer
	];

	let regexes = CLASSES
		.map(|class| class.join("|"))
		.map(|class| format!("^{class}"))
		.map(|pattern| 
			RegexBuilder::new(&pattern)
				.build()
				.unwrap()
		);

	Box::new(regexes)
});

pub static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
	static PATTERN: &str = r"[\w][\w\d]*";

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
		assert!(KEYWORD.longest_prefix::<String, _>("").is_none());
		assert!(LITERAL.iter().any(|regex| regex.is_match("")) == false);
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
