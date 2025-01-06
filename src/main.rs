mod parser;
mod runtime;
mod ioutils;
use crate::ioutils::CharIterator;
use crate::parser::Parser;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
    let filepath = args.get(1).expect("No input file provided!");

    // extra tokenizer print for debug purposes
    let tokenizer = Parser::new_tokenizer(CharIterator::new(ioutils::mk_buf_reader(filepath)?));
    for t in tokenizer {
        println!("{:?}", t);
    }

    let mut parser = Parser::new(CharIterator::new(ioutils::mk_buf_reader(filepath)?));
    let ast = parser.parse_program().unwrap();
    let rtvals = runtime::evaluate(&ast);
    println!("{:?}", rtvals);

    Ok(())
}
