use crate::tokenizer::{Token, Coin};

struct Meta {
    debug_pos: usize, // <-- position in vector
}

// [] indicates optional in these snippets
// fn syntax: \:name some[:default:type], args[:default:type] -> [ret_type] { body }
// fn syntax: myfn = \ some[:default:type], args[:default:type] -> [ret_type] { body }
// functions are closures

// calling function requires no parenthesis around args other than for grouping

// infer types where possible

#[derive(Debug, PartialEq)]
pub struct Atom {
}

//struct PreExpr { <-- infix operators and prefix operators are to be the same thing, 1 arg can only be called prefix, for methods, self var eats the ability to be infix
//}

#[derive(Debug, PartialEq)]
pub struct Expr {
}

//struct PostExpr { <- will be infix operators with default value instead. you may curry up until the first default argument,
//}               at which point you must provide the rest or it will call, varargs are allowed at end and cannot be curried.

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
