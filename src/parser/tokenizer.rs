use crate::parser::ast::{Token, TokenizerSettings};

pub struct Tokenizer<'a, I>
where
    I: Iterator<Item = char>,
{
    input: I,
    peeked: Vec<char>,
    position: usize,
    ops_struct: &'a Ops<'a>,
    in_template: bool,
    out: Vec<Token>,
}
impl<'a, I> std::fmt::Debug for Tokenizer<'a, I>
where
    I: Iterator<Item = char>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tokenizer")
            .field("peeked", &self.peeked)
            .field("position", &self.position)
            .field("ops_struct", &self.ops_struct)
            .field("in_template", &self.in_template)
            .field("out", &self.out)
            .finish()
    }
}

impl<'a, I> Iterator for Tokenizer<'a, I>
where
    I: Iterator<Item = char>,
{
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.out.is_empty() {
            self.populate_next();
        };
        if !self.out.is_empty() {
            Some(self.out.remove(0))
        } else {
            None
        }
    }
}

impl<'a, I> Tokenizer<'a, I>
where
    I: Iterator<Item = char>,
{
    pub fn new(input: I, options: &'a TokenizerSettings<'a>) -> Tokenizer<'a, I> {
        let mut ret = Tokenizer {
            input,
            peeked: Vec::new(),
            position: 0,
            ops_struct: Box::leak(Box::new(Ops::new(options))),
            in_template: false,
            out: Vec::new(),
        };
        ret.populate_next();
        ret
    }

    fn new_template_tokenizer(input: I, ops_struct: &'a Ops<'a>) -> Tokenizer<'a, I> {
        let mut ret = Tokenizer {
            input,
            peeked: Vec::new(),
            position: 0,
            ops_struct,
            in_template: true,
            out: Vec::new(),
        };
        ret.populate_next();
        ret
    }
    fn at(&mut self) -> Option<char> {
        self.peek_ahead_n(0)
    }
    fn eat(&mut self) -> Option<char> {
        if self.peeked.is_empty() {
            if let Some(c) = self.input.next() {
                self.position += 1;
                Some(c)
            } else {
                None
            }
        } else {
            self.position += 1;
            Some(self.peeked.remove(0))
        }
    }
    fn remaining_starts_with(&mut self, pat: Vec<char>) -> bool {
        for (i, c) in pat.iter().enumerate() {
            if let Some(cn) = self.peek_ahead_n(i) {
                if cn != *c {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
    // NOTE: self.pos() is for adding debug info only
    fn pos(&self) -> usize {
        self.position
    }
    fn peek_ahead_n(&mut self, chars: usize) -> Option<char> {
        if self.peeked.len() <= chars {
            while chars >= self.peeked.len() {
                let Some(c) = self.input.next() else {
                    break;
                };
                self.peeked.push(c);
            }
        }
        if chars >= self.peeked.len() {
            None
        } else {
            Some(self.peeked[chars])
        }
    }

    fn populate_next(&mut self) {
        let mut tokens = Vec::new();
        let mut is_templ_literal = self.in_template;
        let mut level = 0;
        while let Some(c) = self.at() {
            let token = match c {
                _ if self.in_template && is_templ_literal => {
                    is_templ_literal = false;
                    self.consume_capturing(&mut tokens, self.ops_struct.interstart)
                }
                _ if self.ops_struct.is(&c.to_string())
                    || self.ops_struct.is_fragment(&c.to_string()) =>
                {
                    let pos = self.pos();
                    if let Some(op) = self.consume_op() {
                        match op {
                            _ if op == self.ops_struct.blockcomstart => {
                                if let Some(comment) = self.consume_comment(op, pos, true) {
                                    comment
                                } else {
                                    continue
                                }
                            }
                            _ if op == self.ops_struct.linecom => {
                                if let Some(comment) = self.consume_comment(op, pos, false) {
                                    comment
                                } else {
                                    continue
                                }
                            }
                            _ if self.in_template && self.ops_struct.is_right_encloser(&op) => {
                                if level == 0 {
                                    is_templ_literal = true;
                                } else {
                                    level -= 1;
                                }
                                Token::Op(op, pos)
                            }
                            _ if self.in_template && self.ops_struct.is_left_encloser(&op) => {
                                level += 1;
                                Token::Op(op, pos)
                            }
                            _ if self.ops_struct.is_template_op(&op) => {
                                tokens.push(Token::Op(op.clone(), pos));
                                self.consume_capturing(&mut tokens, &op)
                            }
                            _ if self.ops_struct.is_literal_left(&op) => {
                                tokens.push(Token::Op(op.clone(), pos));
                                let Some(token) = self.consume_literal(&mut tokens, &op) else {
                                    break;
                                };
                                token
                            }
                            _ => Token::Op(op, pos),
                        }
                    } else {
                        Token::Identifier(self.consume_identifier(), pos)
                    }
                }
                _ if c.is_whitespace() => {
                    self.eat();
                    continue; // Skip whitespace
                }
                '0'..='9' => {
                    let pos = self.pos();
                    self.consume_numeric(pos)
                }
                _ => {
                    let pos = self.pos();
                    Token::Identifier(self.consume_identifier(), pos)
                }
            };
            tokens.push(token);
            if !self.in_template {
                break;
            }
        }
        for token in tokens {
            self.out.push(token)
        }
    }

    fn consume_literal(&mut self, tokens: &mut Vec<Token>, start_encloser: &str) -> Option<Token> {
        let end_encloser = Ops::get_literal_end(start_encloser);
        let mut literal = String::new();
        let start = self.pos();
        let mut retval: Option<Token> = None;
        while let Some(c) = self.at() {
            if self.remaining_starts_with(end_encloser.chars().collect()) {
                retval = Some(Token::Op(end_encloser.clone(), self.pos()));
                for _ in end_encloser.chars() {
                    self.eat();
                }
                break;
            }
            self.eat();
            literal.push(c);
        }
        tokens.push(Token::Literal(literal, start));
        retval
    }
    fn consume_capturing(&mut self, tokens: &mut Vec<Token>, end_encloser: &str) -> Token {
        let mut literal = String::new();
        let start = self.pos();
        let mut is_escaped = false;
        while let Some(c) = self.at() {
            if self.remaining_starts_with(end_encloser.chars().collect()) && !is_escaped {
                break;
            }
            self.eat();
            is_escaped = c == self.ops_struct.escape_char;
            if !(is_escaped && self.remaining_starts_with(end_encloser.chars().collect())) {
                literal.push(c);
            }
        }
        let end_enc_pos = self.pos();
        for _ in end_encloser.chars() {
            self.eat();
        }
        if !self.in_template || self.at().is_some() {
            if self.ops_struct.is_template_op(end_encloser) {
                let format_tokenizer =
                    Tokenizer::new_template_tokenizer(literal.chars(), self.ops_struct);
                let mut format_tokens = Vec::new();
                for token in format_tokenizer {
                    format_tokens.push(token);
                }
                if self.at().is_some() {
                    tokens.push(Token::Format(format_tokens, start));
                    Token::Op(end_encloser.to_string(), end_enc_pos)
                } else {
                    Token::Format(format_tokens, start)
                }
            } else if self.at().is_some() {
                tokens.push(Token::Literal(literal, start));
                Token::Op(end_encloser.to_string(), end_enc_pos)
            } else {
                Token::Literal(literal, start)
            }
        } else if self.ops_struct.is_template_op(end_encloser) {
            let format_tokenizer =
                Tokenizer::new_template_tokenizer(literal.chars(), self.ops_struct);
            let mut format_tokens = Vec::new();
            for token in format_tokenizer {
                format_tokens.push(token);
            }
            Token::Format(format_tokens, start)
        } else {
            Token::Literal(literal, start)
        }
    }
    fn consume_comment(&mut self, op: String, pos:usize, block: bool) -> Option<Token> {
        let endchar = if block {
            self.ops_struct.blockcomend
        } else {
            "\n"
        };
        if self.ops_struct.capture_comments {
            let mut buffer = String::new();
            while let Some(c) = self.at() {
                buffer.push(c);
                if self.remaining_starts_with(endchar.chars().collect()) {
                    buffer.pop();
                    for c in endchar.chars() {
                        buffer.push(c);
                        self.eat();
                    }
                    break;
                }
                self.eat();
            }
            Some(Token::Comment(op + &buffer, pos))
        } else {
            while self.at().is_some() {
                if self.remaining_starts_with(endchar.chars().collect()) {
                    for _ in endchar.chars() {
                        self.eat();
                    }
                    break;
                }
                self.eat();
            }
            None
        }
    }
    fn consume_numeric(&mut self, pos:usize) -> Token {
        let mut buffer = String::new();
        let mut is_float = false;
        let mut is_hex = false;
        let mut count = 0;
        while let Some(c) = self.at() {
            if (is_float && !(c.is_ascii_digit() || c == '_'))
                || (is_hex && !(c.is_ascii_hexdigit() || c == '_'))
                || ((c == '.' && is_float) || (c == 'x' && is_hex))
                || !(is_float || is_hex || c.is_ascii_digit()
                    || c == '_' || (c == 'x' && count == 1) || c == '.')
            {
                break;
            }
            buffer.push(c);
            if count == 1 && c == 'x' {
                is_hex = true;
            }
            count += 1;
            is_float = c == '.' || is_float;
            self.eat();
        }
        if is_float {
            Token::FloatLit(buffer.replace("_", ""), pos)
        } else if is_hex {
            Token::HexLit(buffer.replace("_", ""), pos)
        } else {
            Token::IntLit(buffer.replace("_", ""), pos)
        }
    }
    fn consume_identifier(&mut self) -> String {
        let mut opbuffer = String::new();
        let mut count = 0;
        while let Some(c) = self.peek_ahead_n(count) {
            opbuffer.push(c);
            if c.is_whitespace() || self.ops_struct.is(&c.to_string()) {
                break;
            } else if self.ops_struct.is(opbuffer.as_str()) {
                count -= opbuffer.len();
                break;
            }
            if !self.ops_struct.is_fragment(opbuffer.as_str()) {
                opbuffer.clear();
            }
            count += 1;
        }
        let mut ret = String::new();
        for _ in 0..count {
            if let Some(c) = self.eat() {
                ret.push(c);
            }
        }
        ret
    }
    fn consume_op(&mut self) -> Option<String> {
        let mut buffer = String::new();
        let mut count = 0;
        while let Some(c) = self.peek_ahead_n(count) {
            buffer.push(c);
            if !(self.ops_struct.is(buffer.as_str())
                || self.ops_struct.is_fragment(buffer.as_str()))
            {
                buffer.pop();
                break;
            }
            count += 1;
        }
        if self.ops_struct.is(buffer.as_str()) {
            for _ in 0..count {
                self.eat();
            }
            Some(buffer)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Ops<'a> {
    blockcomstart: &'a str,
    blockcomend: &'a str,
    linecom: &'a str,
    interstart: &'a str,
    enclosers: &'a [(&'a str, &'a str)],
    ops: &'a [&'a str],
    charop: &'a str,
    templop: &'a str,
    escape_char: char,
    capture_comments: bool,
}

impl<'a> Ops<'a> {
    fn new(options: &'a TokenizerSettings<'a>) -> Ops<'a> {
        let mut combined_ops = vec![
            options.blockcomstart,
            options.blockcomend,
            options.linecom,
            options.interstart,
            options.interend,
            options.templop,
        ];
        combined_ops.extend_from_slice(options.ops);
        for (open, close) in options.enclosers {
            combined_ops.push(open);
            combined_ops.push(close);
        }
        let filtered_enclosers: Box<[(&'a str, &'a str)]> = options
            .enclosers
            .iter()
            .filter(|(_, close)| *close == options.interend)
            .cloned()
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Ops {
            blockcomstart: options.blockcomstart,
            blockcomend: options.blockcomend,
            linecom: options.linecom,
            ops: Box::leak(combined_ops.into_boxed_slice()),
            charop: options.charop,
            templop: options.templop,
            enclosers: Box::leak(filtered_enclosers),
            interstart: options.interstart,
            escape_char: options.escape_char,
            capture_comments: options.capture_comments,
        }
    }

    fn is(&self, op: &str) -> bool {
        self.ops.contains(&op) || self.is_literal_left(op) || self.is_literal_right(op)
    }
    fn is_fragment(&self, op: &str) -> bool {
        self.ops
            .iter()
            .any(|&op_def| op_def != op && op_def.starts_with(op))
            || Self::is_literal_left_frag(op)
            || Self::is_literal_right_frag(op)
    }

    fn is_template_op(&self, op: &str) -> bool {
        self.templop == op
    }

    fn is_left_encloser(&self, op: &str) -> bool {
        self.enclosers.iter().any(|(left, _)| *left == op)
    }

    fn is_right_encloser(&self, op: &str) -> bool {
        self.enclosers.iter().any(|(_, right)| *right == op)
    }

    fn is_literal_left(&self, op: &str) -> bool {
        self.charop == op
            || op.starts_with("[")
                && op.len() > 1
                && (op.ends_with("[") && op[1..op.len() - 1].chars().all(|c| c == '='))
    }
    fn get_literal_end(left_lit_op: &str) -> String {
        left_lit_op.replace("[", "]")
    }
    fn is_literal_right(&self, op: &str) -> bool {
        self.charop == op
            || op.starts_with("]")
                && op.len() > 1
                && (op.ends_with("]") && op[1..op.len() - 1].chars().all(|c| c == '='))
    }

    fn is_literal_left_frag(op: &str) -> bool {
        op == "[" || op.starts_with("[") && op.len() > 1 && op[1..].chars().all(|c| c == '=')
    }
    fn is_literal_right_frag(op: &str) -> bool {
        op == "]" || op.starts_with("]") && op.len() > 1 && op[1..].chars().all(|c| c == '=')
    }
}
