#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeVal {
    Float(f64),
    Int(i64),
}

pub type RuntimeResult = Result<RuntimeVal, RuntimeError>;

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError {
    TypeError(&'static str),
    Teapot,
}
