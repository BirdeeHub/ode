use std::fs::File;
use std::io::{self, Read};
mod tokenizer;
mod parser;

fn read_file(file_path: &str) -> io::Result<String> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Create a string buffer to store the file contents
    let mut contents = String::new();

    // Read the file into the string buffer
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Print all arguments
    println!("Arguments: {:?}", args);

    let contents = if args.len() > 1 {
        read_file(&args[1])
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No file path provided",
        ))
    };

    let contents = contents?;

    let settings = tokenizer::TokenizerSettings {
        blockcomstart: "/*",
        blockcomend: "*/",
        linecom: "//",
        ops: &[
            "=", "+", "-", "*", "/", "%", ";", "|",
        ],
        enclosers: &[("(", ")")],
        charop: "'",
        templop: "\"",
        interstart: "$[",
        interend: "]",
        escape_char: '\\',
    };

    let mut tokenizer = tokenizer::Tokenizer::new(&contents, &settings, false);
    let tokens = tokenizer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }

    //let parser_obj = parser::Parser::new(tokens);
    //let tree = parser_obj.parse();
    //println!("{:?}", tree);

    Ok(())
}
