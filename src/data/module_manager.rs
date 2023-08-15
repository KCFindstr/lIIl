use std::collections::HashMap;

use crate::module::{
    cpu::{self, CpuModule},
    Module,
};

use super::context::ContextRc;

pub struct ModuleFactory {
    pub abs_path: String,
    pub factory: Box<dyn Fn(&ContextRc) -> Module>,
}

pub struct ModuleFactoryManager {
    factories: HashMap<String, ModuleFactory>,
}

impl ModuleFactoryManager {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn add_factory(&mut self, factory: ModuleFactory) {
        self.factories.insert(factory.abs_path.clone(), factory);
    }

    pub fn get_factory(&self, abs_path: &str) -> Option<&ModuleFactory> {
        self.factories.get(abs_path)
    }

    pub fn has_factory(&self, abs_path: &str) -> bool {
        self.factories.contains_key(abs_path)
    }
}

pub fn register_builtin_modules(manager: &mut ModuleFactoryManager) {
    CpuModule::register(manager);
}
