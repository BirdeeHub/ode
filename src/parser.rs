use crate::tokenizer::{Token, Coin};

struct Meta {
    debug_pos: usize, // <-- position in vector
}

#[derive(Debug, PartialEq)]
pub struct Atom {
}

#[derive(Debug, PartialEq)]
struct PreExpr {
}

#[derive(Debug, PartialEq)]
pub struct InfixExpr {
}

//struct PostExpr { <- will be infix operators with default value as a second arg instead.
//}

#[derive(Debug, PartialEq)]
pub struct ExprTree {
}

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
    pub fn parse(&self) -> ExprTree {
        todo!()
    }
}
