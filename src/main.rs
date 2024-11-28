use std::fs::File;
use std::io::{self, Read};
mod lexxer;
mod parser;
mod tokenizer;

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

    let mut dassda:&i32 = &0;

    dassda = &1;

    println!("{:?}", dassda);

    println!("{:?}", dassda);

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
            "=", "+", "-", "/", "%", "||", "&&", "|",
            ">>", "<<", "!", "!=", "==", "<=", ">=", "=",
            "\\", "\\:", "|>", "<-", "->",
            "=>", "!>",
            "&", "*", "`",
            "|=", "^=", "~=", "...",
            "~", "?",
            ">>>", ">>|", ">>!",
            "<@", "@", "@>", "@>>",
            ":", ",", ";",
            "#",
        ],
        enclosers: &[("(", ")"), ("[", "]"), ("{", "}"), ("<", ">"), ("#<", ">")],
        charop: "'",
        templop: "\"",
        interstart: "$[",
        interend: "]",
        escape_char: '\\',
    };
    // ` mutability op (lifetime if needed goes before, & goes after)
    // \ arg, arg -> {}
    // left (\: arg, arg -> {}) right
    // then => else !> and match ~ only
    // enum ~= constraint |= impl ^=
    // [[<T>]:`type:] [`]{}
    // while (pid @~ match) cond {}
    // <@ is value to stream/actor
    // @ is create/get stream/actor
    // @> is value from stream/actor
    // @>> untilcond, fallback TTL(int)
    // These are also used in message passing
    // >>> while >>| continue >>! break

    let mut tokenizer = tokenizer::Tokenizer::new(&contents, &settings, false);
    let tokens = tokenizer.tokenize();

    for token in &tokens {
        println!("{:?}", token);
    }

    let lexxer = lexxer::Lexxer::new(&tokens);
    let lexemes = lexxer.lex();
    for lexeme in &lexemes {
        println!("{:?}", lexeme);
    }

    //let parser = parser::Parser::new(&lexemes);
    //let tree = parser.parse();
    //println!("{:?}", tree);

    Ok(())
}
