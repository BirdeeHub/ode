mod types;
use crate::parser::parser_types::*;
use std::sync::Arc;
use crate::runtime::types::*;

fn eval_program(code: &[Arc<Stmt>]) -> RuntimeResult {
    let mut last: RuntimeResult = Err(RuntimeError::Teapot);
    for stmt in code.iter() {
        last = evaluate(stmt);
        println!("{:?}", last);
    }
    last
}

fn eval_float_binary_expr(lhs: f64, rhs: f64, op: Lexeme) -> RuntimeResult<'static> {
    match op {
        Lexeme::Add => Ok(RuntimeVal::Float(lhs + rhs)),
        Lexeme::Sub => Ok(RuntimeVal::Float(lhs - rhs)),
        Lexeme::Mult => Ok(RuntimeVal::Float(lhs * rhs)),
        Lexeme::Div => Ok(RuntimeVal::Float(lhs / rhs)),
        Lexeme::Mod => Ok(RuntimeVal::Float(lhs % rhs)),
        _ => Err(RuntimeError::Teapot),
    }
}

fn eval_int_binary_expr(lhs: i64, rhs: i64, op: Lexeme) -> RuntimeResult<'static> {
    match op {
        Lexeme::Add => Ok(RuntimeVal::Int(lhs + rhs)),
        Lexeme::Sub => Ok(RuntimeVal::Int(lhs - rhs)),
        Lexeme::Mult => Ok(RuntimeVal::Int(lhs * rhs)),
        Lexeme::Div => Ok(RuntimeVal::Int(lhs / rhs)),
        Lexeme::Mod => Ok(RuntimeVal::Int(lhs % rhs)),
        _ => Err(RuntimeError::Teapot),
    }
}

fn eval_binary_expr<'a>(coin: Coin<String>, ttype: Lexeme, l: &'a Stmt, r: &'a Stmt) -> RuntimeResult<'a> {
    let lhs = evaluate(l)?;
    let rhs = evaluate(r)?;
    if let RuntimeVal::Float(ls) = lhs {
        if let RuntimeVal::Float(rs) = rhs {
            eval_float_binary_expr(ls, rs, ttype)
        } else {
            Err(RuntimeError::TypeError("cannot add a float to another type"))
        }
    } else if let RuntimeVal::Int(ls) = lhs {
        if let RuntimeVal::Int(rs) = rhs {
            eval_int_binary_expr(ls, rs, ttype)
        } else {
            Err(RuntimeError::TypeError("cannot add an int to another type"))
        }
    } else {
        Err(RuntimeError::TypeError("math expr on non-number"))
    }
}

pub fn evaluate(code: &Stmt) -> RuntimeResult {
    match code {
        Stmt::IntLiteral { coin: _, ttype: _, val } => Ok(RuntimeVal::Int(*val as i64)),
        Stmt::FloatLiteral { coin: _, ttype: _, val } => Ok(RuntimeVal::Float(*val)),
        Stmt::BinaryExpr { coin, ttype, l, r } => eval_binary_expr(coin.clone(), *ttype, l, r),
        Stmt::Module { body, ttype: _ } => eval_program(&body[..]),
        _ => todo!(),
    }
}
