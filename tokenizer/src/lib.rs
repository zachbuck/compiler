
use std::io::BufRead;

mod pattern_matching;

static MAX_TOKEN_SIZE: usize = 2usize.pow(10);

pub enum Token {
	Keyword(String),
}

pub fn tokenize(reader: impl BufRead) -> Box<[Token]> {
	


	todo!();
}
