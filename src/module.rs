use crate::{
    data::{
        context::{Context, ContextRc},
        data::{MemData, Mess},
        variable::VarType,
    },
    statement::{CodeExecError, Statements},
    utils::path::Path,
};
pub mod cpu;

pub enum Module {
    Code(CodeModule),
    Native(NativeModule),
}

impl Module {
    const BUILTIN_MODULE_PREFIX: &'static str = "<module>";
    const LIIL_EXT: &'static str = ".lIIl";

    pub fn builtin_path(name: &str) -> String {
        return Module::BUILTIN_MODULE_PREFIX.to_owned() + "/" + name;
    }

    pub fn code_path(path_wo_ext: &str) -> String {
        return path_wo_ext.to_owned() + Module::LIIL_EXT;
    }

    pub fn exec(&mut self) -> Result<VarType, crate::statement::CodeExecError> {
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
    pub fn exec(&mut self) -> Result<VarType, CodeExecError> {
        self.stmts.exec(&self.ctx)?;
        return Ok(VarType::Ref(self.ctx.borrow().get_symbol_mess_id()));
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
        return Ok(self.ctx.borrow().add_mem(MemData::Mess(mess)));
    }
}
