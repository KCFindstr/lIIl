use crate::{
    data::context::Context,
    data::variable::{SymbolProvider, VarType},
    module::NativeNode,
    statement::CodeExecError,
};

pub struct CpuModule {
    op: VarType,
    pub ctx: Context,
}

impl CpuModule {
    pub fn new() -> CpuModule {
        CpuModule {
            op: NativeNode::as_vartype(
                |_: &Context, args: &Vec<VarType>| -> Result<VarType, CodeExecError> {
                    let joined = args
                        .iter()
                        .map(|item| format!("{}", item))
                        .collect::<Vec<String>>()
                        .join(" ");
                    println!("{}", joined);
                    Ok(VarType::Nzero)
                },
            ),
            ctx: Context::new(),
        }
    }

    pub fn exec(&self) -> Result<(), CodeExecError> {
        Ok(())
    }
}

impl SymbolProvider for CpuModule {
    fn add(&mut self, _name: &str, _value: VarType) -> &VarType {
        panic!("Cannot add symbol to CPU module.")
    }
    fn get(&self, name: &str) -> Option<&VarType> {
        match name {
            "op" => Some(&self.op),
            _ => None,
        }
    }
    fn set(&mut self, _name: &str, _value: VarType) -> Option<&VarType> {
        panic!("Cannot set symbol in CPU module.")
    }
}
