use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::statement::CodeExecError;

use super::{
    data::{MemData, Mess},
    global::{DataItemRef, Global},
    variable::VarType,
};

pub type ContextRef = Rc<RefCell<Context>>;
pub type GlobalRef = Rc<RefCell<Global>>;
pub type VarTypeRef = Rc<RefCell<VarType>>;

pub struct Context {
    pub global: GlobalRef,
    pub parent: Option<ContextRef>,
    pub symbols: Mess,
}

impl Context {
    pub fn new(parent: &ContextRef) -> Self {
        let global = parent.borrow().global.clone();
        Context {
            global,
            parent: Some(parent.clone()),
            symbols: Mess::new(),
        }
    }
    pub fn root() -> Self {
        Context {
            global: Rc::new(RefCell::new(Global::new())),
            parent: None,
            symbols: Mess::new(),
        }
    }
}

impl Context {
    pub fn get_mem(&mut self, name: &str) -> Option<DataItemRef> {
        let item = self.get_symbol(name);
        if let Some(item) = item {
            if let VarType::Ref(var_ref) = &*item.borrow() {
                let mut borrowed_global = self.global.borrow_mut();
                borrowed_global.data.get(var_ref.id)
            } else {
                panic!("Expected reference type, got {:?}", *item)
            }
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().get_mem(name)
        } else {
            None
        }
    }

    pub fn get_symbol(&self, name: &str) -> Option<VarTypeRef> {
        if let Some(item) = self.symbols.get(name) {
            None
        } else if let Some(parent) = &self.parent {
            parent.borrow().get_symbol(name)
        } else {
            None
        }
    }

    pub fn has_symbol(&self, name: &str) -> bool {
        if self.symbols.has(name) {
            true
        } else if let Some(parent) = &self.parent {
            parent.borrow().has_symbol(name)
        } else {
            false
        }
    }

    pub fn set_symbol(&mut self, name: &str, value: VarType) {
        if self.symbols.has(name) {
            self.symbols.set(name, value);
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().set_symbol(name, value);
        }
    }

    pub fn get_symbol_or_err(
        &self,
        ctx: &Context,
        name: &str,
    ) -> Result<VarTypeRef, CodeExecError> {
        if let Some(item) = self.symbols.get(name) {
            Ok(item)
        } else if let Some(parent) = &self.parent {
            parent.borrow().get_symbol_or_err(ctx, name)
        } else {
            Err(CodeExecError::new(
                ctx,
                format!("{} is not found.", name.to_string()),
            ))
        }
    }
}
