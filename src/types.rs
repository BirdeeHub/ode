use crate::tokenizer::{Token, Coin};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    Teapot(Token),
    TypeError(Token),
    AssignmentError(Token),
    StatementError(Token),
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Stmt {
    fn get_type(&self) -> Lexeme;
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub struct Program<T: Stmt> {
    pub statements: Vec<Arc<T>>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression<T: Stmt, V: Stmt> {
    pub token: Token,
    pub ttype: Lexeme,
    pub l: Option<Arc<T>>,
    pub r: Option<Arc<V>>,
}
impl<T: Stmt, V: Stmt> Stmt for BinaryExpression<T,V> {
    fn get_type(&self) -> Lexeme {
        self.ttype
    }
}

#[derive(Debug, PartialEq)]
pub struct NumericLiteral {
    pub token: Token,
    pub ttype: Lexeme,
}
impl Stmt for NumericLiteral {
    fn get_type(&self) -> Lexeme {
        self.ttype
    }
}

#[derive(Debug, PartialEq)]
pub struct Ident {
    pub token: Token,
    pub ttype: Lexeme,
}
impl Stmt for Ident {
    fn get_type(&self) -> Lexeme {
        self.ttype
    }
}
