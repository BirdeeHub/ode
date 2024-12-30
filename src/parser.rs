use crate::tokenizer::{Token, Coin};
use crate::types::*;

#[derive(Debug, PartialEq)]
pub struct Parser<'a> {
    in_tokens: &'a Vec<Token>,
    position: usize,
}
impl<'a> Parser<'a> {
    pub fn new(in_tokens: &'a Vec<Token>) -> Parser {
        Parser{ in_tokens, position: 0, }
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
        self.parse_primary()
    }
    pub fn parse_primary(&mut self) -> ParseResult {
        match self.at() {
            Some(Token::Identifier(_)) => self.parse_ident(),
            Some(Token::Numeric(_)) => self.parse_numeric(),
            _ => Err(ParseError::InvalidExpression(self.at().unwrap_or(&Token::Eof).clone())),
        }
    }
    pub fn parse_ident(&mut self) -> ParseResult {
        let Some(Token::Identifier(coin)) = self.eat() else { return Err(ParseError::Teapot(Token::Eof)) };
        Ok(Stmt::Identifier(Identifier{ ttype:Lexeme::Ident,coin:coin.clone()}))
    }
    pub fn parse_numeric(&mut self) -> ParseResult {
        // coin is coin.val (which is a string) and coin.pos
        // check if the string parses to float it or hex
        let Some(Token::Numeric(coin)) = self.eat() else { return Err(ParseError::Teapot(Token::Eof)) };
        let value = &coin.val; // Assuming `coin.val` is the string representation of the number.
        if let Ok(val) = value.parse::<i64>() {
            Ok(Stmt::IntLiteral(IntLiteral{ ttype:Lexeme::Int,coin:coin.clone(),val}))
        } else if let Ok(val) = value.parse::<f32>() {
            Ok(Stmt::FloatLiteral(FloatLiteral{ ttype:Lexeme::Float,coin:coin.clone(),val}))
        } else if let Some(stripped) = value.strip_prefix("0x") {
            let Ok(val) = u64::from_str_radix(stripped, 16) else {
                return Err(ParseError::InvalidNumber(Token::Numeric(coin.clone())))
            };
            Ok(Stmt::IntLiteral(IntLiteral{ ttype:Lexeme::Int,coin:coin.clone(),val:val as i64}))
        } else {
            Err(ParseError::InvalidNumber(Token::Numeric(coin.clone())))
        }
    }
}
