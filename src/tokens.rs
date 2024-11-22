#[derive(Debug, PartialEq)]
pub enum Token {
    Atom(String),
    Uop(String),
    BinOp(String),
    Keyword(String),
    Number(i32),
    Encloser(String),
    Semicolon,
    Eof, // End of file/input
    Unknown(char),
}

pub struct Keywords;

pub struct Uops;

pub struct BinOps;

pub struct Enclosers;

enum Paths<T> {
    Left(T),
    Right(T),
    Either(T),
    Neither(T),
}

fn is_long_left(op: &str) -> bool {
    op.starts_with("[=") &&
    (op.ends_with("[") || op.ends_with("=")) &&
    op[2..op.len() - 1].chars().all(|c| c == '=')
}
fn is_long_right(op: &str) -> bool {
    op.starts_with("]=") &&
    (op.ends_with("]") || op.ends_with("=")) &&
    op[2..op.len() - 1].chars().all(|c| c == '=')
}

impl Enclosers {
    pub const OPS: &'static [(&'static str, &'static str)] = &[
        ("(", ")"),
        ("{", "}"),
        ("<", ">"),
        ("[", "]"),
        ("`", "`"),
        ("\"", "\""),
        ("[[", "]]"),
    ];

    pub fn is(op: &str) -> bool {
        Self::OPS.iter().any(|(left, right)| left == &op || right == &op) ||
        Self::is_long(op)
    }
    pub fn is_long(op: &str) -> bool {
        is_long_left(op) || is_long_right(op)
    }
    pub fn complete(op: &str) -> bool {
        Self::is(op) && ! op.ends_with("=")
    }
    pub fn l_or_r(op: &str) -> Paths<&str> {
        match op {
            _ if is_long_left(op) => Paths::Left(op),
            _ if is_long_right(op) => Paths::Right(op),
            _ if op == "`" || op == "\"" => Paths::Either(op),
            _ if Self::OPS.iter().any(|(left, _right)| left == &op) => Paths::Left(op),
            _ if Self::OPS.iter().any(|(_left, right)| right == &op) => Paths::Right(op),
            _ => Paths::Neither(op),
        }
    }
}

impl Keywords {
    pub const WORDS: &'static [&'static str] = &[
        "if",
        "else",
        "while",
        "where",
        "for",
        "return",
        "fn",
        "let",
        "mut",
        "const",
        "match",
    ];

    fn is(&self, word: &str) -> bool {
        Self::WORDS.contains(&word)
    }
}

impl Uops {
    pub const UOPS: &'static [&'static str] = &[
        "-",
        "&",
        "*",
    ];

    fn is(&self, op: &str) -> bool {
        Self::UOPS.contains(&op)
    }
}

impl BinOps {
    pub const BINOPS: &'static [&'static str] = &[
        "=",
        "+=",
        "-=",
        "*=",
        "/=",
        "+",
        "-",
        "*",
        "/",
        "%",
        "&",
        ".",
        "|",
        "&&",
        "||",
        "==",
        "!=",
        "<",
        "<=",
        ">",
        ">=",
        "|>",
        "<|",
    ];

    fn is(&self, op: &str) -> bool {
        Self::BINOPS.contains(&op)
    }
}

