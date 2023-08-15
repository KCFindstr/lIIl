use crate::{
    data::{
        context::{Context, ContextRc},
        data::{MemData, Mess},
        variable::VarType,
    },
    statement::{CodeExecError, Statement},
};
pub mod cpu;

pub enum Module {
    Code(CodeModule),
    Native(NativeModule),
}

impl Module {
    pub const MODULE_SYMBOL_PREFIX: &'static str = "module >> ";
    pub fn exec(&self) -> Result<VarType, crate::statement::CodeExecError> {
        match self {
            Module::Code(module) => module.exec(),
            Module::Native(module) => module.exec(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Module::Code(module) => &module.name,
            Module::Native(module) => &module.name,
        }
    }
}

pub struct CodeModule {
    pub name: String,
    pub path: Vec<String>,
    pub ctx: ContextRc,
    pub stmts: Vec<Statement>,
}

impl CodeModule {
    pub fn new(name: &str, path: &str, parent: &ContextRc) -> Self {
        CodeModule {
            name: name.to_string(),
            path: path.split('/').map(|s| s.to_string()).collect(),
            ctx: Context::new_rc(parent),
            stmts: Vec::new(),
        }
    }
    pub fn exec(&self) -> Result<VarType, CodeExecError> {
        for stmt in &self.stmts {
            if let Some(var) = stmt.exec(&self.ctx)? {
                return Ok(var);
            }
        }
        Ok(VarType::Nzero)
    }
}

pub trait IModule {
    fn exec(&self, ctx: &ContextRc) -> Result<Mess, CodeExecError>;
}

pub struct NativeModule {
    pub name: String,
    pub path: String,
    pub ctx: ContextRc,
    module: Box<dyn IModule>,
}

impl NativeModule {
    pub fn new(name: &str, path: &str, parent: &ContextRc, module: Box<dyn IModule>) -> Self {
        NativeModule {
            name: name.to_string(),
            path: path.to_string(),
            ctx: Context::new_rc(&parent.borrow().get_root()),
            module,
        }
    }
    pub fn exec(&self) -> Result<VarType, CodeExecError> {
        let mess = self.module.exec(&self.ctx)?;
        let ctx = self.ctx.borrow();
        let global = ctx.get_global();
        let mut global = global.borrow_mut();
        let id = global.data.add(MemData::Mess(mess));
        return Ok(VarType::Ref(id));
    }
}
