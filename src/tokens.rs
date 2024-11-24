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

pub struct Ops<'a> {
    pub blockcomstart: &'a str,
    pub blockcomend: &'a str,
    pub linecom: &'a str,
    ops: &'a [&'a str],
    capops: &'a [&'a str],
}

pub struct TokenizerSettings<'a> {
    pub blockcomstart: &'a str,
    pub blockcomend: &'a str,
    pub linecom: &'a str,
    pub ops: &'a [&'a str],
    pub capops: &'a [&'a str],
}

impl<'a> Ops<'a> {
    pub fn new(options: TokenizerSettings<'a>) -> Ops<'a> {
        // Create a new vector for ops that includes blockcomstart, blockcoment, and linecom.
        let mut combined_ops = Vec::new();
        combined_ops.push(options.blockcomstart);
        combined_ops.push(options.blockcomend);
        combined_ops.push(options.linecom);
        combined_ops.extend_from_slice(options.ops);

        // Convert Vec to slice for the new Ops struct
        let combined_ops: &'a [&'a str] = Box::leak(combined_ops.into_boxed_slice());

        Ops {
            blockcomstart: options.blockcomstart,
            blockcomend: options.blockcomend,
            linecom: options.linecom,
            ops: combined_ops,
            capops: options.capops,
        }
    }

    pub fn is(&self, op: &str) -> bool {
        self.ops.contains(&op) || self.capops.contains(&op) ||
        Ops::<'a>::is_literal_left(op) || Ops::<'a>::is_literal_right(op)
    }
    pub fn is_fragment(&self, op: &str) -> bool {
        self.ops
            .iter()
            .any(|&op_def| op_def != op && op_def.starts_with(op)) ||
        is_literal_left_frag(op) || is_literal_right_frag(op)
    }

    pub fn is_other_capturing(&self, op: &str) -> bool {
        self.capops.contains(&op)
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
