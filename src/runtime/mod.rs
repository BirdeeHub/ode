mod types;
use crate::parser::types::*;
use crate::runtime::types::*;

fn eval_program(code: &Stmt) -> RuntimeResult {
    let Stmt::Module(code) = code else { return Err(RuntimeError::Teapot) };
    let mut last: RuntimeResult = Err(RuntimeError::Teapot);
    for stmt in code.body.iter() {
        last = evaluate(stmt);
        println!("{:?}", last);
    }
    last
}

fn eval_float_binary_expr(lhs: f64, rhs: f64, op: Lexeme) -> RuntimeResult {
    match op {
        Lexeme::Add => Ok(RuntimeVal::Float(lhs + rhs)),
        Lexeme::Sub => Ok(RuntimeVal::Float(lhs - rhs)),
        Lexeme::Mult => Ok(RuntimeVal::Float(lhs * rhs)),
        Lexeme::Div => Ok(RuntimeVal::Float(lhs / rhs)),
        Lexeme::Mod => Ok(RuntimeVal::Float(lhs % rhs)),
        _ => Err(RuntimeError::Teapot),
    }
}

fn eval_int_binary_expr(lhs: i64, rhs: i64, op: Lexeme) -> RuntimeResult {
    match op {
        Lexeme::Add => Ok(RuntimeVal::Int(lhs + rhs)),
        Lexeme::Sub => Ok(RuntimeVal::Int(lhs - rhs)),
        Lexeme::Mult => Ok(RuntimeVal::Int(lhs * rhs)),
        Lexeme::Div => Ok(RuntimeVal::Int(lhs / rhs)),
        Lexeme::Mod => Ok(RuntimeVal::Int(lhs % rhs)),
        _ => Err(RuntimeError::Teapot),
    }
}

fn eval_binary_expr(code: &Stmt) -> RuntimeResult {
    let Stmt::BinaryExpr(binop) = code else { return Err(RuntimeError::Teapot) };
    let lhs = evaluate(&binop.l)?;
    let rhs = evaluate(&binop.r)?;
    if let RuntimeVal::Float(l) = lhs {
        if let RuntimeVal::Float(r) = rhs {
            eval_float_binary_expr(l, r, binop.ttype)
        } else {
            Err(RuntimeError::TypeError("cannot add a float to another type"))
        }
    } else if let RuntimeVal::Int(l) = lhs {
        if let RuntimeVal::Int(r) = rhs {
            eval_int_binary_expr(l, r, binop.ttype)
        } else {
            Err(RuntimeError::TypeError("cannot add an int to another type"))
        }
    } else {
        Err(RuntimeError::Teapot)
    }
}

pub fn evaluate(code: &Stmt) -> RuntimeResult {
    match code {
        Stmt::IntLiteral(val) => Ok(RuntimeVal::Int(val.val as i64)),
        Stmt::FloatLiteral(val) => Ok(RuntimeVal::Float(val.val)),
        Stmt::BinaryExpr(expr) => eval_binary_expr(&Stmt::BinaryExpr(expr.clone())),
        Stmt::Module(expr) => eval_program(&Stmt::Module(expr.clone())),
        _ => todo!(),
    }
}
