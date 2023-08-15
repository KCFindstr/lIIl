use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use super::{context::VarTypeRc, node::Node, variable::VarType};

pub enum MemData {
    Mess(Mess),
    Array(Array),
    Node(Node),
}

#[derive(Debug, Clone)]
pub struct Mess {
    members: HashMap<String, VarTypeRc>,
}

impl Mess {
    pub fn new() -> Self {
        Mess {
            members: HashMap::new(),
        }
    }
}

impl Mess {
    pub fn has(&self, name: &str) -> bool {
        self.members.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<VarTypeRc> {
        if let Some(var) = self.members.get(name) {
            return Some(var.clone());
        } else {
            None
        }
    }

    pub fn set(&mut self, name: &str, var: VarType) {
        self.members
            .insert(name.to_string(), Rc::new(RefCell::new(var)));
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    pub items: Vec<VarType>,
}
