
use tokenizer::tokenizer::Tokenizer;

fn main() {
	let artifact = Tokenizer::tokenize_file("./test.jonk").unwrap();
	for token in artifact.get_tokens() {
		println!("{:?}", token);
	}
}
