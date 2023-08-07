use std::fmt;

use crate::{
    builtin::Module,
    context::Context,
    statement::{CodeExecError, Statement},
    variable::{SymbolProvider, VarNode, VarType},
};

#[derive(Clone)]
pub enum Node {
    Var(VarNode),
    Native(NativeNode),
}

impl Node {
    pub fn exec(&self, parent: &Context, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        match self {
            Node::Var(node) => node.exec(parent, args),
            Node::Native(node) => node.exec(parent, args),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node").finish()
    }
}

pub struct CodeModule {
    pub name: String,
    pub path: String,
    pub stmts: Vec<Statement>,
    pub ctx: Context,
}

impl CodeModule {
    pub fn new(name: &str, path: &str) -> CodeModule {
        CodeModule {
            name: name.to_string(),
            path: path.to_string(),
            stmts: Vec::new(),
            ctx: Context::new(),
        }
    }
    pub fn exec(&self) -> Result<(), CodeExecError> {
        for stmt in &self.stmts {
            stmt.exec(&self.ctx);
        }
        Ok(())
    }
}

impl SymbolProvider for CodeModule {
    fn add(&mut self, name: &str, value: VarType) -> &VarType {
        self.ctx.vars.add(name, value)
    }

    fn get(&self, name: &str) -> Option<&VarType> {
        self.ctx.vars.get(name)
    }

    fn set(&mut self, name: &str, value: VarType) -> Option<&VarType> {
        self.ctx.vars.set(name, value)
    }
}

#[derive(Clone)]
pub struct NativeNode {
    func: fn(parent: &Context, args: &Vec<VarType>) -> Result<VarType, CodeExecError>,
}

impl NativeNode {
    fn exec(&self, parent: &Context, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        (self.func)(parent, args)
    }

    pub fn as_vartype(
        func: fn(parent: &Context, args: &Vec<VarType>) -> Result<VarType, CodeExecError>,
    ) -> VarType {
        VarType::Node(Node::Native(NativeNode { func }))
    }
}

fn register_module(
    parent: &mut Context,
    name: &str,
    mut module: Module,
) -> Result<(), CodeExecError> {
    if parent.get_mod(name).is_some() {
        return Ok(());
    }
    module.exec()?;
    let module_mut = &mut module;
    module_mut.get_ctx_mut().set_parent(parent);
    parent.add_mod(name, module);
    Ok(())
}
