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
        Tokenizer { input, position: 0 }
    }

    fn get_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn advance(&mut self) {
        self.position += self.get_char().unwrap_or_default().len_utf8();
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.get_char() {
            let token = match c {
                _ if c.is_whitespace() => {
                    self.advance();
                    continue; // Skip whitespace
                }
                '0'..='9' => {
                    let number = self.consume_numeric();
                    Token::Numeric(number)
                }
                _ if Ops::is(&c.to_string()) || Ops::is_fragment(&c.to_string()) => {
                    let op = self.consume_op();
                    match op {
                        _ if op == Ops::BLOCKCOMSTART => {
                            self.advance();
                            self.advance();
                            self.consume_comment(true);
                            continue;
                        }
                        _ if op == Ops::LINECOM => {
                            self.advance();
                            self.advance();
                            self.consume_comment(false);
                            continue;
                        }
                        _ if Ops::is_literal_left(&op) => {
                            tokens.push(Token::Op(op.clone()));
                            self.consume_literal(&mut tokens, op)
                        },
                        _ if Ops::is_other_capturing(&op) => {
                            tokens.push(Token::Op(op.clone()));
                            match op.chars().next() {
                                Some(c) => self.consume_string(&mut tokens, c),
                                None => panic!("Non-literal capturing operators must be single characters"),
                            }
                        },
                        _ if Ops::is(&op) => Token::Op(op),
                        _ => Token::Identifier(op),
                    }
                }
                _ => {
                    Token::Identifier(self.consume_identifier())
                }
            };
            tokens.push(token);
        }

        tokens.push(Token::Eof); // End of file
        tokens
    }

    fn consume_comment(&mut self, block: bool) {
        while let Some(_c) = self.get_char() {
            let remaining = &self.input[self.position..];
            if remaining.starts_with(if block { Ops::BLOCKCOMEND } else { "\n" }) {
                self.advance();
                self.advance();
                break;
            }
            self.advance();
        }
    }
    fn consume_literal(&mut self, tokens: &mut Vec<Token>, start_encloser: String) -> Token {
        let end_encloser = start_encloser.replace("[", "]");
        let mut literal = String::new();
        while let Some(c) = self.get_char() {
            let remaining = &self.input[self.position..];
            if remaining.starts_with(&end_encloser) {
                let mut count = 0;
                while count < end_encloser.len() {
                    count += 1;
                    self.advance();
                }
                break;
            }
            self.advance();
            literal.push(c);
        }
        tokens.push(Token::Literal(literal));
        Token::Op(end_encloser)
    }
    fn consume_string(&mut self, tokens: &mut Vec<Token>, start_encloser: char) -> Token {
        let mut current_literal = String::new();
        let mut is_escaped = false;
        while let Some(c) = self.get_char() {
            self.advance();
            if is_escaped {
                current_literal.push(c);
                is_escaped = false;
            } else if c == '\\' {
                is_escaped = true;
            } else if c == start_encloser {
                break;
            } else {
                current_literal.push(c);
            }
        }
        tokens.push(Token::Literal(current_literal.clone()));
        Token::Op(start_encloser.to_string())
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
            if Ops::is(&c.to_string()) || Ops::is_fragment(&c.to_string()) || c.is_whitespace() {
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
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No file path provided",
        ))
    };

    let contents = contents?;

    let mut tokenizer = Tokenizer::new(&contents);
    let tokens = tokenizer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
