use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use crate::{
    data::context::Context,
    module::Node,
    statement::{CodeExecError, Statement},
};

pub trait SymbolProvider {
    fn get(&self, name: &str) -> Option<&VarType>;
    fn add(&mut self, name: &str, value: VarType) -> &VarType;
    fn set(&mut self, name: &str, value: VarType) -> Option<&VarType>;
    fn has(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
    fn get_or_add(&mut self, name: &str) -> &VarType {
        if self.has(name) {
            return self.get(name).unwrap();
        }
        self.add(name, VarType::Nzero);
        self.get(name).unwrap()
    }
    fn set_or_add(&mut self, name: &str, value: VarType) -> &VarType {
        if self.has(name) {
            return self.set(name, value).unwrap();
        }
        return self.add(name, value);
    }
    fn get_or_err(&self, ctx: &Context, name: &str) -> Result<&VarType, CodeExecError> {
        match self.get(name) {
            Some(var) => Ok(var),
            None => Err(CodeExecError::new(
                ctx,
                format!("Symbol {} not found", name),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VarMess {
    members: HashMap<String, VarType>,
}

impl VarMess {
    pub fn new() -> VarMess {
        VarMess {
            members: HashMap::new(),
        }
    }
}

impl SymbolProvider for VarMess {
    fn get(&self, name: &str) -> Option<&VarType> {
        self.members.get(name)
    }

    fn add(&mut self, name: &str, value: VarType) -> &VarType {
        self.members.insert(name.to_string(), value);
        self.members.get(name).unwrap()
    }

    fn set(&mut self, name: &str, value: VarType) -> Option<&VarType> {
        match self.members.get_mut(name) {
            Some(var) => {
                *var = value;
                Some(var)
            }
            None => None,
        }
    }
}

#[derive(Clone)]
pub struct VarNode {
    args: Vec<String>,
    body: Vec<Statement>,
}

impl VarNode {
    pub fn exec(&self, parent: &Context, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        let mut ctx = Context::new();
        if args.len() > self.args.len() {
            return Err(CodeExecError::new(
                parent,
                format!(
                    "Too many arguments for function: expected {}, got {}",
                    self.args.len(),
                    args.len()
                ),
            ));
        }
        for (value, name) in args.iter().zip(&self.args) {
            ctx.vars.add(&name, value.clone());
        }
        for stmt in &self.body {
            if let Some(ret) = stmt.exec(&ctx)? {
                return Ok(ret);
            }
        }
        Ok(VarType::Nzero)
    }
}

#[derive(Debug, Clone)]
pub struct VarArray {
    pub items: Vec<VarType>,
}

#[derive(Debug, Clone)]
pub struct VarRef {
    pub id: i64,
}

#[derive(Debug, Clone)]
pub enum VarType {
    Nzero,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Mess(VarMess),
    Node(Node),
    Array(VarArray),
    Tuple(VarArray),
    Ref(VarRef),
}

impl Display for VarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VarType::Nzero => write!(f, "N0"),
            VarType::Bool(val) => write!(f, "{}", val),
            VarType::Int(val) => write!(f, "{}", val),
            VarType::Float(val) => write!(f, "{}", val),
            VarType::String(val) => write!(f, "{}", val),
            VarType::Mess(val) => write!(f, "{:?}", val),
            VarType::Node(val) => write!(f, "{:?}", val),
            VarType::Array(val) => write!(f, "{:?}", val),
            VarType::Tuple(val) => write!(f, "{:?}", val),
            VarType::Ref(val) => write!(f, "Ref({})", val.id),
        }
    }
}
