use std::{cell::RefCell, fmt, ops::Deref, rc::Rc};

use crate::statement::{CodeExecError, Statement};

use super::{
    context::{Context, ContextRc},
    variable::VarType,
};

#[derive(Clone)]
pub enum Node {
    Code(CodeNode),
    Native(NativeNode),
}

impl Node {
    pub fn exec(&self, parent: &ContextRc, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        match self {
            Node::Code(node) => node.exec(parent, args),
            Node::Native(node) => node.exec(parent, args),
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
    pub fn exec(&self, parent: &ContextRc, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        let ctx = Context::new_rc(parent);
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
            ctx.borrow_mut().symbols.set(&name, value.clone());
        }
        for stmt in &self.body {
            stmt.exec(&ctx)?;
        }
        Ok(VarType::Nzero)
    }
}

#[derive(Clone)]
pub struct NativeNode {
    func: fn(parent: &ContextRc, args: &Vec<VarType>) -> Result<VarType, CodeExecError>,
}

impl NativeNode {
    fn exec(&self, parent: &ContextRc, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        (self.func)(parent, args)
    }

    pub fn as_vartype(
        func: fn(parent: &ContextRc, args: &Vec<VarType>) -> Result<VarType, CodeExecError>,
    ) -> VarType {
        VarType::Node(Node::Native(NativeNode { func }))
    }
}
