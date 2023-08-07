use std::collections::HashMap;

use crate::{builtin::Module, variable::VarMess};

pub struct Context {
    pub parent: Option<*const Context>,
    pub vars: VarMess,
    pub modules: HashMap<String, Module>,
}

unsafe impl Sync for Context {}

impl Context {
    pub fn new() -> Context {
        Context {
            parent: None,
            vars: VarMess::new(),
            modules: HashMap::new(),
        }
    }
}

impl Context {
    pub fn add_mod(&mut self, name: &str, value: Module) {
        self.modules.insert(name.to_string(), value);
    }

    pub fn get_mod(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }

    pub fn has_mod(&self, name: &str) -> bool {
        self.get_mod(name).is_some()
    }

    pub fn set_parent(&mut self, parent: &Context) {
        self.parent = Some(parent as *const Context);
    }
}
