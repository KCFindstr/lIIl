use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{module::Module, statement::CodeExecError};

use super::{
    context::{Context, ContextRc, GlobalRc},
    data::MemData,
    module_manager::{ModuleFactory, ModuleFactoryManager},
    stack::ProgramStack,
};

#[derive(Debug)]
pub struct DataItem {
    pub data: MemData,
    ref_count: i64,
}

impl DataItem {
    pub fn new(data: MemData) -> DataItem {
        DataItem { data, ref_count: 0 }
    }

    pub fn add_ref(&mut self) {
        self.ref_count += 1;
    }

    // Returns whether ref count is zero.
    pub fn deref(&mut self) -> bool {
        self.ref_count -= 1;
        self.ref_count <= 0
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
        let mut item = DataItem::new(data);
        item.add_ref();
        self.variables.insert(id, Rc::new(RefCell::new(item)));
        return id;
    }

    pub fn get(&self, id: i64) -> Option<DataItemRc> {
        self.variables.get(&id).and_then(|v| Some(v.clone()))
    }

    pub fn get_or_err(&self, ctx: &Context, id: i64) -> Result<DataItemRc, CodeExecError> {
        self.get(id).ok_or(CodeExecError::new(
            ctx,
            format!("Variable {} not found", id),
        ))
    }

    pub fn obtain(&self, id: i64) -> Option<DataItemRc> {
        if let Some(data) = self.variables.get(&id) {
            data.borrow_mut().add_ref();
            Some(data.clone())
        } else {
            None
        }
    }

    pub fn obtain_or_err(&self, ctx: &Context, id: i64) -> Result<DataItemRc, CodeExecError> {
        self.obtain(id).ok_or(CodeExecError::new(
            ctx,
            format!("Variable {} not found", id),
        ))
    }

    pub fn release(&mut self, id: i64) {
        if let Some(data) = self.variables.get(&id) {
            if data.borrow_mut().deref() {
                self.variables.remove(&id);
            }
        }
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
        Global {
            context_root: None,
            data: GlobalData::new(),
            builtin_modules: ModuleFactoryManager::new(),
            stack: ProgramStack::new(),
        }
    }

    pub fn new_rc() -> GlobalRc {
        Rc::new(RefCell::new(Global::new()))
    }
}
