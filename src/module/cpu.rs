use crate::{
    data::node::NativeNode,
    data::{
        context::{Context, ContextRef},
        variable::VarType,
    },
    statement::CodeExecError,
};

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
