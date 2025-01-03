mod tokenizer;
pub mod parser_types;
use crate::parser::parser_types::*;
use crate::parser::tokenizer::Tokenizer;

#[derive(Debug, PartialEq)]
pub struct Parser<'a> {
    input_string: &'a str,
    in_tokens: Vec<Token>,
    position: usize,
}
impl<'a> Parser<'a> {
    pub fn new(input_string:&'a str) -> Parser<'a> {
        let settings = tokenizer::TokenizerSettings {
            blockcomstart: "#^",
            blockcomend: "#$",
            linecom: "#",
            ops: &[
                "=", "+", "-", "/", "%", "//", "|",
                ">>", "<<", "!", "||", "&&",
                "!=", "==", "<=", ">=",
                "-=", "+=", "*=", "/=", "&=", "|=", "%=", "//=",
                "\\", "\\:", "...", "->", "<-", ">>=", "|>", "<|", "?",
                "`", "&", "*", "\\&",
                "=>", "!>", "~",
                "_=", "^=", "~=",
                ">>>", ">>|", ">>!",
                "<@", "@", "@@", "@>", "@>>",
                ":", ".", ",", ";",
            ],
            enclosers: &[("(", ")"), ("[", "]"), ("{", "}"), ("<", ">"), ("#<", ">"), ("#!", "#@")],
            charop: "'",
            templop: "\"",
            interstart: "$[",
            interend: "]",
            escape_char: '\\',
        };
        /*
        ` mutability op (lifetime if needed goes before, & goes after)
        left (\: arg, arg -> {}) right
        then => else !> and match ~ only
        enum ~= constraint |= impl ^=
        [[<T>]:`type:] [`]{}
        <@ is value to stream/actor
        @ is open/run stream/actor on node
        @@ is same but on current node
        @> is value from stream/actor
        @>> untilcond, fallback TTL(int)
        These are also used in message passing
        >>> while >>| continue >>! break

        :name = 5;
        `int:name2 = 6;

        \ ::arg, ::arg -> {}

        "#!" "#@" <- node config enclosers
        doubles as shebang for interpreted mode
        */

        let tokenizer = Tokenizer::new(input_string, &settings);
        let mut in_tokens = Vec::new();
        let mut position = 0;
        for token in tokenizer {
            println!("{token:?}");
            in_tokens.push(token);
            position += 1;
        }
        println!("tokens {}", in_tokens.len());
        println!("position {}", position);
        Parser{ in_tokens, input_string, position: 0, }
    }
    fn at(&self) -> Option<&Token> {
        self.in_tokens.get(self.position)
    }
    fn eat(&mut self) -> Option<&Token> {
        let res = self.in_tokens.get(self.position);
        self.position += 1;
        res
    }
    fn skip(&mut self) {
        self.position += 1;
    }
    fn prev(&self) -> Option<&Token> {
        self.in_tokens.get(self.position-1)
    }
    fn not_eof(&self) -> bool {
        ! matches!(self.at(), Some(Token::Eof) | None)
    }

    pub fn parse_program(&mut self) -> ParseResult {
        let mut program = Vec::new();
        while self.not_eof() {
            program.push(self.parse_stmt()?.into());
        }
        Ok(Stmt::Module { body: program, ttype: Lexeme::Module})
    }

    pub fn parse_stmt(&mut self) -> ParseResult {
        self.parse_expr()
    }
    pub fn parse_expr(&mut self) -> ParseResult {
        self.parse_binary_expr()
    }
    pub fn parse_binary_expr(&mut self) -> ParseResult {
        self.parse_additive_expr()
    }
    pub fn parse_additive_expr(&mut self) -> ParseResult {
        let mut left = self.parse_multiplicative_expr()?;
        while let Some(Token::Op(coin)) = self.at() {
            let coin = coin.clone();
            if !matches!(coin.val.as_str(), "+" | "-") {
                break;
            }
            self.skip();
            let ttype = match coin.val.as_str() {
                "+" => Lexeme::Add,
                _ => Lexeme::Sub,
            };
            let right = self.parse_multiplicative_expr()?;
            left = Stmt::BinaryExpr{ ttype,coin,l:left.into(),r:right.into()};
        }
        Ok(left)
    }
    pub fn parse_multiplicative_expr(&mut self) -> ParseResult {
        let mut left = self.parse_primary_expr()?;
        while let Some(Token::Op(coin)) = self.at() {
            let coin = coin.clone();
            if !matches!(coin.val.as_str(), "*" | "/" | "%") {
                break;
            }
            self.skip();
            let ttype = match coin.val.as_str() {
                "*" => Lexeme::Mult,
                "/" => Lexeme::Div,
                _ => Lexeme::Mod,
            };
            let right = self.parse_primary_expr()?;
            left = Stmt::BinaryExpr{ ttype,coin,l:left.into(),r:right.into()};
        }
        Ok(left)
    }
    pub fn parse_primary_expr(&mut self) -> ParseResult {
        match self.at() {
            Some(Token::Identifier(_)) => self.parse_ident(),
            Some(Token::Numeric(_)) => self.parse_numeric(),
            Some(Token::Op(coin)) if coin.val.as_str() == "(" => {
                let coin = coin.clone();
                self.skip();
                let val = self.parse_expr();
                match self.eat() {
                    Some(Token::Op(c)) if c.val.as_str() == ")" => {},
                    _ => {
                        return Err(ParseError::UnmatchedEncloser(Token::Op(coin)))
                    },
                }
                val
            },
            _ => Err(ParseError::InvalidExpression(self.at().unwrap_or(&Token::Eof).clone())),
        }
    }
    pub fn parse_ident(&mut self) -> ParseResult {
        let Some(Token::Identifier(coin)) = self.eat() else { return Err(ParseError::InvalidIdent(self.prev().unwrap_or(&Token::Eof).clone())) };
        Ok(Stmt::Identifier{ ttype:Lexeme::Ident,coin:coin.clone(),val:coin.val.clone().into()})
    }
    pub fn parse_numeric(&mut self) -> ParseResult {
        // coin is coin.val (which is a string) and coin.pos
        // check if the string parses to float it or hex
        let Some(Token::Numeric(coin)) = self.eat() else { return Err(ParseError::InvalidNumber(self.prev().unwrap_or(&Token::Eof).clone())) };
        let value = &coin.val; // Assuming `coin.val` is the string representation of the number.
        if let Ok(val) = value.parse::<u64>() {
            Ok(Stmt::IntLiteral{ ttype:Lexeme::Int,coin:coin.clone(),val})
        } else if let Ok(val) = value.parse::<f64>() {
            Ok(Stmt::FloatLiteral{ ttype:Lexeme::Float,coin:coin.clone(),val})
        } else if let Some(stripped) = value.strip_prefix("0x") {
            let Ok(val) = u64::from_str_radix(stripped, 16) else {
                return Err(ParseError::InvalidNumber(Token::Numeric(coin.clone())))
            };
            Ok(Stmt::IntLiteral{ ttype:Lexeme::Int,coin:coin.clone(),val})
        } else {
            Err(ParseError::InvalidNumber(Token::Numeric(coin.clone())))
        }
    }
}
