use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    fmt,
    ops::Deref,
};

use crate::statement::{CodeExecError, Statement};

use super::{
    context::{Context, SymbolProvider},
    variable::VarType,
};

#[derive(Clone)]
pub enum Node {
    Code(CodeNode),
    Native(NativeNode),
}

impl Node {
    pub fn exec(
        &self,
        parent: &RefCell<Context>,
        args: &Vec<VarType>,
    ) -> Result<VarType, CodeExecError> {
        match self {
            Node::Code(node) => node.exec(parent, args),
            Node::Native(node) => node.exec(&mut parent.borrow_mut(), args),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node").finish()
    }
}

#[derive(Debug, Clone)]
pub struct CodeNode {
    args: Vec<String>,
    body: Vec<Statement>,
}

impl CodeNode {
    pub fn exec(
        &self,
        parent: &RefCell<Context>,
        args: &Vec<VarType>,
    ) -> Result<VarType, CodeExecError> {
        let mut ctx = Context::new(&parent);
        if args.len() > self.args.len() {
            return Err(CodeExecError::new(
                &parent.borrow(),
                format!(
                    "Too many arguments for function: expected {}, got {}",
                    self.args.len(),
                    args.len()
                ),
            ));
        }
        for (value, name) in args.iter().zip(&self.args) {
            ctx.symbols.set(&name, value.clone());
        }
        for stmt in &self.body {
            if let Some(ret) = stmt.exec(&ctx)? {
                return Ok(ret);
            }
        }
        Ok(VarType::Nzero)
    }
}

#[derive(Clone)]
pub struct NativeNode {
    func: fn(parent: &mut Context, args: &Vec<VarType>) -> Result<VarType, CodeExecError>,
}

impl NativeNode {
    fn exec(&self, parent: &mut Context, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        (self.func)(parent, args)
    }

    pub fn as_vartype(
        func: fn(parent: &mut Context, args: &Vec<VarType>) -> Result<VarType, CodeExecError>,
    ) -> VarType {
        VarType::Node(Node::Native(NativeNode { func }))
    }
}
