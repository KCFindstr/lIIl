use std::{cell::RefCell, rc::Rc};

use crate::{statement::CodeExecError, utils::path::Path};

use super::{
    data::{self, MemData, Mess},
    global::{DataItemRc, Global},
    variable::VarType,
};

pub type ContextRc = Rc<RefCell<Context>>;
pub type GlobalRc = Rc<RefCell<Global>>;

pub struct Context {
    global: GlobalRc,
    pub parent: Option<ContextRc>,
    pub messId: i64,
}

impl Context {
    pub fn new(parent: &ContextRc) -> Self {
        let global = parent.borrow().global.clone();
        let messId = global.borrow_mut().data.add(MemData::Mess(Mess::new()));
        Context {
            global,
            parent: Some(parent.clone()),
            messId,
        }
    }
    pub fn new_rc(parent: &ContextRc) -> ContextRc {
        Rc::new(RefCell::new(Context::new(parent)))
    }
    pub fn root(global: &GlobalRc) -> Self {
        let messId = global.borrow_mut().data.add(MemData::Mess(Mess::new()));
        Context {
            global: global.clone(),
            parent: None,
            messId,
        }
    }
    pub fn root_rc() -> ContextRc {
        let global = Global::new_rc();
        let ret = Rc::new(RefCell::new(Context::root(&global)));
        global.borrow_mut().context_root = Some(ret.clone());
        return ret;
    }
}

impl Context {
    fn get_data_item(&self) -> DataItemRc {
        self.global.borrow().data.get(self.messId).unwrap()
    }

    pub fn get_global(&self) -> GlobalRc {
        self.global.clone()
    }

    pub fn get_root(&self) -> ContextRc {
        self.get_global().borrow().context_root.to_owned().unwrap()
    }

    pub fn get_mem(&self, name: &str) -> Option<DataItemRc> {
        let item = self.get_symbol(name);
        if let Some(item) = item {
            if let VarType::Ref(var_ref) = item {
                self.global.borrow().data.get(var_ref)
            } else {
                panic!("Expected reference type, got {:?}", item)
            }
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().get_mem(name)
        } else {
            None
        }
    }

    pub fn add_mem(&self, data: MemData) -> VarType {
        VarType::Ref(self.get_global().borrow_mut().data.add(data))
    }

    pub fn get_symbol(&self, name: &str) -> Option<VarType> {
        let data_item = self.get_data_item();
        if let MemData::Mess(mess) = &data_item.borrow().data {
            if let Some(item) = mess.get(name) {
                return Some(item);
            } else if let Some(parent) = &self.parent {
                return parent.borrow().get_symbol(name);
            }
        }
        None
    }

    pub fn get_symbol_mess_id(&self) -> i64 {
        return self.messId;
    }

    pub fn has_symbol(&self, name: &str) -> bool {
        let data_item = self.get_data_item();
        if let MemData::Mess(mess) = &data_item.borrow().data {
            if mess.has(name) {
                return true;
            } else if let Some(parent) = &self.parent {
                return parent.borrow().has_symbol(name);
            }
        }
        false
    }

    pub fn set_symbol(&self, name: &str, value: VarType) {
        let data_item = self.get_data_item();
        let mut borrowed_data_item = data_item.borrow_mut();
        if let MemData::Mess(mess) = &mut borrowed_data_item.data {
            if mess.has(name) {
                mess.set(name, value);
            } else if let Some(parent) = &self.parent {
                parent.borrow_mut().set_symbol(name, value);
            }
        }
    }

    pub fn get_symbol_or_err(&self, name: &str) -> Result<VarType, CodeExecError> {
        if let Some(item) = self.get_symbol(name) {
            Ok(item)
        } else {
            Err(CodeExecError::new(
                &self,
                format!("{} is not found.", name.to_string()),
            ))
        }
    }
}
