#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Op(String),
    Numeric(String), // int or float in string form
    Literal(String),
    Format(Vec<Token>),
    Eof,
}

fn is_literal_left_frag(op: &str) -> bool {
    op == "[" || op.starts_with("[") && op.len() > 1 && op[1..].chars().all(|c| c == '=')
}
fn is_literal_right_frag(op: &str) -> bool {
    op == "]" || op.starts_with("]") && op.len() > 1 && op[1..].chars().all(|c| c == '=')
}

pub struct Ops<'a> {
    pub blockcomstart: &'a str,
    pub blockcomend: &'a str,
    pub linecom: &'a str,
    pub interstart: &'a str,
    pub interend: &'a str,
    pub enclosers: Vec<(&'a str, &'a str)>,
    ops: &'a [&'a str],
    capops: &'a [&'a str],
    templops: &'a [&'a str],
}

#[derive(Clone)]
pub struct TokenizerSettings<'a> {
    pub blockcomstart: &'a str,
    pub blockcomend: &'a str,
    pub linecom: &'a str,
    pub ops: &'a [&'a str],
    pub capops: &'a [&'a str],
    pub templops: &'a [&'a str],
    pub enclosers: &'a [(&'a str, &'a str)],
    pub interstart: &'a str,
    pub interend: &'a str,
}

impl<'a> Ops<'a> {
    pub fn new(options: &'a TokenizerSettings<'a>) -> Ops<'a> {
        let mut combined_ops = vec![
            options.blockcomstart,
            options.blockcomend,
            options.linecom,
            options.interstart,
            options.interend,
        ];
        combined_ops.extend_from_slice(options.ops);
        combined_ops.extend_from_slice(options.capops);
        combined_ops.extend_from_slice(options.templops);
        for (open, close) in options.enclosers {
            combined_ops.push(open);
            combined_ops.push(close);
        }
        let filtered_enclosers = options
            .enclosers
            .iter()
            .filter(|(_, close)| *close == options.interend)
            .cloned()
            .collect();

        // Convert Vec to slice for the new Ops struct
        let combined_ops: &'a [&'a str] = Box::leak(combined_ops.into_boxed_slice());

        Ops {
            blockcomstart: options.blockcomstart,
            blockcomend: options.blockcomend,
            linecom: options.linecom,
            ops: combined_ops,
            capops: options.capops,
            templops: options.templops,
            enclosers: filtered_enclosers,
            interstart: options.interstart,
            interend: options.interend,
        }
    }

    pub fn is(&self, op: &str) -> bool {
        self.ops.contains(&op) || Ops::is_literal_left(op) || Ops::is_literal_right(op)
    }
    pub fn is_fragment(&self, op: &str) -> bool {
        self.ops
            .iter()
            .any(|&op_def| op_def != op && op_def.starts_with(op))
            || is_literal_left_frag(op)
            || is_literal_right_frag(op)
    }

    pub fn is_template_op(&self, op: &str) -> bool {
        self.templops.contains(&op)
    }

    pub fn is_other_capturing(&self, op: &str) -> bool {
        self.capops.contains(&op) || self.templops.contains(&op)
    }

    pub fn is_left_encloser(&self, op: &str) -> bool {
        self.enclosers.iter().any(|(left, _)| *left == op)
    }

    pub fn is_right_encloser(&self, op: &str) -> bool {
        self.enclosers.iter().any(|(right, _)| *right == op)
    }

    pub fn is_literal_left(op: &str) -> bool {
        op.starts_with("[")
            && op.len() > 1
            && (op.ends_with("[") && op[1..op.len() - 1].chars().all(|c| c == '='))
    }
    pub fn get_literal_left(left_lit_op: &str) -> String {
        left_lit_op.replace("[", "]")
    }
    pub fn is_literal_right(op: &str) -> bool {
        op.starts_with("]")
            && op.len() > 1
            && (op.ends_with("]") && op[1..op.len() - 1].chars().all(|c| c == '='))
    }
}
