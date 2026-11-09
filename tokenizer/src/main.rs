
use tokenizer::tokenizer::Tokenizer;

fn main() {
	let artifact = Tokenizer::tokenize_file("./test.jonk").unwrap();
	for token in artifact.tokens {
		println!("{:?}", token.token_type);
	}
}
