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
                // TODO: parse out and ignore comments using // and /**/ for comments
                ' ' | '\n' | '\t' => {
                    self.advance();
                    continue; // Skip whitespace
                },
                ';' => {
                    self.advance();
                    Token::Semicolon
                },
                _ if Enclosers::is(&c.to_string()) || Enclosers::is_fragment(&c.to_string()) => {
                    let enclosestr = self.consume_encloser();
                    let encloser = Enclosers::l_or_r(enclosestr.clone());
                    let enclosetok = Token::Encloser(encloser);
                    match Enclosers::l_or_r(enclosestr) {
                        Side::Left(item) => {
                            if Enclosers::is_literal(&item) {
                                tokens.push(enclosetok);
                                self.consume_literal(&mut tokens, item)
                            } else {
                                enclosetok
                            }
                        },
                        Side::Either(item) => {
                            if item == "\"" {
                                tokens.push(enclosetok);
                                self.consume_template(&mut tokens)
                            } else {
                                enclosetok
                            }
                        },
                        _ => enclosetok,
                    }
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
    fn consume_literal(&mut self, tokens: &mut Vec<Token>, start_encloser: String) -> Token {
        let end_encloser = start_encloser.replace("[","]");
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
        Token::Encloser(Side::Right(end_encloser))
    }
    fn consume_template(&mut self, tokens: &mut Vec<Token>) -> Token {
        // TODO:
        // Read input until non-escaped closing "
        // make the string parts into literals
        // and read the $[interpolated parts] into a string.
        // Call tokenizer on that string.
        // push resulting vec of tokens into tokens as a Tokens::Template
        // return closing " encloser
        Token::Unknown(' ')
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
