use std::collections::HashMap;

use crate::statement::CodeExecError;

use super::{context::Context, node::Node, variable::VarType};

#[derive(Debug)]
pub enum MemData {
    Mess(Mess),
    Array(Array),
    Node(Node),
}

impl MemData {
    pub fn set(&mut self, ctx: &Context, key: &str, val: VarType) -> Result<(), CodeExecError> {
        match self {
            MemData::Mess(mess) => Ok(mess.set(key, val)),
            MemData::Array(array) => {
                if array.set(key, val) {
                    Ok(())
                } else {
                    Err(CodeExecError::new(
                        ctx,
                        format!("Cannot set key {} on array.", key),
                    ))
                }
            }
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
            MemData::Array(array) => {
                if let Some(var) = array.get(key) {
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
            MemData::Array(array) => array.has(key),
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
pub struct Array {
    pub items: Vec<VarType>,
}

impl Array {
    pub fn has(&self, name: &str) -> bool {
        if let Ok(index) = name.parse::<usize>() {
            index < self.items.len()
        } else {
            false
        }
    }

    pub fn get(&self, name: &str) -> Option<VarType> {
        if let Ok(index) = name.parse::<usize>() {
            if index < self.items.len() {
                return Some(self.items[index].clone());
            }
        }
        None
    }

    pub fn set(&mut self, name: &str, var: VarType) -> bool {
        if let Ok(index) = name.parse::<usize>() {
            if index < self.items.len() {
                self.items[index] = var;
                return true;
            }
        }
        false
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
