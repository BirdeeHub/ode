use std::fs::File;
use std::time::Instant;
use std::io::{self, Read};
mod parser;
mod runtime;
use crate::parser::Parser;

struct CharIterator {
    reader: File,
}

impl CharIterator {
    fn new(path: &str) -> io::Result<Self> {
        let reader = File::open(path)?;
        Ok(CharIterator { reader })
    }
}

//TODO: figure out how to actually make this get chars and not 1 byte
impl Iterator for CharIterator {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0; 1];
        match self.reader.read(&mut buffer) {
            Ok(0) => None,
            Ok(_) => {
                char::from_u32(buffer[0] as u32).map(Some).unwrap_or(None)
            }
            Err(_) => None,
        }
    }
}

fn main() -> io::Result<()> {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
    let inputvar = args.get(1).expect("No input file provided!");
    let contents = CharIterator::new(inputvar)?;
    let mut parser = Parser::new(contents);
    let ast = parser.parse_program().unwrap();
    let rtvals = runtime::evaluate(&ast);
    println!("{:?}", rtvals);
    println!("Time: {:?}", start.elapsed());

    Ok(())
}
