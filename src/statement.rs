use crate::{
    data::context::Context,
    data::{context::ContextRc, variable::VarType},
};

use self::{block::BlockStatement, rm::RmStatement};

mod block;
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
    Block(BlockStatement),
}

impl Statement {
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        match self {
            Statement::Rm(stmt) => stmt.exec(ctx),
            Statement::Block(stmt) => stmt.exec(ctx),
        }
    }
}
