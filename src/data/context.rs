use std::{cell::RefCell, rc::Rc};

use crate::statement::CodeExecError;

use super::{
    data::{MemData, MemDataRc, Mess},
    global::Global,
    variable::VarType,
};

pub type ContextRc = Rc<RefCell<Context>>;
pub type GlobalRc = Rc<RefCell<Global>>;

pub struct Context {
    global: GlobalRc,
    parent: Option<ContextRc>,
    mess: MemDataRc,
}

impl Context {
    pub fn new(parent: &ContextRc) -> Self {
        let global = parent.borrow().global.clone();
        let mess = MemData::new_rc(MemData::Mess(Mess::new()));
        Context {
            global,
            parent: Some(parent.clone()),
            mess,
        }
    }
    pub fn new_rc(parent: &ContextRc) -> ContextRc {
        Rc::new(RefCell::new(Context::new(parent)))
    }
    pub fn root(global: &GlobalRc) -> Self {
        let mess = MemData::new_rc(MemData::Mess(Mess::new()));
        Context {
            global: global.clone(),
            parent: None,
            mess,
        }
    }
    pub fn root_rc() -> ContextRc {
        let global = Global::new_rc();
        let ret = Rc::new(RefCell::new(Context::root(&global)));
        global.borrow_mut().context_root = Some(ret.clone());
        return ret;
    }
    fn enter(ctx: &ContextRc) {
        ctx.borrow()
            .get_global()
            .borrow_mut()
            .stack
            .push(ctx.clone())
    }
    fn exit(ctx: &ContextRc) {
        let poped = ctx
            .borrow()
            .get_global()
            .borrow_mut()
            .stack
            .pop()
            .expect("Context stack is empty.");
        assert!(Rc::ptr_eq(&poped, ctx), "Context stack is corrupted.");
    }
    pub fn with<T>(ctx: &ContextRc, f: impl FnOnce() -> T) -> T {
        Context::enter(ctx);
        let ret = f();
        Context::exit(ctx);
        return ret;
    }
}

impl Context {
    pub fn get_mess(&self) -> MemDataRc {
        self.mess.clone()
    }

    pub fn get_global(&self) -> GlobalRc {
        self.global.clone()
    }

    pub fn get_root(&self) -> ContextRc {
        self.get_global().borrow().context_root.to_owned().unwrap()
    }

    pub fn get_symbol(&self, name: &str) -> Option<VarType> {
        let data_item = self.get_mess();
        if data_item.borrow().has(&name) {
            return Some(data_item.borrow().get(&name));
        }
        if let Some(parent) = &self.parent {
            parent.borrow().get_symbol(name)
        } else {
            None
        }
    }

    pub fn has_symbol(&self, name: &str) -> bool {
        let data_item = self.get_mess();
        if data_item.borrow().has(&name) {
            return true;
        }
        if let Some(parent) = &self.parent {
            parent.borrow().has_symbol(name)
        } else {
            false
        }
    }

    pub fn set_symbol(&self, name: &str, value: VarType) {
        if self.has_symbol(name) {
            let data_item = self.get_mess();
            if data_item.borrow().has(&name) {
                data_item.borrow_mut().set(&self, &name, value).unwrap();
                return;
            }
            if let Some(parent) = &self.parent {
                parent.borrow().set_symbol(name, value)
            } else {
                panic!("Symbol {} not found.", name.to_string())
            }
        } else {
            self.get_mess()
                .borrow_mut()
                .set(&self, &name, value)
                .unwrap();
        }
    }

    pub fn get_symbol_or_err(&self, name: &str) -> Result<VarType, CodeExecError> {
        if let Some(item) = self.get_symbol(name) {
            Ok(item)
        } else {
            Err(CodeExecError::new(
                &self,
                format!("Symbol {} not found.", name.to_string()),
            ))
        }
    }
}
