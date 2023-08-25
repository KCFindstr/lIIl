use crate::{
    data::{
        context::{Context, ContextRc},
        data::{MemData, Mess},
        variable::VarType,
    },
    statement::{CodeExecError, Statement, Statements},
    utils::path::Path,
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
            Module::Code(module) => Ok(module.exec()?.unwrap_or(VarType::Nzero)),
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
    pub path: Path,
    pub ctx: ContextRc,
    pub stmts: Statements,
}

impl CodeModule {
    pub fn new(name: &str, path: &str, parent: &ContextRc) -> Self {
        CodeModule {
            name: name.to_string(),
            path: Path::new(path),
            ctx: Context::new_rc(parent),
            stmts: Statements::new(),
        }
    }
    pub fn exec(&self) -> Result<Option<VarType>, CodeExecError> {
        self.stmts.exec(&self.ctx)
    }
}

pub trait IModule {
    fn exec(&self, ctx: &ContextRc) -> Result<Mess, CodeExecError>;
}

pub struct NativeModule {
    pub name: String,
    pub path: Path,
    pub ctx: ContextRc,
    module: Box<dyn IModule>,
}

impl NativeModule {
    pub fn new(name: &str, path: &str, parent: &ContextRc, module: Box<dyn IModule>) -> Self {
        NativeModule {
            name: name.to_string(),
            path: Path::new(path),
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
