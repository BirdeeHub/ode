use crate::tokenizer::{Token, Coin};

enum TokenType {
}

struct UnaryExpr {
}

struct InfixExpr {
}

struct ExprTree {
}

#[derive(Debug, PartialEq)]
pub struct Parser {
    in_tokens: Vec<Token>,
}
impl Parser {
    pub fn new(in_tokens: Vec<Token>) -> Parser {
        Parser{ in_tokens, }
    }
    pub fn parse(&self) -> () {
        todo!()
    }
}
