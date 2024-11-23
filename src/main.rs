use std::fs::File;
use std::io::{self, Read};

mod tokens;

use tokens::*;

struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            input,
            position: 0,
        }
    }

    fn get_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn advance(&mut self) {
        self.position += self.get_char().unwrap_or_default().len_utf8();
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut in_template = false;

        while let Some(c) = self.get_char() {
            let last_is_open_str: bool = match tokens.last() {
                Some(Token::Encloser(item)) => match item {
                    Side::Left(str) => {
                        Enclosers::is_literal(str)
                    },
                    Side::Either(str) => {
                        if str == "\"" {
                            in_template = true;
                            true
                        } else {
                            false
                        }
                    },
                    _ => false
                },
                _ => false,
            };
            let token = match c {
                _ if last_is_open_str && ! in_template => {
                    let literal = self.consume_literal();
                    Token::Literal(literal)
                },
                _ if last_is_open_str && in_template => {
                    let literal = self.consume_template();
                    Token::Template(literal)
                },
                ' ' | '\n' | '\t' => {
                    self.advance();
                    continue; // Skip whitespace
                },
                ';' => {
                    self.advance();
                    Token::Semicolon
                },
                _ if Enclosers::is(&c.to_string()) || Enclosers::is_fragment(&c.to_string()) => {
                    let encloser = Enclosers::l_or_r(self.consume_encloser());
                    Token::Encloser(encloser)
                },
                _ if Ops::is(&c.to_string()) || Ops::is_fragment(&c.to_string()) => {
                    let op = self.consume_op();
                    Token::Op(op)
                },
                '0'..='9' => {
                    let number = self.consume_numeric();
                    Token::Numeric(number)
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.consume_identifier();
                    match identifier.as_str() {
                        _ if Keywords::is(&identifier) => Token::Keyword(identifier),
                        _ => Token::Identifier(identifier),
                    }
                },
                _ => {
                    self.advance();
                    Token::Unknown(c)
                },
            };
            tokens.push(token);
        }

        tokens.push(Token::Eof); // End of file
        tokens
    }

    fn consume_encloser(&mut self) -> String {
        let start = self.position;
        let mut buffer = String::new();
        while let Some(c) = self.get_char() {
            buffer.push(c);
            if !Enclosers::is(buffer.as_str()) && !Enclosers::is_fragment(buffer.as_str()) {
                break;
            }
            self.advance();
        }
        self.input[start..self.position].to_string()
    }
    fn consume_literal(&mut self) -> String {
        // TODO:
        // read until closing literal encloser
    }
    fn consume_template(&mut self) -> Vec<Token> {
        // TODO:
        // Read input until non-escaped closing "
        // make the string parts into literals and make the interpolated parts into tokens
        // combine into vec of tokens in order
    }

    fn consume_op(&mut self) -> String {
        let start = self.position;
        let mut buffer = String::new();
        while let Some(c) = self.get_char() {
            buffer.push(c);
            if !(Ops::is(buffer.as_str()) || Ops::is_fragment(buffer.as_str())) {
                break;
            }
            self.advance();
        }
        self.input[start..self.position].to_string()
    }

    fn consume_numeric(&mut self) -> String {
        let start = self.position;
        let mut is_float = false;
        while let Some(c) = self.get_char() {
            if !(c.is_ascii_digit() || c == '.') || (c == '.' && is_float) {
                break;
            }
            is_float = c == '.' || is_float;
            self.advance();
        }
        self.input[start..self.position].to_string()
    }
    fn consume_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(c) = self.get_char() {
            if !(c.is_alphanumeric() || c == '_') {
                break;
            }
            self.advance();
        }
        self.input[start..self.position].to_string()
    }
}

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
        Err(io::Error::new(io::ErrorKind::InvalidInput, "No file path provided"))
    };

    let contents = contents?;

    let mut tokenizer = Tokenizer::new(&contents);
    let tokens = tokenizer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
