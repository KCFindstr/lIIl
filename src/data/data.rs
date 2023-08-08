use std::collections::HashMap;

use super::{context::SymbolProvider, node::Node, variable::VarType};

pub enum MemData {
    Mess(Mess),
    Array(Array),
    Node(Node),
}

#[derive(Debug, Clone)]
pub struct Mess {
    members: HashMap<String, VarType>,
}

impl Mess {
    pub fn new() -> Mess {
        Mess {
            members: HashMap::new(),
        }
    }
}

impl SymbolProvider<VarType> for Mess {
    fn has(&self, name: &str) -> bool {
        self.members.contains_key(name)
    }

    fn get(&self, name: &str) -> Option<&VarType> {
        self.members.get(name)
    }

    fn set(&mut self, name: &str, value: VarType) {
        if let Some(var) = self.members.get_mut(name) {
            *var = value;
        } else {
            self.members.insert(name.to_string(), value);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    pub items: Vec<VarType>,
}
