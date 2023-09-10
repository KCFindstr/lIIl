use std::fmt;

use crate::statement::{CodeExecError, Statement};

use super::{
    context::{Context, ContextRc},
    data::MemData,
    variable::VarType,
};

#[derive(Clone)]
pub enum Node {
    Code(CodeNode),
    Native(NativeNode),
}

impl Node {
    pub fn exec(&mut self, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        match self {
            Node::Code(node) => node.exec(args),
            Node::Native(node) => node.exec(args),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node").finish()
    }
}

pub struct CodeNode {
    parent: ContextRc,
    pub args: Vec<String>,
    pub body: Box<Statement>,
}

impl fmt::Debug for CodeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CodeNode")
            .field("args", &self.args)
            .field("body", &self.body)
            .finish()
    }
}

impl Clone for CodeNode {
    fn clone(&self) -> Self {
        CodeNode {
            parent: self.parent.clone(),
            args: self.args.clone(),
            body: self.body.clone(),
        }
    }
}

impl CodeNode {
    pub fn new(parent: &ContextRc, body: Box<Statement>) -> Self {
        CodeNode {
            parent: parent.clone(),
            args: Vec::new(),
            body: body,
        }
    }

    pub fn exec(&mut self, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        let ctx = Context::new_rc(&self.parent);
        if args.len() > self.args.len() {
            return Err(CodeExecError::new(
                &ctx.borrow(),
                format!(
                    "Too many arguments for function: expected {}, got {}",
                    self.args.len(),
                    args.len()
                ),
            ));
        }
        for (value, name) in args.iter().zip(&self.args) {
            ctx.borrow_mut().set_symbol(&name, value.clone());
        }
        Context::with(&ctx, || {
            self.body.exec(&ctx).map(|v| v.unwrap_or(VarType::Nzero))
        })
    }
}

pub type NativeFunc = fn(parent: &ContextRc, args: &Vec<VarType>) -> Result<VarType, CodeExecError>;

#[derive(Clone)]
pub struct NativeNode {
    parent: ContextRc,
    func: NativeFunc,
}

impl NativeNode {
    pub fn new(parent: &ContextRc, func: NativeFunc) -> Self {
        NativeNode {
            parent: parent.clone(),
            func,
        }
    }

    fn exec(&self, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        (self.func)(&self.parent, args)
    }

    pub fn as_vartype(parent: &ContextRc, func: NativeFunc) -> VarType {
        let node = MemData::Node(Node::Native(NativeNode::new(parent, func)));
        VarType::Ref(MemData::new_rc(node))
    }
}
