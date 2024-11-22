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

        while let Some(c) = self.get_char() {
            let token = match c {
                ' ' | '\n' | '\t' => {
                    self.advance();
                    continue; // Skip whitespace
                }
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '-' => {
                    self.advance();
                    Token::Minus
                }
                '=' => {
                    self.advance();
                    Token::Equals
                }
                '(' => {
                    self.advance();
                    Token::LParen
                }
                ')' => {
                    self.advance();
                    Token::RParen
                }
                '{' => {
                    self.advance();
                    Token::LSquirrel
                }
                '}' => {
                    self.advance();
                    Token::RSquirrel
                }
                ';' => {
                    self.advance();
                    Token::Semicolon
                }
                '0'..='9' => {
                    let number = self.consume_number();
                    Token::Number(number)
                }
                'a'..='z' | 'A'..='Z' => {
                    let identifier = self.consume_identifier();
                    match identifier.as_str() {
                        "else" => Token::Keyword(identifier),
                        _ => Token::Identifier(identifier),
                    }
                }
                _ => {
                    self.advance();
                    Token::Unknown(c)
                }
            };
            tokens.push(token);
        }

        tokens.push(Token::Eof); // End of file
        tokens
    }

    fn consume_number(&mut self) -> i32 {
        let start = self.position;
        while let Some(c) = self.get_char() {
            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }
        self.input[start..self.position].parse().unwrap_or(0)
    }

    fn consume_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(c) = self.get_char() {
            if !c.is_alphanumeric() {
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
