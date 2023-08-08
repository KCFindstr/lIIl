use crate::{data::context::Context, data::variable::VarType};

#[derive(Debug)]
pub struct CodeExecError {
    desc: String,
}

impl CodeExecError {
    pub fn new(_: &Context, desc: String) -> CodeExecError {
        CodeExecError { desc }
    }
}

#[derive(Clone, Debug)]
pub struct RmStatement {
    module_name: String,
}

impl RmStatement {
    fn exec(&self, _ctx: &Context) -> Result<Option<VarType>, CodeExecError> {
        // TODO: Implement this.
        Ok(None)
    }
}

#[derive(Clone, Debug)]
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
