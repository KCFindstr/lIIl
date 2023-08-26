use crate::{
    data::context::Context,
    data::{context::ContextRc, variable::VarType},
    expr::Expr,
};

use self::{ass::AssStatement, rm::RmStatement};

pub mod ass;
pub mod rm;

#[derive(Debug)]
pub struct CodeExecError {
    desc: String,
}

impl CodeExecError {
    pub fn new(_: &Context, desc: String) -> CodeExecError {
        CodeExecError { desc }
    }
    pub fn new_str(desc: String) -> CodeExecError {
        CodeExecError { desc }
    }
}

#[derive(Debug)]
pub enum Statement {
    Rm(RmStatement),
    Ass(AssStatement),
    Expr(ExprStatement),
    Stmts(Statements),
}

impl Statement {
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        match self {
            Statement::Rm(stmt) => stmt.exec(ctx),
            Statement::Ass(stmt) => stmt.exec(ctx),
            Statement::Expr(stmt) => stmt.exec(ctx),
            Statement::Stmts(stmt) => stmt.exec(ctx),
        }
    }
}

#[derive(Debug)]
pub struct Statements {
    pub stmts: Vec<Statement>,
}

impl Statements {
    pub fn new() -> Self {
        Statements { stmts: Vec::new() }
    }
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        for stmt in &self.stmts {
            if let Some(var) = stmt.exec(ctx)? {
                return Ok(Some(var));
            }
        }
        Ok(None)
    }
    pub fn push(&mut self, stmt: Statement) {
        self.stmts.push(stmt);
    }
}

#[derive(Debug)]
pub struct ExprStatement {
    pub expr: Expr,
}

impl ExprStatement {
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        self.expr.eval(ctx)?;
        Ok(None)
    }
}
