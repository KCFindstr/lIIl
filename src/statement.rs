use crate::{
    data::context::Context,
    data::{context::ContextRc, variable::VarType},
};

use self::rm::RmStatement;

mod rm;

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
pub enum Statement {
    Rm(RmStatement),
}

impl Statement {
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        match self {
            Statement::Rm(stmt) => stmt.exec(ctx),
        }
    }
}
