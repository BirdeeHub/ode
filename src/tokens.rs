#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Op(String),
    Keyword(String),
    Encloser(Facing<String>),
    Numeric(String), // int or float in string form
    Literal(String), // between literal enclosers
    Template(Format), // between " enclosers
    Semicolon,
    Eof, // End of file/input
    Unknown(char),
}

pub type Format = Vec<Token>;

pub struct Keywords;

impl Keywords {
    const WORDS: &'static [&'static str] = &[
        "if", "else", "while", "where", "for", "mod", "return", "fn", "let", "mut", "const",
        "match",
    ];

    pub fn is(word: &str) -> bool {
        Self::WORDS.contains(&word)
    }
    pub fn is_fragment(word: &str) -> bool {
        Self::WORDS
            .iter()
            .any(|&key| key != word && key.starts_with(word))
    }
}

pub struct Enclosers;

#[derive(Debug, PartialEq)]
pub enum Facing<T> {
    Left(T),
    Right(T),
    Either(T),
    Neither(T),
}

fn is_literal_left(op: &str) -> bool {
    if op.len() < 2 {
        false
    } else if op == "[[" {
        true
    } else {
        op.starts_with("[=")
        && (
            (op.ends_with("[") && op[1..op.len() - 1].chars().all(|c| c == '='))
            || (op.ends_with("=") && op[1..op.len()].chars().all(|c| c == '='))
        )
    }
}
fn is_literal_right(op: &str) -> bool {
    if op.len() < 2 {
        false
    } else if op == "]]" {
        true
    } else {
        op.starts_with("]=")
        && (
            (op.ends_with("]") && op[1..op.len() - 1].chars().all(|c| c == '='))
            || (op.ends_with("=") && op[1..op.len()].chars().all(|c| c == '='))
        )
    }
}

impl Enclosers {
    const OPS: &'static [(&'static str, &'static str)] = &[
        ("(", ")"),
        ("{", "}"),
        ("$[", "]"),
        ("#<", ">"),
        ("<", ">"),
        ("[", "]"),
        ("`", "`"),
        ("\"", "\""),
        ("\'", "\'"),
    ];

    pub fn is(op: &str) -> bool {
        Self::OPS
            .iter()
            .any(|(left, right)| left == &op || right == &op)
            || Self::is_literal(op)
    }
    pub fn is_literal(op: &str) -> bool {
        is_literal_left(op) || is_literal_right(op)
    }
    pub fn l_or_r(op: String) -> Facing<String> {
        match op {
            _ if is_literal_left(op.as_str()) => Facing::Left(op),
            _ if is_literal_right(op.as_str()) => Facing::Right(op),
            _ if op == "`" || op == "\"" => Facing::Either(op),
            _ if Self::OPS.iter().any(|(left, _right)| left == &op) => Facing::Left(op),
            _ if Self::OPS.iter().any(|(_left, right)| right == &op) => Facing::Right(op),
            _ => Facing::Neither(op.to_string()),
        }
    }
    pub fn is_fragment(op: &str) -> bool {
        Self::OPS.iter().any(|(l_def, r_def)| {
            (l_def != &op && l_def.starts_with(op)) || (r_def != &op && r_def.starts_with(op))
        }) ||
        Self::is_literal(op) && op.ends_with("=")
    }
}

pub struct Ops;

impl Ops {
    const OPS: &'static [&'static str] = &[
        "=", "+=", "-=", "*=", "/=", "+", "-", "*", "/", "%", "&", ".", "|", "&&", "||", "==",
        "!=", "<", "<=", ">", ">=", "=>", "|>", "<|", "'", "!", "=~", "?", ",", "++"
    ];

    pub fn is(op: &str) -> bool {
        Self::OPS.contains(&op)
    }
    pub fn is_fragment(op: &str) -> bool {
        Self::OPS
            .iter()
            .any(|&op_def| op_def != op && op_def.starts_with(op))
    }
}
