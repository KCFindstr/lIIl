use crate::{
    data::node::NativeNode,
    data::{
        context::ContextRc,
        data::Mess,
        module_manager::{ModuleFactory, ModuleFactoryManager},
        variable::VarType,
    },
    statement::CodeExecError,
};

use super::{IModule, Module, NativeModule};

pub struct CpuModule {
    op: VarType,
}

impl CpuModule {
    pub const NAME: &str = "cpu";

    pub fn new(parent: &ContextRc) -> CpuModule {
        CpuModule {
            op: NativeNode::as_vartype(
                parent,
                |_: &ContextRc, args: &Vec<VarType>| -> Result<VarType, CodeExecError> {
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

    pub fn register(manager: &mut ModuleFactoryManager) {
        manager.add_factory(ModuleFactory {
            name: CpuModule::NAME.to_string(),
            factory: Box::new(move |parent: &ContextRc| -> Module {
                Module::Native(NativeModule::new(
                    CpuModule::NAME,
                    CpuModule::NAME,
                    parent,
                    Box::new(CpuModule::new(parent)),
                ))
            }),
        });
    }
}

impl IModule for CpuModule {
    fn exec(&self, _ctx: &ContextRc) -> Result<Mess, CodeExecError> {
        let mut mess = Mess::new();
        mess.set("op", self.op.clone());
        Ok(mess)
    }
}
