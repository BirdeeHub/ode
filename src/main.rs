mod parser;
mod runtime;
mod ioutils;
use std::time::Instant;
use std::fs::File;
use crate::ioutils::CharIterator;
use crate::parser::Parser;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
    let filepath = args.get(1).expect("No input file provided!");

    let start = Instant::now();
    // extra tokenizer print for debug purposes
    let tokenizer = Parser::new_tokenizer(CharIterator::new(File::open(filepath)?));
    for t in tokenizer {
        println!("{:?}", t);
    }
    println!("Tokenizer took: {:?}", start.elapsed());

    let start = Instant::now();
    let mut parser = Parser::new(CharIterator::new(File::open(filepath)?));
    let ast = parser.parse_program().unwrap();
    let rtvals = runtime::evaluate(&ast);
    println!("{:?}", rtvals);
    println!("Total took: {:?}", start.elapsed());

    Ok(())
}
