use std::collections::HashMap;

use crate::module::Module;

use super::context::ContextRc;

pub struct ModuleFactory {
    pub name: String,
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
        self.factories.insert(factory.name.clone(), factory);
    }

    pub fn get_factory(&self, name: &str) -> Option<&ModuleFactory> {
        self.factories.get(name)
    }

    pub fn has_factory(&self, name: &str) -> bool {
        self.factories.contains_key(name)
    }
}
