use crate::tokenizer::{Token, Coin};

enum TokenType {
    TupleS(Meta),
    TupleE(Meta),
    ScopeS(Meta),
    ScopeE(Meta),
    InterpolateS(Meta),
    InterpolateE(Meta),
    FormatS(Meta),
    FormatE(Meta),
    Add(Meta),
    Sub(Meta),
    Mult(Meta),
    Div(Meta),
    Mod(Meta),
    FnOp(Meta),
    Pub(Meta),
    Mut(Meta),
}

struct Meta {
    debug_pos: usize, // <-- position in vector
}

// [] indicates optional in these snippets
// fn syntax: \[:ret_type] name (tuple[:default:type], of[:default:type], args[:default:type]) { body }
// fn syntax: myfn = \[:ret_type] (tuple[:default:type], of[:default:type], args[:default:type]) { body }

// if it ends up manual memory or something like borrow checked,
// you might be able to mark it `\ instead to make it no longer a closure? idk havent got that far yet

// calling fn: name with (args like nix)

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
}
impl<'a> Parser<'a> {
    pub fn new(in_tokens: &'a Vec<Token>) -> Parser {
        Parser{ in_tokens, }
    }
    pub fn parse(&self) -> ExprTree {
        todo!()
    }
}
