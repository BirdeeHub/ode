#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeVal {
    Float(f64),
    Int(i64),
}

pub type RuntimeResult<'a> = Result<RuntimeVal, RuntimeError<'a>>;

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError<'a> {
    TypeError(&'a str),
    Teapot,
}
