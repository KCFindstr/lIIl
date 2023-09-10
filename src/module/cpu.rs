use std::rc::Rc;

use crate::{
    data::node::NativeNode,
    data::{context::ContextRc, data::Mess, module::ModuleFactoryManager, variable::VarType},
    statement::CodeExecError,
};

use super::{IModule, Module, NativeModule};

pub struct CpuModule {
    op: VarType,
    wcop: VarType,
}

impl CpuModule {
    pub const NAME: &str = "cpu";

    fn print(args: &Vec<VarType>, newline: bool) -> Result<VarType, CodeExecError> {
        let joined = args
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        if newline {
            println!("{}", joined);
        } else {
            print!("{}", joined);
        }
        Ok(VarType::Nzero)
    }

    pub fn new(parent: &ContextRc) -> CpuModule {
        CpuModule {
            op: NativeNode::as_vartype(parent, |_: &ContextRc, args: &Vec<VarType>| {
                Self::print(args, false)
            }),
            wcop: NativeNode::as_vartype(parent, |_: &ContextRc, args: &Vec<VarType>| {
                Self::print(args, true)
            }),
        }
    }

    pub fn register(manager: &mut ModuleFactoryManager) {
        manager.add_factory(
            CpuModule::NAME,
            Rc::new(move |parent: &ContextRc| -> Module {
                Module::Native(NativeModule::new(
                    CpuModule::NAME,
                    CpuModule::NAME,
                    parent,
                    Box::new(CpuModule::new(parent)),
                ))
            }),
        );
    }
}

impl IModule for CpuModule {
    fn exec(&self, _ctx: &ContextRc) -> Result<Mess, CodeExecError> {
        let mut mess = Mess::new();
        mess.set("op", self.op.clone());
        mess.set("wcop", self.wcop.clone());
        Ok(mess)
    }
}
