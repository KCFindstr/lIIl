use std::{cell::RefCell, rc::Rc};

use crate::statement::CodeExecError;

use super::{
    data::{MemData, Mess},
    global::Global,
    variable::VarType,
};

type ContextRef = Rc<RefCell<Context>>;

pub trait SymbolProvider<T>
where
    T: Default,
{
    fn get(&self, name: &str) -> Option<&T>;
    fn set(&mut self, name: &str, value: T);
    fn has(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
    fn get_or_add(&mut self, name: &str) -> &T {
        if self.has(name) {
            return self.get(name).unwrap();
        }
        self.set(name, T::default());
        self.get(name).unwrap()
    }
    fn get_or_err(&self, ctx: &Context, name: &str) -> Result<&T, CodeExecError> {
        match self.get(name) {
            Some(var) => Ok(var),
            None => Err(CodeExecError::new(
                ctx,
                format!("Symbol {} not found", name),
            )),
        }
    }
}

pub struct Context {
    pub global: Rc<RefCell<Global>>,
    pub parent: Rc<Option<RefCell<Context>>>,
    pub symbols: Mess,
}

unsafe impl Sync for Context {}

impl Context {
    pub fn new(parent: &RefCell<Context>) -> Context {
        let global = parent.borrow().global.clone();
        Context {
            global,
            parent: Rc::new(Some(*parent.clone())),
            symbols: Mess::new(),
        }
    }
    pub fn root() -> Context {
        Context {
            global: Rc::new(RefCell::new(Global::new())),
            parent: Rc::new(None),
            symbols: Mess::new(),
        }
    }
}

impl Context {
    pub fn get_mem(&self, name: &str) -> Option<&MemData> {
        let item = self.symbols.get(name);
        if let Some(item) = item {
            if let VarType::Ref(var_ref) = item {
                self.global.borrow().data.get(var_ref.id)
            } else {
                panic!("Expected reference type, got {:?}", item)
            }
        } else if let Some(parent) = &*self.parent {
            Some(parent.borrow().get_mem(name).unwrap())
        } else {
            None
        }
    }
}

impl SymbolProvider<VarType> for Context {
    fn get(&self, name: &str) -> Option<&VarType> {
        if let Some(item) = self.symbols.get(name) {
            Some(item)
        } else if let Some(parent) = &*self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }

    fn has(&self, name: &str) -> bool {
        if self.symbols.has(name) {
            true
        } else if let Some(parent) = &*self.parent {
            parent.borrow().has(name)
        } else {
            false
        }
    }

    fn set(&mut self, name: &str, value: VarType) {
        if self.symbols.has(name) {
            self.symbols.set(name, value);
        } else if let Some(parent) = &*self.parent {
            parent.borrow_mut().set(name, value);
        }
    }
}
