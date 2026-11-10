
//! This module contains the tokenizer which is the method
//! of converting some stream of text into tokens.

use std::{
	fs::File, 
	io::{BufRead, BufReader}, 
	path::{Path, PathBuf},
};

use crate::{
	output::{Keyword, Literal, LiteralType, Punct, PunctType, Span, Token, TokenType, TokenizedFile}, 
	pattern_matching::{IDENTIFIER, IGNORE, KEYWORD, LITERAL, PUNCTUATION},
};

/// The tokenizer struct. Iterates over text and turns it into tokens.
pub struct Tokenizer<T> 
where T: BufRead {
	reader: T,
	bytes_read: usize,
}

/// Errors that may be returned from the creation of a tokenizer. 
#[non_exhaustive]
#[derive(Debug)]
pub enum TokenizerError {
	/// Some sort of error with accessing a file. 
	/// - File does not exist.
	/// - Missing permissions.
	/// - File's path cannot be represented in UTF-8.
	/// - Etc.
	FileError(PathBuf),
}

#[derive(Debug)]
enum TokenClaimError {
	NoTokenFound,
	BufferAccessError,
}

impl Tokenizer<BufReader<File>> {
	/// Generate a collection of all of the tokens in a file.
	/// Takes a path `source_path`.
	/// Returns a `TokenizerArtifact` containing all of the tokens.
	/// Returns `Err` if an error occurs.
	pub fn tokenize_file(source_path: &str) -> Result<TokenizedFile, TokenizerError> {
		let path = Self::check_path(source_path)?;

		// Open the file, build the buffer.
		let file = File::open(&path).map_err(|_| TokenizerError::FileError(path.clone()))?;
		let reader = BufReader::new(file);

		// run internal tokenizer on abstract BufRead object.
		let tokens = Self::tokenize(reader);

		Ok(TokenizedFile {
			path,
			tokens
		})
	}

	/// Generate an `Iterator` over the tokens in a file.
	/// Takes a path `source_path`.
	/// Returns `Tokenizer` which is an iterator over the tokens in the file.
	/// Returns `Err` if an error occurs. 
	pub fn iter_file(source_path: &str) -> Result<Self, TokenizerError> {
		let path = Self::check_path(source_path)?;

		let file = File::open(&path).map_err(|_| TokenizerError::FileError(path.clone()))?;
		let reader = BufReader::new(file);

		Ok(Self::iter(reader))
	}

	fn check_path(source_path: &str) -> Result<PathBuf, TokenizerError> {
		let mut source_path = source_path.to_string();

		// Make the path absolute.
		if !Path::new(&source_path).is_absolute() {
			source_path = std::fs::canonicalize(source_path.clone())
				.map_err(|_| TokenizerError::FileError(source_path.clone().into()))?
				.into_string()
				.map_err(|_| TokenizerError::FileError(source_path.clone().into()))?;
		}

		// Check validity of File.
		if !Path::new(&source_path).is_file() { return Err(TokenizerError::FileError(source_path.into())) }

		Ok(source_path.into())
	}
}

impl<T> Tokenizer<T> 
where T: BufRead {
	/// Generates a collection of tokens from a buffered reader.
	pub fn tokenize(reader: T) -> Vec<Token> {
		Self::iter(reader).collect()
	}

	/// Generates an iterator returning tokens from a buffered reader.
	pub fn iter(reader: T) -> Self {
		Tokenizer { 
			reader, 
			bytes_read: 0usize,
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

	// Detects if the buffer starts with a match to the `IGNORE`
	// pattern matching set. Returns the size matched in bytes
	fn starts_with_ignore(buffer: &[u8]) -> usize {
		let haystack = str::from_utf8(buffer).unwrap();
		match IGNORE.iter().find_map(|regex| regex.find(haystack)) {
			Some(pattern_match) => pattern_match.as_str().bytes().len(),
			None => 0
		}
	}

	// Claims an `IGNORE` token if it exists.
	// Returns `Err()` on not found or error
	// Returns `Ok(())` if an `IGNORE` token is found.
	fn claim_ignore(&mut self) -> Result<(), TokenClaimError> {
		let buffer = self.reader.fill_buf().map_err(|_| TokenClaimError::BufferAccessError)?;
		let byte_count = Self::starts_with_ignore(buffer);
		if byte_count == 0 { return Err(TokenClaimError::NoTokenFound) }

		self.consume_bytes(byte_count);
		Ok(())
	}

	// Detects if the buffer starts with a match to the `PUNCTUATION`
	// pattern matching set. Returns the size matched in bytes.
	fn starts_with_punct(buffer: &[u8]) -> usize {
		let haystack = str::from_utf8(buffer).unwrap();
		match PUNCTUATION.find(haystack) {
			Some(pattern_match) => pattern_match.as_str().bytes().len(),
			None => 0,
		}
	}

	// Claims a `PUNCTUATION` token if it exists.
	// Returns `Err()` on not found or error
	// Returns `Ok(token)` on success.
	fn claim_punct(&mut self) -> Result<Token, TokenClaimError> {
		let buffer = self.reader.fill_buf().map_err(|_| TokenClaimError::BufferAccessError)?;

		let byte_count = Self::starts_with_punct(buffer);
		if byte_count == 0 { return Err(TokenClaimError::NoTokenFound) }

		let punct_type = PunctType::try_from(str::from_utf8(&buffer[..byte_count]).unwrap()).unwrap();
		let continues = Self::starts_with_punct(&buffer[byte_count..]) != 0;
		let span = self.claim_span(byte_count);

		Ok(Token {
			token_type: TokenType::Punct(Punct { 
				punct_type, 
				continues,
			}),
			span,
		})
	}

	// Detects if the buffer starts with a match to the `KEYWORD`
	// pattern matching set. Returns size matched in bytes.
	fn starts_with_keyword(buffer: &[u8]) -> usize {
		let haystack = str::from_utf8(buffer).unwrap();
		match KEYWORD.find(haystack) {
			Some(pattern_match) => pattern_match.as_str().as_bytes().len()	,
			None => 0,
		}
	}

	// Claims a `KEYWORD` token if it exists.
	// Returns `Err()` on not found or error
	// Returns `Ok(token)` on success.
	fn claim_keyword(&mut self) -> Result<Token, TokenClaimError> {
		let buffer = self.reader.fill_buf().map_err(|_| TokenClaimError::BufferAccessError)?;

		let byte_count = Self::starts_with_keyword(buffer);
		if byte_count == 0 { return Err(TokenClaimError::NoTokenFound) }

		// Make sure that the keyword is followed by some whitespace (i.e., prevent it from matching keywords)
		if 		Self::starts_with_ignore(&buffer[byte_count..]) == 0 
			&&	Self::starts_with_punct(&buffer[byte_count..]) == 0 { return Err(TokenClaimError::NoTokenFound) }

		let keyword = Keyword::try_from(str::from_utf8(&buffer[..byte_count]).unwrap()).unwrap(); 
		let span = self.claim_span(byte_count);

		Ok(Token {
			token_type: TokenType::Keyword(keyword),
			span: span,
		})
	}

	// Detects if the buffer starts with a match to the `LITERAL`
	// pattern matching set. A bit different since we also need which LiteralType
	// it matched against. Returns size in bytes.
	fn starts_with_literal(buffer: &[u8]) -> Option<(LiteralType, usize)> {
		let haystack = str::from_utf8(buffer).unwrap();
		LITERAL.iter()
			.find_map(|(lit_type, regex)| 
				regex.find(haystack).map(|pattern_match| (*lit_type, pattern_match))
			).map(|(lit_type, pattern_match)| (lit_type, pattern_match.as_str().as_bytes().len()))
	}

	// Claims a `LITERAL` token if it exists.
	// Returns `Err()` on not found or error.
	// Returns `Ok(token)` on success.
	fn claim_literal(&mut self) -> Result<Token, TokenClaimError> {
		let buffer = self.reader.fill_buf().map_err(|_| TokenClaimError::BufferAccessError)?;

		let (lit_type, byte_count) = Self::starts_with_literal(buffer).ok_or(TokenClaimError::NoTokenFound)?;
		if byte_count == 0 { return Err(TokenClaimError::NoTokenFound) }

		// Make sure that literal is followed by whitespace or punctuation, (i.e., prevent malformed grammer of literal
		// directly next to an identifier or keyword, etc.)
		if 		Self::starts_with_ignore(&buffer[byte_count..]) == 0 
			&& 	Self::starts_with_punct(&buffer[byte_count..]) == 0 { return Err(TokenClaimError::NoTokenFound) }

		let literal = Literal::try_from((lit_type, str::from_utf8(&buffer[..byte_count]).unwrap())).unwrap();
		let span = self.claim_span(byte_count);

		Ok(Token {
			token_type: TokenType::Literal(literal),
			span,
		})
	}

	// Detects if the buffer starts with a match to the `IDENFIFIER`
	// pattern matching set.  Returns size in bytes.
	fn starts_with_identifier(buffer: &[u8]) -> usize {
		let haystack = str::from_utf8(buffer).unwrap();
		match IDENTIFIER.find(haystack) {
			Some(pattern_match) => pattern_match.as_str().as_bytes().len(),
			None => 0,
		}
	}

	// Claims an `Identifier` token if  it exists.
	// Returns `Err()` on not found or error.
	// Returns `Ok(token)` on success.
	fn claim_identifier(&mut self) -> Result<Token, TokenClaimError> {
		let buffer = self.reader.fill_buf().map_err(|_| TokenClaimError::BufferAccessError)?;

		let byte_count = Self::starts_with_identifier(buffer);
		if byte_count == 0 { return Err(TokenClaimError::NoTokenFound) }

		let ident = str::from_utf8(&buffer[..byte_count]).unwrap().to_string();
		let span = self.claim_span(byte_count);

		Ok(Token {
			token_type: TokenType::Ident(ident),
			span,
		})
	}
}

impl<T> Iterator for Tokenizer<T> 
where T: BufRead {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.claim_ignore() {
				Ok(()) 								=> continue,
				Err(TokenClaimError::NoTokenFound) 	=> break,
				Err(err) 			=> panic!("{err:?}"),
			}
		}

		match self.claim_punct() {
			Ok(token) 					=> return Some(token),
			Err(TokenClaimError::NoTokenFound) 	=> (),
			Err(err) 			=> panic!("{err:?}"),
		}

		match self.claim_keyword() {
			Ok(token) 					=> return Some(token),
			Err(TokenClaimError::NoTokenFound) 	=> (),
			Err(err) 			=> panic!("{err:?}"),
		}

		match self.claim_literal() {
			Ok(token) 					=> return Some(token),
			Err(TokenClaimError::NoTokenFound) 	=> (),
			Err(err) 			=> panic!("{err:?}"),
		}

		match self.claim_identifier() {
			Ok(token) 					=> return Some(token),
			Err(TokenClaimError::NoTokenFound) 	=> (),
			Err(err)			=> panic!("{err:?}"),
		}

		// If all patterns are skipped but it is not the end of the file.
		if self.reader.fill_buf().ok()?.len() != 0 { panic!("Unknown token at byte '{}'", self.bytes_read) }

		return None
	}
}
