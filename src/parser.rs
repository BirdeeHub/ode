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
    pub fn parse_program(&self) -> ParseResult {
        let mut program = Module{statements: Vec::new()};
        while self.not_eof() {
            program.statements.push(self.parse_stmt()?.into());
        }
        Ok(Stmt::Module(program))
    }
    pub fn parse_stmt(&self) -> ParseResult {
        todo!()
    }
}
