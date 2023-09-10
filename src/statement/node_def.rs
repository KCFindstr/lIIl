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
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        let mut code_node = CodeNode::new(ctx, self.body.clone());
        code_node.args = self.args.clone();
        let node = Node::Code(code_node);
        let var = MemData::new_rc(MemData::Node(node));
        ctx.borrow().set_symbol(&self.name, VarType::Ref(var));
        Ok(None)
    }
}
