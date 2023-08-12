use crate::{
    data::node::NativeNode,
    data::{
        context::{Context, ContextRef},
        data::Mess,
        variable::VarType,
    },
    statement::CodeExecError,
};

use super::IModule;

pub struct CpuModule {
    op: VarType,
}

impl CpuModule {
    pub fn new() -> CpuModule {
        CpuModule {
            op: NativeNode::as_vartype(
                |_: &ContextRef, args: &Vec<VarType>| -> Result<VarType, CodeExecError> {
                    let joined = args
                        .iter()
                        .map(|item| format!("{}", item))
                        .collect::<Vec<String>>()
                        .join(" ");
                    println!("{}", joined);
                    Ok(VarType::Nzero)
                },
            ),
        }
    }
}
impl IModule for CpuModule {
    fn exec(&self, _ctx: &ContextRef) -> Result<Mess, CodeExecError> {
        let mut mess = Mess::new();
        mess.set("op", self.op.clone());
        Ok(mess)
    }
}
