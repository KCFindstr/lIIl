use std::fmt::{self, Debug};

use crate::{expr::MemberExpr, statement::CodeExecError};

use super::{context::ContextRc, variable::VarType};

#[derive(Clone)]
pub enum LValue {
    Identifier(String),
    MemberExpr(MemberExpr),
}

impl Debug for LValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LValue::Identifier(id) => write!(f, "{}", id),
            LValue::MemberExpr(_expr) => write!(f, "MemberExpr"),
        }
    }
}

impl LValue {
    pub fn set(&self, ctx: &ContextRc, val: VarType) -> Result<(), CodeExecError> {
        match self {
            LValue::Identifier(id) => Ok(ctx.borrow().set_symbol(&id, val)),
            LValue::MemberExpr(expr) => expr.set(ctx, val),
        }
    }
}
