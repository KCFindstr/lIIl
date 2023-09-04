use crate::{
    data::{context::ContextRc, variable::VarType},
    expr::Expr,
};

use super::{CodeExecError, Statement};

#[derive(Debug, Clone)]
pub struct LoliStatement {
    pub cond: Expr,
    pub body: Box<Statement>,
}

impl LoliStatement {
    #[inline]
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        loop {
            let rhs: bool = self.cond.eval(ctx)?.into();
            if !rhs {
                break;
            }
            if let Some(ret) = self.body.exec(&ctx)? {
                return Ok(Some(ret));
            }
        }
        Ok(None)
    }
}
