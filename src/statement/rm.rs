use crate::{
    data::{context::ContextRc, variable::VarType},
    module::Module,
};

use super::CodeExecError;

#[derive(Clone, Debug)]
pub struct RmStatement {
    name: String,
    abs_path: String,
}

impl RmStatement {
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        let global = ctx.borrow_mut().get_global();
        let global = global.borrow();
        let module_path = Module::MODULE_SYMBOL_PREFIX.to_owned() + &self.abs_path;
        if let Some(symbol) = ctx.borrow().get_symbol(&module_path) {
            return Ok(Some(symbol));
        }
        if let Some(factory) = global.builtin_modules.get_factory(&self.abs_path) {
            let module = (factory.factory.as_ref())(&ctx.borrow().get_root());
            ctx.borrow_mut().set_symbol(&self.name, module.exec()?);
            Ok(None)
        } else {
            return Err(CodeExecError::new(
                &ctx.borrow(),
                format!("module {} not found", self.abs_path),
            ));
        }
    }
}
