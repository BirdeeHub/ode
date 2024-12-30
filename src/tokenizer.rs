#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(Coin<String>),
    Op(Coin<String>),
    Numeric(Coin<String>), // int or float in string form
    Literal(Coin<String>),
    Format(Coin<Vec<Token>>),
    Eof,
}

#[derive(Debug, Clone)]
pub struct Coin<T> {
    pub val: T,
    pub pos: usize,
}

impl<T: PartialEq> PartialEq for Coin<T> {
    fn eq(&self, other: &Coin<T>) -> bool {
        self.val == other.val
    }
}

fn is_literal_left_frag(op: &str) -> bool {
    op == "[" || op.starts_with("[") && op.len() > 1 && op[1..].chars().all(|c| c == '=')
}
fn is_literal_right_frag(op: &str) -> bool {
    op == "]" || op.starts_with("]") && op.len() > 1 && op[1..].chars().all(|c| c == '=')
}

struct Ops<'a> {
    blockcomstart: &'a str,
    blockcomend: &'a str,
    linecom: &'a str,
    interstart: &'a str,
    interend: &'a str,
    enclosers: Vec<(&'a str, &'a str)>,
    ops: &'a [&'a str],
    charop: &'a str,
    templop: &'a str,
    escape_char: char,
}

pub struct TokenizerSettings<'a> {
    pub blockcomstart: &'a str,
    pub blockcomend: &'a str,
    pub linecom: &'a str,
    pub ops: &'a [&'a str],
    pub charop: &'a str,
    pub templop: &'a str,
    pub enclosers: &'a [(&'a str, &'a str)],
    pub interstart: &'a str,
    pub interend: &'a str,
    pub escape_char: char,
}

impl<'a> Ops<'a> {
    fn new(options: &'a TokenizerSettings<'a>) -> Ops<'a> {
        let mut combined_ops = vec![
            options.blockcomstart,
            options.blockcomend,
            options.linecom,
            options.interstart,
            options.interend,
            options.charop,
            options.templop,
        ];
        combined_ops.extend_from_slice(options.ops);
        for (open, close) in options.enclosers {
            combined_ops.push(open);
            combined_ops.push(close);
        }
        let filtered_enclosers = options
            .enclosers
            .iter()
            .filter(|(_, close)| *close == options.interend)
            .cloned()
            .collect();

        // Convert Vec to slice for the new Ops struct
        let combined_ops: &'a [&'a str] = Box::leak(combined_ops.into_boxed_slice());

        Ops {
            blockcomstart: options.blockcomstart,
            blockcomend: options.blockcomend,
            linecom: options.linecom,
            ops: combined_ops,
            charop: options.charop,
            templop: options.templop,
            enclosers: filtered_enclosers,
            interstart: options.interstart,
            interend: options.interend,
            escape_char: options.escape_char,
        }
    }

    fn is(&self, op: &str) -> bool {
        self.ops.contains(&op) || Ops::is_literal_left(op) || Ops::is_literal_right(op)
    }
    fn is_fragment(&self, op: &str) -> bool {
        self.ops
            .iter()
            .any(|&op_def| op_def != op && op_def.starts_with(op))
            || is_literal_left_frag(op)
            || is_literal_right_frag(op)
    }

    fn is_template_op(&self, op: &str) -> bool {
        self.templop == op
    }

    fn is_other_capturing(&self, op: &str) -> bool {
        self.templop == op || self.charop == op
    }

    fn is_left_encloser(&self, op: &str) -> bool {
        self.enclosers.iter().any(|(left, _)| *left == op)
    }

    fn is_right_encloser(&self, op: &str) -> bool {
        self.enclosers.iter().any(|(right, _)| *right == op)
    }

    fn is_literal_left(op: &str) -> bool {
        op.starts_with("[")
            && op.len() > 1
            && (op.ends_with("[") && op[1..op.len() - 1].chars().all(|c| c == '='))
    }
    fn get_literal_left(left_lit_op: &str) -> String {
        left_lit_op.replace("[", "]")
    }
    fn is_literal_right(op: &str) -> bool {
        op.starts_with("]")
            && op.len() > 1
            && (op.ends_with("]") && op[1..op.len() - 1].chars().all(|c| c == '='))
    }
}

pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    ops_struct: Ops<'a>,
    in_template: bool,
    options: &'a TokenizerSettings<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(
        input: &'a str,
        options: &'a TokenizerSettings<'a>,
        in_template: bool,
    ) -> Tokenizer<'a> {
        Tokenizer {
            input,
            position: 0,
            ops_struct: Ops::new(options),
            in_template,
            options,
        }
    }

    fn get_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn advance(&mut self) {
        self.position += self.get_char().unwrap_or_default().len_utf8();
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut is_templ_literal = self.in_template;
        let mut level = 0;
        while let Some(c) = self.get_char() {
            let token = match c {
                _ if self.in_template && is_templ_literal => {
                    is_templ_literal = false;
                    self.consume_capturing(&mut tokens, self.ops_struct.interstart)
                }
                _ if self.ops_struct.is(&c.to_string())
                    || self.ops_struct.is_fragment(&c.to_string()) =>
                {
                    let pos = self.position;
                    let op = self.consume_op();
                    match op {
                        _ if op == self.ops_struct.blockcomstart => {
                            self.consume_comment(true);
                            continue;
                        }
                        _ if op == self.ops_struct.linecom => {
                            self.consume_comment(false);
                            continue;
                        }
                        _ if Ops::is_literal_left(&op) => {
                            tokens.push(Token::Op(Coin {
                                val: op.clone(),
                                pos,
                            }));
                            self.consume_literal(&mut tokens, &op)
                        }
                        _ if self.ops_struct.is_other_capturing(&op) => {
                            tokens.push(Token::Op(Coin {
                                val: op.clone(),
                                pos,
                            }));
                            self.consume_capturing(&mut tokens, &op)
                        }
                        _ if self.in_template && self.ops_struct.is_right_encloser(&op)
                            || self.ops_struct.interend == op =>
                        {
                            if level == 0 {
                                is_templ_literal = true;
                            } else {
                                level -= 1;
                            }
                            Token::Op(Coin {
                                val: op,
                                pos,
                            })
                        }
                        _ if self.in_template && self.ops_struct.is_left_encloser(&op) => {
                            level += 1;
                            Token::Op(Coin {
                                val: op,
                                pos,
                            })
                        }
                        _ if self.ops_struct.is(&op) => Token::Op(Coin {
                            val: op,
                            pos,
                        }),
                        _ => Token::Identifier(Coin {
                            val: op,
                            pos,
                        }),
                    }
                }
                _ if c.is_whitespace() => {
                    self.advance();
                    continue; // Skip whitespace
                }
                '0'..='9' => Token::Numeric(Coin {
                    pos: self.position,
                    val: self.consume_numeric(),
                }),
                _ => Token::Identifier(Coin {
                    pos: self.position,
                    val: self.consume_identifier(),
                }),
            };
            tokens.push(token);
        }
        tokens.push(Token::Eof);
        tokens
    }

    fn consume_comment(&mut self, block: bool) {
        let endchar = if block {
            self.ops_struct.blockcomend
        } else {
            "\n"
        };
        while let Some(_c) = self.get_char() {
            let remaining = &self.input[self.position..];
            if remaining.starts_with(endchar) {
                let mut count = 0;
                while count < endchar.len() {
                    self.advance();
                    count += 1;
                }
                break;
            }
            self.advance();
        }
    }
    fn consume_literal(&mut self, tokens: &mut Vec<Token>, start_encloser: &str) -> Token {
        let end_encloser = Ops::get_literal_left(start_encloser);
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
        tokens.push(Token::Literal(Coin {
            val: literal,
            pos: self.position,
        }));
        Token::Op(Coin {
            val: end_encloser,
            pos: self.position,
        })
    }
    fn consume_capturing(&mut self, tokens: &mut Vec<Token>, end_encloser: &str) -> Token {
        let mut literal = String::new();
        let start = self.position;
        let mut is_escaped = false;
        while let Some(c) = self.get_char() {
            let remaining = &self.input[self.position..];
            if remaining.starts_with(end_encloser) && !is_escaped {
                let mut count = 0;
                while count < end_encloser.len() {
                    count += 1;
                    self.advance();
                }
                break;
            }
            self.advance();
            is_escaped = c == self.ops_struct.escape_char;
            if is_escaped && self.input[self.position..].starts_with(end_encloser) {
            } else {
                literal.push(c);
            }
        }
        if !self.in_template || self.get_char().is_some() {
            if self.ops_struct.is_template_op(end_encloser) {
                let format_tokens = Tokenizer::new(&literal, self.options, true).tokenize();
                tokens.push(Token::Format(Coin {
                    val: format_tokens,
                    pos: start,
                }));
            } else {
                tokens.push(Token::Literal(Coin {
                    val: literal,
                    pos: start,
                }));
            }
            Token::Op(Coin {
                val: end_encloser.to_string(),
                pos: self.position,
            })
        } else if self.ops_struct.is_template_op(end_encloser) {
            let format_tokens = Tokenizer::new(&literal, self.options, true).tokenize();
            Token::Format(Coin {
                val: format_tokens,
                pos: start,
            })
        } else {
            Token::Literal(Coin {
                val: literal,
                pos: start,
            })
        }
    }
    fn consume_op(&mut self) -> String {
        let start = self.position;
        let mut buffer = String::new();
        while let Some(c) = self.get_char() {
            buffer.push(c);
            if !(self.ops_struct.is(buffer.as_str())
                || self.ops_struct.is_fragment(buffer.as_str()))
            {
                break;
            }
            self.advance();
        }
        self.input[start..self.position].to_string()
    }
    fn consume_numeric(&mut self) -> String {
        let start = self.position;
        let mut is_float = false;
        let mut is_hex = false;
        let mut count = 0;
        while let Some(c) = self.get_char() {
            if (is_float && !c.is_ascii_digit()) || (is_hex && !c.is_ascii_hexdigit())
                || ((c == '.' || c == 'x') && (is_float || is_hex))
                || !(is_float || is_hex || c.is_ascii_digit() || (c == 'x' && count == 1) || c == '.')
            {
                break;
            }
            if count == 1 && c == 'x' {
                is_hex = true;
            }
            count += 1;
            is_float = c == '.' || is_float;
            self.advance();
        }
        self.input[start..self.position].to_string()
    }
    fn consume_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(c) = self.get_char() {
            if self.ops_struct.is(&c.to_string())
                || self.ops_struct.is_fragment(&c.to_string())
                || c.is_whitespace()
            {
                break;
            }
            self.advance();
        }
        self.input[start..self.position].to_string()
    }
}
