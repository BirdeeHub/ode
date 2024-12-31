use std::fs::File;
use std::io::{self, Read};
mod parser;
mod runtime;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
    let contents = if args.len() > 1 {
        read_file(&args[1])
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No file path provided",
        ))
    }?;
    println!("Contents: {:?}", contents);
    let mut parser = parser::Parser::new(&contents);
    let ast = parser.parse_program().unwrap();
    let rtvals = runtime::evaluate(&ast);
    println!("{:?}", rtvals);

    Ok(())
}
