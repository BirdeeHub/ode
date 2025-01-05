use std::fs::File;
use std::io::{self, Read};
mod parser;
mod runtime;
use crate::parser::Parser;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
    let inputvar = args.get(1).expect("No input file provided!");
    let contents = read_file(inputvar)?;

    // extra tokenizer print for debug purposes
    let tokenizer = Parser::new_tokenizer(contents.chars());
    for t in tokenizer {
        println!("{:?}", t);
    }

    let mut parser = Parser::new(contents.chars());
    let ast = parser.parse_program().unwrap();
    let rtvals = runtime::evaluate(&ast);
    println!("{:?}", rtvals);

    Ok(())
}
