use crate::tokenizer::{Token, Coin};

#[derive(Debug, PartialEq)]
pub enum Lexemes {
    ParenB(Meta), // (
    ParenE(Meta), // )
    ScopeB(Meta), // {
    ScopeE(Meta), // }
    InterpolateB(Meta), // $[
    InterpolateE(Meta), // ]
    FormatB(Meta), // "
    FormatE(Meta), // "
    CharS(Meta), // '
    SquareB(Meta), // [
    SquareE(Meta), // ]
    Chain(Meta), // ,
    Add(Meta), // +
    Sub(Meta), // -
    Mult(Meta), // *
    Div(Meta), // /
    Pow(Meta), // ^
    Mod(Meta), // %
    FnOp(Meta), // \
    FnOpNamed(Meta), // \: followed by IDENT
    Pipe(Meta), // |>
    Pub(Meta), // pub
    Hex(Meta, i64),
    Int(Meta, i64),
    Float(Meta, f64),
    Char(Meta, char),
    Type(Meta, String), // <-- will probably need to be a type struct
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
    Colon(Meta), // ":"
    Semicolon(Meta), // ";"
    StaticField(Meta), // ::
    Field(Meta), // .
    BitAnd(Meta), // &
    BitOr(Meta), // |
    And(Meta), // &&
    Or(Meta), // ||
    For(Meta), // for
    True(Meta), // true
    False(Meta), // false
    Enum(Meta), // enum
    Match(Meta), // match
    Struct(Meta), // struct
    Implement(Meta), // impl <-- in this language, you will be able to implement traits on structs not created by your file, allowing pseudo-structural typing
    Trait(Meta), // trait
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
