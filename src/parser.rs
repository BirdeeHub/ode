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
    fn get_token(&self) -> Option<&Token> {
        self.in_tokens.get(self.position)
    }
    fn advance(&self) -> Option<&Token> {
        self.in_tokens.get(self.position)
    }
    pub fn parse_program<T: Stmt>(&self) -> ParseResult<Program<T>> {
        todo!()
    }
    pub fn parsePrimary(&self, in_tokens: &[Token]) -> ParseResult<(Node,&[Token])> {
        todo!()
    }
}
