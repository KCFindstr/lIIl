use std::{collections::HashMap, rc::Rc};

use crate::module::{cpu::CpuModule, test::TestModule, Module};

use super::context::ContextRc;

pub type FactoryFn = Rc<dyn Fn(&ContextRc) -> Module>;

struct ModuleFactory {
    pub name: String,
    pub factory: FactoryFn,
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

    pub fn add_factory(&mut self, name: &str, factory: FactoryFn) {
        self.factories.insert(
            name.to_owned(),
            ModuleFactory {
                name: name.to_owned(),
                factory,
            },
        );
    }

    pub fn get_factory(&self, name: &str) -> Option<FactoryFn> {
        self.factories.get(name).map(|f| f.factory.clone())
    }
}

pub fn register_builtin_modules(manager: &mut ModuleFactoryManager) {
    CpuModule::register(manager);
    TestModule::register(manager);
}
