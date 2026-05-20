use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use rand::Rng;

use super::{
    context::{ContextRc, GlobalRc},
    data::{MemData, MemDataRc},
    module::{register_builtin_modules, ModuleFactoryManager},
    stack::ProgramStack,
};

pub struct Global {
    pub context_root: Option<ContextRc>,
    pub builtin_modules: ModuleFactoryManager,
    pub stack: ProgramStack,
    pub objects: Vec<Weak<RefCell<MemData>>>,
}

impl Global {
    pub fn new() -> Self {
        let mut builtin_modules = ModuleFactoryManager::new();
        register_builtin_modules(&mut builtin_modules);
        Global {
            context_root: None,
            builtin_modules,
            stack: ProgramStack::new(),
            objects: Vec::new(),
        }
    }

    pub fn new_rc() -> GlobalRc {
        Rc::new(RefCell::new(Global::new()))
    }

    pub fn register_object(&mut self, rc: &MemDataRc) {
        self.objects.retain(|w| w.upgrade().is_some());
        self.objects.push(Rc::downgrade(rc));
    }

    pub fn random_other_object(&self, exclude: &MemDataRc) -> Option<MemDataRc> {
        let candidates: Vec<MemDataRc> = self
            .objects
            .iter()
            .filter_map(|w| w.upgrade())
            .filter(|rc| !Rc::ptr_eq(rc, exclude) && matches!(*rc.borrow(), MemData::Object(_)))
            .collect();
        if candidates.is_empty() {
            return None;
        }
        let idx = rand::thread_rng().gen_range(0..candidates.len());
        Some(candidates[idx].clone())
    }
}
