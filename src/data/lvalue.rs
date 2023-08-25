use crate::{expr::MemberExpr, statement::CodeExecError};

use super::{context::ContextRc, variable::VarType};

pub enum LValue {
    Identifier(String),
    MemberExpr(MemberExpr),
}

impl LValue {
    pub fn set(&mut self, ctx: &ContextRc, val: VarType) -> Result<(), CodeExecError> {
        match self {
            LValue::Identifier(id) => Ok(ctx.borrow_mut().set_symbol(&id, val)),
            LValue::MemberExpr(expr) => expr.set(ctx, val),
        }
    }
}
