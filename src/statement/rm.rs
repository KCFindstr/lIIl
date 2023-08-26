use crate::{
    data::{context::ContextRc, variable::VarType},
    module::{CodeModule, Module},
    parser::parse_file,
    utils::path::Path,
};

use super::CodeExecError;

#[derive(Clone, Debug)]
pub struct RmStatement {
    pub path: String,
    pub parent_path: Path,
}

impl RmStatement {
    pub fn new(module: &CodeModule, path: &str) -> Self {
        RmStatement {
            path: path.to_owned(),
            parent_path: module.path.clone(),
        }
    }
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        let global = ctx.borrow_mut().get_global();
        let module_path = Module::builtin_path(&self.path);

        // Module already exists.
        if ctx.borrow().has_symbol(&module_path) {
            return Ok(None);
        }

        // Built-in module.
        if let Some(factory) = global.borrow().builtin_modules.get_factory(&self.path) {
            let module = factory.create(&ctx.borrow().get_root());
            let module_ret = module.exec()?;
            ctx.borrow_mut().set_symbol(&self.path, module_ret);
            return Ok(None);
        }

        // Code module.
        let module_path = self
            .parent_path
            .relative(&Path::new(&Module::code_path(&self.path)))
            .as_std_path();
        if !module_path.is_file() {
            return Err(CodeExecError::new(
                &ctx.borrow(),
                format!("module {} not found", self.path),
            ));
        }
        let module = parse_file(
            module_path.to_str().unwrap(),
            Some(&ctx.borrow().get_root()),
        )?;
        let module_ret = module.exec()?;
        ctx.borrow_mut().set_symbol(&self.path, module_ret);
        module.exec()?;
        Ok(None)
    }
}
