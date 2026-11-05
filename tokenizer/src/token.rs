
//! This contains output types for the token stream returned by this library.

#[derive(Clone)]
#[derive(Debug)]
pub enum Token {
	Punctuation(Punctuation),
	Keyword(Keyword),
	Literal(String),
	Identifier(String),
}

impl Token {
	/// Returns the number of bytes in the bytestream that were consumed by the token
	pub fn get_text(&self) -> &str {
		match self {
			Token::Punctuation(internal) 	=> internal.as_str(),
			Token::Keyword(internal) 			=> internal.as_str(),
			Token::Literal(internal) 			=> internal,
			Token::Identifier(internal) 		=> internal,
		}
	}
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum Punctuation {
	// Seperator characters
	Period,				// '.'
	Comma,				// ','
	Colon,				// ':'
	Semicolon,			// ';'
	Equal,				// '='
	OpenParenthesis,	// '('
	CloseParenthesis,	// ')'
	OpenBracket,		// '['
	CloseBracket,		// ']'
	OpenBrace,			// '{'
	CloseBrace,			// '}'
	Arrow,				// '->'
	Minus,				// '-'
	Plus,				// '+'
	Asterisk,			// '*'
	Slash,				// '/'
	DoubleAsterisk,		// '**'
	Tilde,				// '~'
	And,				// '&'
	Pipe,				// '|'
	Caret,				// '^'
	DoubleLessThan,		// '<<'
	DoubleGreaterThan,	// '>>'
	Exclamation,		// '!'
	DoubleAnd,			// '&&'
	DoublePipe,			// '||'
	DoubleCaret,		// '^^' 
	DoubleEqual,		// '=='
	ExclaimEqual,		// '!='
	GreaterThan,		// '>'
	GreaterEqual,		// '>='
	LessThan,			// '<'
	LessEqual,			// '<='
}

impl Punctuation {
	pub fn as_str(&self) -> &str {
		match self {
			Punctuation::Period 			=> ".",
			Punctuation::Comma 				=> ",",
			Punctuation::Colon				=> ":",
			Punctuation::Semicolon 			=> ";",
			Punctuation::Equal 				=> "=",
			Punctuation::OpenParenthesis 	=> "(",
			Punctuation::CloseParenthesis 	=> ")",
			Punctuation::OpenBracket 		=> "{",
			Punctuation::CloseBracket 		=> "}",
			Punctuation::OpenBrace 			=> "[",
			Punctuation::CloseBrace 		=> "]",
			Punctuation::Arrow 				=> "->",
			Punctuation::Minus 				=> "-",
			Punctuation::Plus 				=> "+",
			Punctuation::Asterisk 			=> "*",
			Punctuation::Slash 				=> "/",
			Punctuation::DoubleAsterisk 	=> "**",
			Punctuation::Tilde 				=> "~",
			Punctuation::And 				=> "&",
			Punctuation::Pipe 				=> "|",
			Punctuation::Caret 				=> "^",
			Punctuation::DoubleLessThan 	=> "<<",
			Punctuation::DoubleGreaterThan 	=> ">>",
			Punctuation::Exclamation 		=> "!",
			Punctuation::DoubleAnd 			=> "&&",
			Punctuation::DoublePipe 		=> "||",
			Punctuation::DoubleCaret 		=> "^^",
			Punctuation::DoubleEqual 		=> "==",
			Punctuation::ExclaimEqual 		=> "!=",
			Punctuation::GreaterThan 		=> ">",
			Punctuation::GreaterEqual 		=> ">=",
			Punctuation::LessThan 			=> "<",
			Punctuation::LessEqual 			=> "<=",
		}
	}
}

impl Into<Token> for Punctuation {
	fn into(self) -> Token { Token::Punctuation(self) }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum Keyword {
	Fn,			// 'fn'
	Return,		// 'return'
	Int,		// 'int'
}

impl Keyword {
	pub fn as_str(&self) -> &str {
		match self {
			Keyword::Fn 	=> "fn",
			Keyword::Return => "return",
			Keyword::Int 	=> "int",
		}
	}
}

impl Into<Token> for Keyword {
	fn into(self) -> Token { Token::Keyword(self) }
}
