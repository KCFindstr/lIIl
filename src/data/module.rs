use std::{collections::HashMap, rc::Rc};

use crate::module::{
    ai::AiModule, cpu::CpuModule, ll::LlModule, test::TestModule, tp::TpModule, Module,
};

use super::context::ContextRc;

pub type FactoryFn = Rc<dyn Fn(&ContextRc) -> Module>;

pub struct ModuleFactoryManager {
    factories: HashMap<String, FactoryFn>,
}

impl ModuleFactoryManager {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn add_factory(&mut self, name: &str, factory: FactoryFn) {
        self.factories.insert(name.to_owned(), factory);
    }

    pub fn get_factory(&self, name: &str) -> Option<FactoryFn> {
        self.factories.get(name).map(|f| f.clone())
    }
}

pub fn register_builtin_modules(manager: &mut ModuleFactoryManager) {
    AiModule::register(manager);
    CpuModule::register(manager);
    LlModule::register(manager);
    TestModule::register(manager);
    TpModule::register(manager);
}
