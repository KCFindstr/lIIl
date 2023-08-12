use std::cell::RefCell;

use crate::{
    data::{
        context::{Context, ContextRef},
        data::Mess,
    },
    statement::{CodeExecError, Statement},
};
pub mod cpu;

pub enum Module {
    Code(CodeModule),
    Native(NativeModule),
}

impl Module {
    pub fn exec(&self) -> Result<&Mess, crate::statement::CodeExecError> {
        match self {
            Module::Code(module) => module.exec(),
            Module::Native(module) => module.exec(),
        }
    }
    pub fn get_ctx_mut(&mut self) -> &mut Context {
        match self {
            Module::Code(module) => &mut module.ctx,
            Module::Native(module) => &mut module.ctx,
        }
    }
}

pub struct CodeModule {
    pub name: String,
    pub path: String,
    pub ctx: Context,
    pub stmts: Vec<Statement>,
}

impl CodeModule {
    pub fn new(name: &str, path: &str, parent: &ContextRef) -> Self {
        CodeModule {
            name: name.to_string(),
            path: path.to_string(),
            ctx: Context::new(parent),
            stmts: Vec::new(),
        }
    }
    pub fn exec(&self) -> Result<&Mess, CodeExecError> {
        for stmt in &self.stmts {
            stmt.exec(&self.ctx)?;
        }
        Ok(&self.ctx.symbols)
    }
}

pub trait MessProvider {
    fn get_mess(&self) -> &Mess;
}

pub struct NativeModule {
    pub name: String,
    pub path: String,
    pub ctx: Context,
    module: Box<dyn MessProvider>,
}

impl NativeModule {
    pub fn new(
        name: &str,
        path: &str,
        parent: &ContextRef,
        module: Box<dyn MessProvider>,
    ) -> NativeModule {
        NativeModule {
            name: name.to_string(),
            path: path.to_string(),
            ctx: Context::new(parent),
            module,
        }
    }
    pub fn exec(&self) -> Result<&Mess, CodeExecError> {
        Ok(self.module.get_mess())
    }
}
