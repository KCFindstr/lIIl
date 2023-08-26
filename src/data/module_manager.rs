use std::collections::HashMap;

use crate::module::{cpu::CpuModule, Module};

use super::context::ContextRc;

pub struct ModuleFactory {
    pub name: String,
    pub factory: Box<dyn Fn(&ContextRc) -> Module>,
}

impl ModuleFactory {
    pub fn create(&self, parent: &ContextRc) -> Module {
        (self.factory)(parent)
    }
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
        self.factories.insert(factory.name.clone(), factory);
    }

    pub fn get_factory(&self, name: &str) -> Option<&ModuleFactory> {
        self.factories.get(name)
    }

    pub fn has_factory(&self, name: &str) -> bool {
        self.factories.contains_key(name)
    }
}

pub fn register_builtin_modules(manager: &mut ModuleFactoryManager) {
    CpuModule::register(manager);
}
