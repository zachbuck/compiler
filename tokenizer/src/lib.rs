
//! This library acts as the tokenizer for my compiler

use std::{io::BufRead, sync::LazyLock};

use crate::token::Token;

mod pattern_matching;
pub mod token;

struct Tokenizer<R> 
where R: BufRead,
{
	reader: R,
}

impl<R> Iterator for Tokenizer<R> 
where R: BufRead
{
	type Item = Token;
	
	fn next(&mut self) -> Option<Self::Item> {
		let text = unsafe { std::str::from_utf8_unchecked(self.reader.fill_buf().ok()?) };

		if let Some(token) = pattern_matching::IGNORE.try_match(text) {
			self.reader.consume(token.get_text().bytes().len());
			return self.next()
		}

		let patterns = [
			&pattern_matching::PUNCTUATION,
			&pattern_matching::KEYWORDS,
			&pattern_matching::LITERALS,
			&pattern_matching::IDENTIFIER,
		];
		for pattern in patterns {
			let result = pattern.try_match(text);
			if let Some(token) = result {
				self.reader.consume(token.get_text().bytes().len());
				return Some(token)
			}
		}

		return None
	}
}

/// make sure that all `LazyLock`s are initialized ahead of time, may reduce random lagging
pub fn init() -> () {
	pattern_matching::ALL.iter().for_each(|lock| { LazyLock::force(lock); });
}

pub fn tokenize(reader: impl BufRead) -> impl Iterator<Item = Token> {
	Tokenizer {
		reader: reader,
	}
}
