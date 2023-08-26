use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    context::{ContextRc, GlobalRc},
    data::MemData,
    module_manager::{register_builtin_modules, ModuleFactoryManager},
    stack::ProgramStack,
};

#[derive(Debug)]
pub struct DataItem {
    pub data: MemData,
}

impl DataItem {
    pub fn new(data: MemData) -> DataItem {
        DataItem { data }
    }
}

pub type DataItemRc = Rc<RefCell<DataItem>>;

pub struct GlobalData {
    pub variables: HashMap<i64, DataItemRc>,
    next_id: i64,
}

impl GlobalData {
    pub fn new() -> Self {
        GlobalData {
            variables: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add(&mut self, data: MemData) -> i64 {
        let id = self.next_id();
        let item = DataItem::new(data);
        self.variables.insert(id, Rc::new(RefCell::new(item)));
        return id;
    }

    pub fn get(&self, id: i64) -> Option<DataItemRc> {
        self.variables.get(&id).and_then(|v| Some(v.clone()))
    }

    fn next_id(&mut self) -> i64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

pub struct Global {
    pub context_root: Option<ContextRc>,
    pub data: GlobalData,
    pub builtin_modules: ModuleFactoryManager,
    pub stack: ProgramStack,
}

impl Global {
    pub fn new() -> Self {
        let mut builtin_modules = ModuleFactoryManager::new();
        register_builtin_modules(&mut builtin_modules);
        Global {
            context_root: None,
            data: GlobalData::new(),
            builtin_modules,
            stack: ProgramStack::new(),
        }
    }

    pub fn new_rc() -> GlobalRc {
        Rc::new(RefCell::new(Global::new()))
    }
}
