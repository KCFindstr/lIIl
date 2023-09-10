use crate::{
    data::{context::ContextRc, variable::VarType},
    expr::Expr,
};

use super::CodeExecError;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Expr,
}

impl ReturnStatement {
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        Ok(Some(self.value.eval(ctx)?))
    }
}
