use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenizerSettings<'a> {
    pub blockcomstart: &'a str,
    pub blockcomend: &'a str,
    pub linecom: &'a str,
    pub ops: &'a [&'a str],
    pub charop: &'a str,
    pub templop: &'a str,
    pub enclosers: &'a [(&'a str, &'a str)],
    pub interstart: &'a str,
    pub interend: &'a str,
    pub escape_char: char,
    pub capture_comments: bool,
}

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String,usize),
    Op(String,usize),
    IntLit(String,usize),
    HexLit(String,usize),
    FloatLit(String,usize),
    Literal(String,usize),
    Comment(String,usize),
    Format(Vec<Token>,usize),
    Eof,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Identifier(s1, _), Token::Identifier(s2, _)) => s1 == s2,
            (Token::Op(s1, _), Token::Op(s2, _)) => s1 == s2,
            (Token::IntLit(s1, _), Token::IntLit(s2, _)) => s1 == s2,
            (Token::HexLit(s1, _), Token::HexLit(s2, _)) => s1 == s2,
            (Token::FloatLit(s1, _), Token::FloatLit(s2, _)) => s1 == s2,
            (Token::Literal(s1, _), Token::Literal(s2, _)) => s1 == s2,
            (Token::Comment(s1, _), Token::Comment(s2, _)) => s1 == s2,
            (Token::Format(v1, _), Token::Format(v2, _)) => v1 == v2, // Delegates to Vec's PartialEq
            (Token::Eof, Token::Eof) => true,
            _ => false,
        }
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
    Module,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    FloatLiteral { pos: usize, ttype: Lexeme, val: f64 },
    IntLiteral { pos: usize, ttype: Lexeme, val: u64 },
    StringLiteral { pos: usize, ttype: Lexeme, val: Arc<str> },
    Identifier { pos: usize, ttype: Lexeme, val: Arc<str> },
    BinaryExpr { pos: usize, ttype: Lexeme, l: Arc<Stmt>, r: Arc<Stmt> },
    PreExpr { pos: usize, ttype: Lexeme, r: Arc<Stmt> },
    PostExpr { pos: usize, ttype: Lexeme, l: Arc<Stmt> },
    GroupExpr { start: usize, end: usize, ttype: Lexeme, body: Arc<Stmt> },
    Scope { start: usize, end: usize, ttype: Lexeme, body: Vec<Arc<Stmt>> },
    Module { body: Vec<Arc<Stmt>>, ttype: Lexeme },
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
