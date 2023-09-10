use crate::{
    data::{context::ContextRc, variable::VarType},
    expr::Expr,
};

use super::CodeExecError;

#[derive(Debug, Clone)]
pub struct ExprStatement {
    pub value: Expr,
}

impl ExprStatement {
    const THAT: &'static str = "that";

    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        let value = self.value.eval(ctx)?;
        match value {
            VarType::Nzero => Ok(None),
            _ => {
                ctx.borrow().set_symbol(Self::THAT, value);
                Ok(None)
            }
        }
    }
}
