use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::statement::CodeExecError;

use super::{context::Context, node::Node, variable::VarType};

pub const LEN_KEY: &'static str = "len";

#[derive(Debug)]
pub enum MemData {
    Mess(Mess),
    Object(Object),
    Node(Node),
}
pub type MemDataRc = Rc<RefCell<MemData>>;

impl MemData {
    pub fn new_rc(data: MemData) -> MemDataRc {
        Rc::new(RefCell::new(data))
    }

    pub fn set(&mut self, ctx: &Context, key: &str, val: VarType) -> Result<(), CodeExecError> {
        match self {
            MemData::Mess(mess) => Ok(mess.set(key, val)),
            MemData::Object(obj) => Ok(obj.set(key, val)),
            MemData::Node(_node) => Err(CodeExecError::new(
                ctx,
                format!("Cannot set key {} on node.", key),
            )),
        }
    }

    pub fn get(&self, key: &str) -> VarType {
        match self {
            MemData::Mess(mess) => {
                if let Some(var) = mess.get(key) {
                    var
                } else {
                    VarType::Nzero
                }
            }
            MemData::Object(obj) => {
                if let Some(var) = obj.get(key) {
                    var
                } else {
                    VarType::Nzero
                }
            }
            MemData::Node(_node) => VarType::Nzero,
        }
    }

    pub fn has(&self, key: &str) -> bool {
        match self {
            MemData::Mess(mess) => mess.has(key),
            MemData::Object(obj) => obj.has(key),
            MemData::Node(_node) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mess {
    members: HashMap<String, VarType>,
}

impl Mess {
    pub fn new() -> Self {
        Mess {
            members: HashMap::new(),
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.members.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<VarType> {
        if let Some(var) = self.members.get(name) {
            return Some(var.clone());
        } else {
            None
        }
    }

    pub fn set(&mut self, name: &str, var: VarType) {
        self.members.insert(name.to_string(), var);
    }
}

#[derive(Debug, Clone)]
pub struct Tuple {
    pub items: Vec<VarType>,
}

#[derive(Debug, Clone)]
pub struct Object {
    members: HashMap<String, VarType>,
}

impl Object {
    pub fn new() -> Self {
        Object {
            members: HashMap::new(),
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.members.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<VarType> {
        self.members.get(name).cloned()
    }

    pub fn set(&mut self, name: &str, var: VarType) {
        self.members.insert(name.to_string(), var);
    }

    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.members.keys().cloned().collect();
        keys.sort();
        keys
    }
}

impl Tuple {
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
