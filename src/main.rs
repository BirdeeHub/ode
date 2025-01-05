mod parser;
mod runtime;
mod ioutils;
use std::fs::File;
use std::io::{self, BufReader};
use crate::ioutils::CharIterator;
use crate::parser::Parser;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
    let filepath = args.get(1).expect("No input file provided!");
    let reader = BufReader::new(File::open(filepath)?);
    let reader2 = BufReader::new(File::open(filepath)?);
    // extra tokenizer print for debug purposes
    let tokenizer = Parser::new_tokenizer(CharIterator::new(reader)?);
    for t in tokenizer {
        println!("{:?}", t);
    }

    let mut parser = Parser::new(CharIterator::new(reader2)?);
    let ast = parser.parse_program().unwrap();
    let rtvals = runtime::evaluate(&ast);
    println!("{:?}", rtvals);

    Ok(())
}
