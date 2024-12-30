mod tokenizer;
mod types;
use crate::parser::tokenizer::{Tokenizer,TokenizerSettings,Token,Coin};
use crate::parser::types::*;

#[derive(Debug, PartialEq)]
pub struct Parser<'a> {
    input_string: &'a str,
    in_tokens: Vec<Token>,
    position: usize,
}
impl<'a> Parser<'a> {
    pub fn new(input_string:&'a str) -> Parser<'a> {
        let settings = TokenizerSettings {
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

        // ` mutability op (lifetime if needed goes before, & goes after)
        // \ arg, arg -> {}
        // left (\: arg, arg -> {}) right
        // then => else !> and match ~ only
        // enum ~= constraint |= impl ^=
        // [[<T>]:`type:] [`]{}
        // <@ is value to stream/actor
        // @ is open/run stream/actor on node
        // @@ is same but on current node
        // @> is value from stream/actor
        // @>> untilcond, fallback TTL(int)
        // These are also used in message passing
        // >>> while >>| continue >>! break

        // "#!" "#@" <- node config enclosers
        // doubles as shebang for interpreted mode

        let mut tokenizer = Tokenizer::new(input_string, &settings, false);
        let in_tokens = tokenizer.tokenize();
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
    fn not_eof(&self) -> bool {
        ! matches!(self.at(), Some(Token::Eof) | None)
    }
    pub fn parse_program(&mut self) -> ParseResult {
        let mut program = Module{body: Vec::new()};
        while self.not_eof() {
            program.body.push(self.parse_stmt()?.into());
        }
        Ok(Stmt::Module(program))
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
            self.eat();
            let ttype = match coin.val.as_str() {
                "+" => Lexeme::Add,
                _ => Lexeme::Sub,
            };
            let right = self.parse_multiplicative_expr()?;
            left = Stmt::BinaryExpr(BinaryExpression{ ttype,coin,l:left.into(),r:right.into()});
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
            self.eat();
            let ttype = match coin.val.as_str() {
                "*" => Lexeme::Mult,
                "/" => Lexeme::Div,
                _ => Lexeme::Mod,
            };
            let right = self.parse_primary_expr()?;
            left = Stmt::BinaryExpr(BinaryExpression{ ttype,coin,l:left.into(),r:right.into()});
        }
        Ok(left)
    }
    pub fn parse_primary_expr(&mut self) -> ParseResult {
        match self.at() {
            Some(Token::Identifier(_)) => self.parse_ident(),
            Some(Token::Numeric(_)) => self.parse_numeric(),
            Some(Token::Op(coin)) if coin.val.as_str() == "(" => {
                let coin = coin.clone();
                self.eat();
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
        let Some(Token::Identifier(coin)) = self.eat() else { return Err(ParseError::InvalidIdent(self.in_tokens.get(self.position-1).unwrap_or(&Token::Eof).clone())) };
        Ok(Stmt::Identifier(Identifier{ ttype:Lexeme::Ident,coin:coin.clone()}))
    }
    pub fn parse_numeric(&mut self) -> ParseResult {
        // coin is coin.val (which is a string) and coin.pos
        // check if the string parses to float it or hex
        let Some(Token::Numeric(coin)) = self.eat() else { return Err(ParseError::InvalidNumber(self.in_tokens.get(self.position-1).unwrap_or(&Token::Eof).clone())) };
        let value = &coin.val; // Assuming `coin.val` is the string representation of the number.
        if let Ok(val) = value.parse::<u64>() {
            Ok(Stmt::IntLiteral(IntLiteral{ ttype:Lexeme::Int,coin:coin.clone(),val}))
        } else if let Ok(val) = value.parse::<f64>() {
            Ok(Stmt::FloatLiteral(FloatLiteral{ ttype:Lexeme::Float,coin:coin.clone(),val}))
        } else if let Some(stripped) = value.strip_prefix("0x") {
            let Ok(val) = u64::from_str_radix(stripped, 16) else {
                return Err(ParseError::InvalidNumber(Token::Numeric(coin.clone())))
            };
            Ok(Stmt::IntLiteral(IntLiteral{ ttype:Lexeme::Int,coin:coin.clone(),val}))
        } else {
            Err(ParseError::InvalidNumber(Token::Numeric(coin.clone())))
        }
    }
}
