
use std::{
	fs::File, io::{BufRead, BufReader}, path::{Path, PathBuf}, str::from_utf8_unchecked,
};

use crate::{
	output::{Keyword, Literal, LiteralType, Punct, Span, Token, TokenType, TokenizerArtifact}, pattern_matching::{IDENTIFIER, IGNORE, KEYWORD, LITERAL, PUNCTUATION},
};

pub struct Tokenizer<T> 
where T: BufRead {
	reader: T,
	bytes_read: usize,
}

#[derive(Debug)]
pub enum TokenizerError {
	FileError(PathBuf),
}

impl Tokenizer<BufReader<File>> {
	pub fn tokenize_file(source_path: &str) -> Result<TokenizerArtifact, TokenizerError> {
		// copy to local scope variable to hide that it is mutable.
		let mut source_path = source_path.to_string();

		// Correct to be absolute path if needed.
		let path = Path::new(&source_path);
		if !path.is_absolute() { 
			source_path = std::fs::canonicalize(path)
				.map_err(|_| TokenizerError::FileError(path.to_path_buf()) )?
				.into_string()
				.map_err(|_| TokenizerError::FileError(path.to_path_buf()))?;
		}

		// Check that the path is valid.
		let path = Path::new(&source_path);
		if !path.is_file() { return Err(TokenizerError::FileError(path.to_path_buf())) }

		// Open the file, build the buffer.
		let file = File::open(path).map_err(|_| TokenizerError::FileError(path.to_path_buf()))?;
		let reader = BufReader::new(file);

		// run internal tokenizer on abstract BufRead object.
		Self::tokenize(reader, source_path)
	}
}

impl<T> Tokenizer<T> 
where T: BufRead {
	pub fn tokenize(reader: T, source_name: String) -> Result<TokenizerArtifact, TokenizerError> {
		let tokenizer = Self::from_reader(reader);

		Ok(TokenizerArtifact {
			source_name,
			tokens: tokenizer.collect(),
		})
	}

	fn from_reader(reader: T) -> Self {
		Tokenizer { 
			reader, 
			bytes_read: 0usize
		}
	}

	fn claim_span(&mut self, bytes: usize) -> Span {
		let span = Span {
			start: self.bytes_read,
			len: bytes,
		};

		self.consume_bytes(bytes);

		return span;
	}

	fn consume_bytes(&mut self, bytes: usize) {
		self.reader.consume(bytes);
		self.bytes_read += bytes;
	}
}

impl<T> Iterator for Tokenizer<T> 
where T: BufRead {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		// Fill the underlying buffer
		let buffer = self.reader.fill_buf().ok()?;

		// End of file reached, finish the iterator.
		if buffer.len() == 0 { return None; }

		// Check each category in sequence
		// things that should be ignored.
		if let Some(ignore) = IGNORE.iter().find_map(|regex| regex.find(unsafe { str::from_utf8_unchecked(buffer) })) {
			let bytes = ignore.as_str().bytes().len();
			self.consume_bytes(bytes);

			return self.next();
		}

		// punctuation
		if let Some(punct) = PUNCTUATION.find(unsafe { str::from_utf8_unchecked(buffer) }) {
			let token_type = Punct::try_from(punct.as_str()).expect(&format!("Punctuation '{}' failed conversion from &str to enum Punct", punct.as_str()));
			let bytes = punct.as_str().bytes().len();
			let span = self.claim_span(bytes);

			return Some(Token {
				span,
				token_type: TokenType::Punct(token_type),
			})
		}

		if let Some(keyword) = KEYWORD.longest_prefix::<String, _>(buffer) {
			let token_type = Keyword::try_from(keyword.as_str()).expect(&format!("Keyword '{}' failed conversion from &str to enum Keyword", keyword.as_str()));
			let bytes = keyword.as_bytes().len();
			let span = self.claim_span(bytes);

			return Some(Token {
				span,
				token_type: TokenType::Keyword(token_type),
			})
		}

		if let Some((index, literal)) = LITERAL.iter().enumerate().find_map(|(i, regex)| regex.find(unsafe { str::from_utf8_unchecked(buffer) }).map(|literal| (i, literal))) {
			let literal_type = match index {
				0 => LiteralType::Int,
				_ => panic!("Unknown Literal Type Index"),
			};

			let token_type = Literal {
				literal_type,
				text: literal.as_str().to_string(),
			};

			let bytes = literal.as_str().as_bytes().len();
			let span = self.claim_span(bytes);


			return Some(Token {
				span,
				token_type: TokenType::Literal(token_type),
			});
		}

		if let Some(identifier) = IDENTIFIER.find(unsafe { from_utf8_unchecked(buffer) }) {
			let token_type = identifier.as_str().to_string();
			let bytes = identifier.as_str().bytes().len();
			let span = self.claim_span(bytes);

			return Some(Token {
				span,
				token_type: TokenType::Ident(token_type),
			})
		}

		panic!("Unknown token beginning at byte {}", self.bytes_read);
	}
}
