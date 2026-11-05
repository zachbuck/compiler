use std::{fs::File, io::BufReader};


fn main() -> () {
	tokenizer::init();

	let file = File::open("test.txt").unwrap();
	let reader = BufReader::new(file);
	tokenizer::tokenize(reader).for_each(|token| println!("{:?}", token));
}
