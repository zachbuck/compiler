use macros::string_enum::generate_string_enum;


pub struct TokenizerArtifact {
	pub source_name: 	String,
	pub tokens:			Vec<Token>,
}

pub struct Token {
	pub token_type: TokenType,
	
	pub(crate) span: Span,
}

pub struct Span {
	pub(crate) start: usize,
	pub(crate) len: usize,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum TokenType {
	Punct(Punct),
	Keyword(Keyword),
	Literal(Literal),
	Ident(String),
}

generate_string_enum! {
	#[derive(Debug)]
	pub enum Punct {
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
	#[derive(Debug)]
	pub enum Keyword {
		Fn 		= "fn",
		Return 	= "return",
		Int		= "int",
	}
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Literal {
	pub literal_type: 	LiteralType,
	pub text: 			String,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum LiteralType {
	Int = 0,
}
