use crate::data::{context::ContextRc, variable::VarType};

use super::{CodeExecError, Statement};

#[derive(Clone, Debug)]
pub struct BlockStatement {
    stmts: Vec<Statement>,
}

impl BlockStatement {
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        for stmt in &self.stmts {
            if let Some(var) = stmt.exec(&ctx)? {
                return Ok(Some(var));
            }
        }
        Ok(None)
    }
}
