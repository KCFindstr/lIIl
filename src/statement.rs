use crate::{context::Context, variable::VarType};

#[derive(Debug)]
pub struct CodeExecError {
    desc: String,
}

impl CodeExecError {
    pub fn new(_: &Context, desc: String) -> CodeExecError {
        CodeExecError { desc }
    }
}

#[derive(Clone)]
pub struct RmStatement {
    module_name: String,
}

impl RmStatement {
    fn exec(&self, _ctx: &Context) -> Result<Option<VarType>, CodeExecError> {
        // TODO: Implement this.
        Ok(None)
    }
}

#[derive(Clone)]
pub enum Statement {
    Rm(RmStatement),
}

impl Statement {
    pub fn exec(&self, ctx: &Context) -> Result<Option<VarType>, CodeExecError> {
        match self {
            Statement::Rm(stmt) => stmt.exec(ctx),
        }
    }
}
