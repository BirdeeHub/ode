#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Op(String),
    Numeric(String), // int or float in string form
    Literal(String),
    Eof,
}

fn is_literal_left_frag(op: &str) -> bool {
    op == "[" || op.starts_with("[") && op.len() > 1
        && op[1..].chars().all(|c| c == '=')
}
fn is_literal_right_frag(op: &str) -> bool {
    op == "]" || op.starts_with("]") && op.len() > 1
        && op[1..].chars().all(|c| c == '=')
}

pub struct Ops;

impl Ops {
    pub const BLOCKCOMSTART: &'static str = "/*";
    pub const BLOCKCOMEND: &'static str = "*/";
    pub const LINECOM: &'static str = "//";
    const OPS: &'static [&'static str] = &[ Self::BLOCKCOMSTART, Self::BLOCKCOMEND, Self::LINECOM,
        "=", "+=", "-=", "*=", "/=", "+", "-", "*", "/", "%", "&", ".", "|", "&&", "||", "==",
        "!=", "<", "<=", ">", ">=", "=>", "|>", "<|", "'", "!", "=~", "?", ",", "++", ":",
        "::", ";", "{", "}", "[", "]", "(", ")",
    ];
    const CAPOPS: &'static [&'static str] = &[
        "'", "\"",
    ];

    pub fn is(op: &str) -> bool {
        Self::OPS.contains(&op) || Self::CAPOPS.contains(&op) ||
        Self::is_literal_left(op) || Self::is_literal_right(op)
    }
    pub fn is_fragment(op: &str) -> bool {
        Self::OPS
            .iter()
            .any(|&op_def| op_def != op && op_def.starts_with(op)) ||
        is_literal_left_frag(op) || is_literal_right_frag(op)
    }

    pub fn is_other_capturing(op: &str) -> bool {
        Self::CAPOPS.contains(&op)
    }

    pub fn is_literal_left(op: &str) -> bool {
        op.starts_with("[") && op.len() > 1
            && (op.ends_with("[") && op[1..op.len() - 1].chars().all(|c| c == '='))
    }
    pub fn get_literal_left(left_lit_op: &str) -> String {
        left_lit_op.replace("[", "]")
    }
    pub fn is_literal_right(op: &str) -> bool {
        op.starts_with("]") && op.len() > 1
            && (op.ends_with("]") && op[1..op.len() - 1].chars().all(|c| c == '='))
    }

}
