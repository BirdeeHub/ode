use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(Coin<String>),
    Op(Coin<String>),
    Numeric(Coin<String>), // int or float in string form
    Literal(Coin<String>),
    Format(Coin<Vec<Token>>),
    Eof,
}

#[derive(Debug, Clone)]
pub struct Coin<T> {
    pub val: T,
    pub pos: usize,
}
impl<T> Coin<T> {
    pub fn new(val: T, pos: usize) -> Coin<T> {
        Coin{val,pos}
    }
}
impl<T: PartialEq> PartialEq for Coin<T> {
    fn eq(&self, other: &Coin<T>) -> bool {
        self.val == other.val
    }
}

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
    Mult, // *
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

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    FloatLiteral { coin: Coin<String>, ttype: Lexeme, val: f64 },
    IntLiteral { coin: Coin<String>, ttype: Lexeme, val: u64 },
    StringLiteral { coin: Coin<String>, ttype: Lexeme, val: Arc<str> },
    Identifier { coin: Coin<String>, ttype: Lexeme, val: Arc<str> },
    BinaryExpr { coin: Coin<String>, ttype: Lexeme, l: Arc<Stmt>, r: Arc<Stmt> },
    PreExpr { coin: Coin<String>, ttype: Lexeme, r: Arc<Stmt> },
    PostExpr { coin: Coin<String>, ttype: Lexeme, l: Arc<Stmt> },
    GroupExpr { start: Coin<String>, end: Coin<String>, ttype: Lexeme, body: Arc<Stmt> },
    Scope { start: Coin<String>, end: Coin<String>, ttype: Lexeme, body: Vec<Arc<Stmt>> },
    Module { body: Vec<Arc<Stmt>> },
}

pub type ParseResult = Result<Stmt, ParseError>;
#[derive(Debug, PartialEq)]
pub enum ParseError {
    Teapot(Token),
    TypeError(Token),
    InvalidNumber(Token),
    InvalidIdent(Token),
    InvalidExpression(Token),
    UnmatchedEncloser(Token),
    AssignmentError(Token),
    StatementError(Token),
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
