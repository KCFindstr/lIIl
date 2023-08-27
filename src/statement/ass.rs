use crate::{
    data::{context::ContextRc, lvalue::LValue, variable::VarType},
    expr::Expr,
};

use super::CodeExecError;

#[derive(Debug, Clone)]
pub struct AssStatement {
    pub lhs: LValue,
    pub rhs: Expr,
}

impl AssStatement {
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        let rhs = self.rhs.eval(ctx)?;
        self.lhs.set(ctx, rhs)?;
        Ok(None)
    }
}
