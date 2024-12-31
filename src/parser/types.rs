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
    BinaryExpr(BinaryExpression),
    FloatLiteral(FloatLiteral),
    IntLiteral(IntLiteral),
    Identifier(Identifier),
    Module(Module),
}

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


pub type ParseResult = Result<Stmt, ParseError>;

#[derive(Debug, PartialEq,Clone)]
pub struct Module {
    pub body: Vec<Arc<Stmt>>,
}

#[derive(Debug, PartialEq,Clone)]
pub struct BinaryExpression {
    pub coin: Coin<String>,
    pub ttype: Lexeme,
    pub l: Arc<Stmt>,
    pub r: Arc<Stmt>,
}

#[derive(Debug, PartialEq,Clone)]
pub struct FloatLiteral {
    pub coin: Coin<String>,
    pub ttype: Lexeme,
    pub val: f64,
}

#[derive(Debug, PartialEq,Clone)]
pub struct IntLiteral {
    pub coin: Coin<String>,
    pub ttype: Lexeme,
    pub val: u64,
}

#[derive(Debug, PartialEq,Clone)]
pub struct Identifier {
    pub coin: Coin<String>,
    pub ttype: Lexeme,
}
