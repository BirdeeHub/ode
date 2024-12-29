use crate::tokenizer::{Token, Coin};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub enum Lexeme {
    ParenB, // (
    ParenE, // )
    ScopeB, // { Can also be generic set if , instead of ;
    ScopeE, // } All scopes return a value or () (good substitute for let in)
    InterpolateB, // $[
    InterpolateE, // ]
    FormatB, // "
    FormatE, // "
    AngleB, // <
    AngleE, // >
    CharS, // '
    SquareB, // [ Will be tuples
    SquareE, // ]
    Chain, // ,
    Add, // +
    Sub, // -
    Star, // *
    Div, // /
    Pow, // ^
    Mod, // %
    FnOp, // \
    Return, // << for early return
    FnOpNamed, // \: followed by IDENT
    FnOpInfix, // \:: followed by IDENT
    VarArg, // ...
    ArgEnd, // ->
    SelfArg, // self may be first argument to mark as method
    Pipe, // |>
    Hex,
    Int,
    Float,
    Char,
    Ident,
    Literal,
    Assign, // =
    Mut, // `
    Gt, // >
    Lt, // <
    GtEq, // >=
    LtEq, // <=
    Eq, // =
    Semicolon, // ";"
    TypeSep, // :
    Field, // .
    BitAnd, // &
    BitOr, // |
    And, // &&
    Or, // ||
    True, // true
    False, // false
    Enum, // =|
    For, // for
    Match, // ~|
    Then, // =>
    Else, // >>
    ElseIf, // >>>
    Struct, // struct
    Implement, // ^=
    ConstraintAssign, // _=
    Send, // <@
    Recieve, // @>
    Spawn, // @
    Format,
    Pattern,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Arc<Node>>,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub token: Token,
    pub ttype: Lexeme,
    pub l: Option<Arc<Node>>,
    pub r: Option<Arc<Node>>,
    pub scope_id: Option<usize>,
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
    pub fn parse_program(&self) -> ParseResult<Program> {
        todo!()
    }
    pub fn parsePrimary(&self, in_tokens: &[Token]) -> ParseResult<(Node,&[Token])> {
        todo!()
    }
}
