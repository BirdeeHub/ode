use crate::tokenizer::{Token, Coin};

#[derive(Debug, PartialEq)]
pub enum Lexemes {
    ParenB(Meta), // (
    ParenE(Meta), // )
    ScopeB(Meta), // { Can also be generic set if , instead of ;
    ScopeE(Meta), // } All scopes return a value or () (good substitute for let in)
    InterpolateB(Meta), // $[
    InterpolateE(Meta), // ]
    FormatB(Meta), // "
    FormatE(Meta), // "
    AngleB(Meta), // <
    AngleE(Meta), // >
    CharS(Meta), // '
    SquareB(Meta), // [ Will be tuples
    SquareE(Meta), // ]
    Chain(Meta), // ,
    Add(Meta), // +
    Sub(Meta), // -
    Star(Meta), // *
    Div(Meta), // /
    Pow(Meta), // ^
    Mod(Meta), // %
    FnOp(Meta), // \
    Return(Meta), // << for early return
    FnOpNamed(Meta), // \: followed by IDENT
    FnOpInfix(Meta), // \:: followed by IDENT
    Pipe(Meta), // |>
    Hex(Meta, i64),
    Int(Meta, i64),
    Float(Meta, f64),
    Char(Meta, char),
    Ident(Meta, String),
    Literal(Meta, String),
    Assign(Meta), // =
    Mut(Meta), // `
    MutAssign(Meta), // `=
    SubAssign(Meta), // -=
    AddAssign(Meta), // +=
    Gt(Meta), // >
    Lt(Meta), // <
    GtEq(Meta), // >=
    LtEq(Meta), // <=
    Eq(Meta), // =
    Semicolon(Meta), // ";"
    TypeSep(Meta), // :
    Field(Meta), // .
    BitAnd(Meta), // &
    BitOr(Meta), // |
    And(Meta), // &&
    Or(Meta), // ||
    True(Meta), // true
    False(Meta), // false
    Enum(Meta), // enum
    For(Meta), // for
    Match(Meta), // ~@ 
    Then(Meta), // =>
    Else(Meta), // >>
    ElseIf(Meta), // >>>
    Struct(Meta), // struct
    Implement(Meta), // impl <-- in this language, you will be able to implement traits on structs not created by your file, allowing pseudo-structural typing
    Trait(Meta), // trait
    Pub(Meta), // pub
}

#[derive(Debug, PartialEq)]
pub struct Meta {
    debug_pos: usize, // <-- position in vector
}

#[derive(Debug, PartialEq)]
pub struct Lexxer<'a> {
    in_tokens: &'a Vec<Token>,
    position: usize,
}
impl<'a> Lexxer<'a> {
    pub fn new(in_tokens: &'a Vec<Token>) -> Lexxer {
        Lexxer{ in_tokens, position: 0, }
    }
    fn get_token(&self) -> Option<&Token> {
        self.in_tokens.get(self.position)
    }
    fn advance(&self) -> Option<&Token> {
        self.in_tokens.get(self.position)
    }
    pub fn lex(&self) -> Vec<Lexemes> {
        todo!()
    }
}
