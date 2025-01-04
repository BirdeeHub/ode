use crate::parser::parser_types::{ Coin, Token };

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    ops_struct: Ops<'a>,
    in_template: bool,
    options: &'a TokenizerSettings<'a>,
    out: Vec<Token>,
    outpos: usize,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.has_next();
        let ret = self.at();
        self.skip();
        ret
    }
}

impl<'a> Tokenizer<'a> {
    fn new_template_tokenizer(
        input: &'a str,
        options: &'a TokenizerSettings<'a>,
    ) -> Tokenizer<'a> {
        let mut ret = Tokenizer {
            input,
            position: 0,
            ops_struct: Ops::new(options),
            in_template: true,
            options,
            out: Vec::new(),
            outpos: 0,
        };
        ret.populate_next();
        ret
    }
    pub fn new(
        input: &'a str,
        options: &'a TokenizerSettings<'a>,
    ) -> Tokenizer<'a> {
        let mut ret = Tokenizer {
            input,
            position: 0,
            ops_struct: Ops::new(options),
            in_template: false,
            options,
            out: Vec::new(),
            outpos: 0,
        };
        ret.populate_next();
        ret
    }

    pub fn at(&self) -> Option<Token> {
        self.out.get(self.outpos).cloned()
    }

    pub fn skip(&mut self) {
        if self.has_next() {
            self.outpos += 1;
        }
    }

    pub fn has_next(&mut self) -> bool {
        self.outpos < self.out.len() || (self.outpos + 1 >= self.out.len() && self.populate_next())
    }

    fn get_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn advance(&mut self) {
        self.position += self.get_char().unwrap_or_default().len_utf8();
    }

    fn remaining_starts_with(&mut self, pat: &str) -> bool {
        self.input[self.position..].starts_with(pat)
    }

    pub fn populate_next(&mut self) -> bool {
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
                    if let Some(op) = self.consume_op() {
                        match op {
                            _ if op == self.ops_struct.blockcomstart => {
                                self.consume_comment(true);
                                continue;
                            }
                            _ if op == self.ops_struct.linecom => {
                                self.consume_comment(false);
                                continue;
                            }
                            _ if self.in_template && self.ops_struct.is_right_encloser(&op)
                                || self.ops_struct.interend == op => {
                                if level == 0 {
                                    is_templ_literal = true;
                                } else {
                                    level -= 1;
                                }
                                Token::Op(Coin::new(op.clone(),pos))
                            }
                            _ if self.in_template && self.ops_struct.is_left_encloser(&op)
                                || self.ops_struct.interstart == op => {
                                level += 1;
                                Token::Op(Coin::new(op.clone(),pos))
                            }
                            _ if self.ops_struct.is_template_op(&op) => {
                                tokens.push(Token::Op(Coin::new(op.clone(),pos)));
                                self.consume_capturing(&mut tokens, &op)
                            }
                            _ if self.ops_struct.is_literal_left(&op) => {
                                tokens.push(Token::Op(Coin::new(op.clone(),pos)));
                                let Some(op) = self.consume_literal(&mut tokens, &op) else {
                                    break;
                                };
                                op
                            }
                            _ if self.ops_struct.is(&op) => Token::Op(Coin::new(op.clone(),pos)),
                            _ => Token::Identifier(Coin::new(op.clone(),pos)),
                        }
                    } else {
                        Token::Op(Coin::new(self.consume_identifier(),pos))
                    }
                }
                _ if c.is_whitespace() => {
                    self.advance();
                    continue; // Skip whitespace
                }
                '0'..='9' => Token::Numeric(Coin::new(self.consume_numeric(), self.position)),
                _ => Token::Identifier(Coin::new(self.consume_identifier(), self.position)),
            };
            tokens.push(token);
            if ! self.in_template {
                break;
            }
        }
        let ret = ! tokens.is_empty();
        for token in tokens {
            self.out.push(token)
        }
        ret
    }

    fn consume_literal(&mut self, tokens: &mut Vec<Token>, start_encloser: &str) -> Option<Token> {
        let end_encloser = Ops::get_literal_end(start_encloser);
        let start = self.position;
        let mut literal = String::new();
        while let Some(c) = self.get_char() {
            if self.remaining_starts_with(&end_encloser) {
                let mut count = 0;
                while count < end_encloser.len() {
                    count += 1;
                    self.advance();
                }
                break;
            }
            self.advance();
            literal.push(c);
            if self.get_char().is_none() {
                self.position = start;
                return None;
            }
        }
        tokens.push(Token::Literal(Coin::new(literal, self.position)));
        Some(Token::Op(Coin::new(end_encloser, self.position)))
    }
    fn consume_capturing(&mut self, tokens: &mut Vec<Token>, end_encloser: &str) -> Token {
        let mut literal = String::new();
        let start = self.position;
        let mut is_escaped = false;
        while let Some(c) = self.get_char() {
            if self.remaining_starts_with(end_encloser) && !is_escaped {
                let mut count = 0;
                while count < end_encloser.len() {
                    count += 1;
                    self.advance();
                }
                break;
            }
            self.advance();
            is_escaped = c == self.ops_struct.escape_char;
            if is_escaped && self.remaining_starts_with(end_encloser) {
            } else {
                literal.push(c);
            }
        }
        if !self.in_template || self.get_char().is_some() {
            if self.ops_struct.is_template_op(end_encloser) {
                let format_tokenizer = Tokenizer::new_template_tokenizer(&literal, self.options);
                let mut format_tokens = Vec::new();
                for token in format_tokenizer {
                    format_tokens.push(token);
                }
                tokens.push(Token::Format(Coin::new(format_tokens, start)));
            } else {
                tokens.push(Token::Literal(Coin::new(literal, start)));
            }
            Token::Op(Coin::new(end_encloser.to_string(), self.position))
        } else if self.ops_struct.is_template_op(end_encloser) {
            let format_tokenizer = Tokenizer::new_template_tokenizer(&literal, self.options);
            let mut format_tokens = Vec::new();
            for token in format_tokenizer {
                format_tokens.push(token);
            }
            Token::Format(Coin::new(format_tokens, start))
        } else {
            Token::Literal(Coin::new(literal, start))
        }
    }
    fn consume_comment(&mut self, block: bool) {
        let endchar = if block {
            self.ops_struct.blockcomend
        } else {
            "\n"
        };
        while let Some(_c) = self.get_char() {
            if self.remaining_starts_with(endchar) {
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
        let mut buffer = String::new();
        while let Some(c) = self.get_char() {
            buffer.push(c);
            if c.is_whitespace() || self.ops_struct.is(&c.to_string()) {
                break;
            } else if self.ops_struct.is(buffer.as_str()) {
                self.position -= buffer.len();
                break;
            }
            if ! self.ops_struct.is_fragment(buffer.as_str()) {
                buffer.clear();
            }
            self.advance();
        }
        self.input[start..self.position].to_string()
    }
    fn consume_op(&mut self) -> Option<String> {
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
        if !self.ops_struct.is(&self.input[start..self.position]) {
            self.position = start;
            return None;
        }
        Some(self.input[start..self.position].to_string())
    }
}

#[derive(Debug)]
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
        combined_ops.push(options.interstart);
        combined_ops.push(options.interend);
        combined_ops.push(options.templop);
        combined_ops.push(options.charop);
        combined_ops.push(options.linecom);
        combined_ops.push(options.blockcomstart);
        combined_ops.push(options.blockcomend);
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
        self.ops.contains(&op) || self.is_literal_left(op) || self.is_literal_right(op)
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

    fn is_left_encloser(&self, op: &str) -> bool {
        self.enclosers.iter().any(|(left, _)| *left == op)
    }

    fn is_right_encloser(&self, op: &str) -> bool {
        self.enclosers.iter().any(|(right, _)| *right == op)
    }

    fn is_literal_left(&self, op: &str) -> bool {
        self.charop == op || op.starts_with("[")
            && op.len() > 1
            && (op.ends_with("[") && op[1..op.len() - 1].chars().all(|c| c == '='))
    }
    fn get_literal_end(left_lit_op: &str) -> String {
        left_lit_op.replace("[", "]")
    }
    fn is_literal_right(&self, op: &str) -> bool {
        self.charop == op || op.starts_with("]")
            && op.len() > 1
            && (op.ends_with("]") && op[1..op.len() - 1].chars().all(|c| c == '='))
    }
}

fn is_literal_left_frag(op: &str) -> bool {
    op == "[" || op.starts_with("[") && op.len() > 1 && op[1..].chars().all(|c| c == '=')
}
fn is_literal_right_frag(op: &str) -> bool {
    op == "]" || op.starts_with("]") && op.len() > 1 && op[1..].chars().all(|c| c == '=')
}

