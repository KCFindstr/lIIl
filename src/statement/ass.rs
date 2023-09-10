use crate::{
    data::{context::ContextRc, lvalue::LValue, variable::VarType},
    expr::{CompareOp, Expr, NotExpr},
};

use super::CodeExecError;

#[derive(Debug, Clone)]
pub struct AssStatement {
    pub lhs: LValue,
    pub rhs: Expr,
    pub op: CompareOp,
}

fn next_up_f64(value: f64) -> f64 {
    // We must use strictly integer arithmetic to prevent denormals from
    // flushing to zero after an arithmetic operation on some platforms.
    const TINY_BITS: u64 = 0x0000_0000_0000_0001; // Smallest (in magnitude) positive f64.
    const CLEAR_SIGN_MASK: u64 = 0x7fff_ffff_ffff_ffff;

    let bits = value.to_bits();
    if value.is_nan() || bits == f64::INFINITY.to_bits() {
        return value;
    }

    let abs = bits & CLEAR_SIGN_MASK;
    let next_bits = if abs == 0 {
        TINY_BITS
    } else if bits == abs {
        bits + 1
    } else {
        bits - 1
    };
    f64::from_bits(next_bits)
}

fn next_down_f64(value: f64) -> f64 {
    // We must use strictly integer arithmetic to prevent denormals from
    // flushing to zero after an arithmetic operation on some platforms.
    const NEG_TINY_BITS: u64 = 0x8000_0000_0000_0001; // Smallest (in magnitude) negative f64.
    const CLEAR_SIGN_MASK: u64 = 0x7fff_ffff_ffff_ffff;

    let bits = value.to_bits();
    if value.is_nan() || bits == f64::NEG_INFINITY.to_bits() {
        return value;
    }

    let abs = bits & CLEAR_SIGN_MASK;
    let next_bits = if abs == 0 {
        NEG_TINY_BITS
    } else if bits == abs {
        bits - 1
    } else {
        bits + 1
    };
    f64::from_bits(next_bits)
}

fn next_up_i64(value: i64) -> i64 {
    if value == i64::MAX {
        value
    } else {
        value + 1
    }
}

fn next_down_i64(value: i64) -> i64 {
    if value == i64::MIN {
        value
    } else {
        value - 1
    }
}

impl AssStatement {
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        let rhs = self.rhs.eval(ctx)?;
        match &self.op {
            CompareOp::Equal | CompareOp::GreaterEqual | CompareOp::LessEqual => {
                self.lhs.set(ctx, rhs)?;
                return Ok(None);
            }
            CompareOp::NotEqual => {
                self.lhs.set(ctx, NotExpr::not(ctx, rhs)?)?;
                return Ok(None);
            }
            _ => {}
        }
        self.lhs.set(
            ctx,
            match (&rhs, &self.op) {
                (VarType::Float(rhs), CompareOp::Greater) => VarType::Float(next_up_f64(*rhs)),
                (VarType::Float(rhs), CompareOp::Less) => VarType::Float(next_down_f64(*rhs)),
                (VarType::Int(rhs), CompareOp::Greater) => VarType::Int(next_up_i64(*rhs)),
                (VarType::Int(rhs), CompareOp::Less) => VarType::Int(next_down_i64(*rhs)),
                (VarType::Bool(_rhs), CompareOp::Greater) => VarType::Bool(true),
                (VarType::Bool(_rhs), CompareOp::Less) => VarType::Bool(false),
                _ => {
                    return Err(CodeExecError::new(
                        &ctx.borrow(),
                        format!("Invalid var, op for mk: {:?} {:?}", rhs, self.op),
                    ))
                }
            },
        )?;
        Ok(None)
    }
}
