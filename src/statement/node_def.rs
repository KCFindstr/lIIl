use crate::data::{
    context::ContextRc,
    data::MemData,
    node::{CodeNode, Node},
    variable::VarType,
};

use super::{CodeExecError, Statement};

#[derive(Debug, Clone)]
pub struct NodeDefStatement {
    pub name: String,
    pub args: Vec<String>,
    pub body: Box<Statement>,
}

impl NodeDefStatement {
    pub fn exec(&mut self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        let mut code_node = CodeNode::new(ctx);
        code_node.args.append(&mut self.args);
        code_node.body.stmts.push(*self.body.clone());
        let node = Node::Code(code_node);
        let var = ctx.borrow().add_mem(MemData::Node(node));
        ctx.borrow().set_symbol(&self.name, var);
        Ok(None)
    }
}
