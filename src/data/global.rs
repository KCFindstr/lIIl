use std::{cell::RefCell, rc::Rc};

use super::{
    context::{ContextRc, GlobalRc},
    module_manager::{register_builtin_modules, ModuleFactoryManager},
    stack::ProgramStack,
};

pub struct Global {
    pub context_root: Option<ContextRc>,
    pub builtin_modules: ModuleFactoryManager,
    pub stack: ProgramStack,
}

impl Global {
    pub fn new() -> Self {
        let mut builtin_modules = ModuleFactoryManager::new();
        register_builtin_modules(&mut builtin_modules);
        Global {
            context_root: None,
            builtin_modules,
            stack: ProgramStack::new(),
        }
    }

    pub fn new_rc() -> GlobalRc {
        Rc::new(RefCell::new(Global::new()))
    }
}
