use crate::{
    data::{context::ContextRc, variable::VarType},
    expr::Expr,
};

use super::{CodeExecError, Statement};

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub cond: Expr,
    pub body: Box<Statement>,
}

impl IfStatement {
    pub fn exec(&mut self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        let rhs: bool = self.cond.eval(ctx)?.into();
        if rhs {
            self.body.exec(&ctx)
        } else {
            Ok(None)
        }
    }
}
