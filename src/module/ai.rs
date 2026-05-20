use std::{io::BufRead, rc::Rc};

use crate::{
    data::node::NativeNode,
    data::{context::ContextRc, data::Mess, module::ModuleFactoryManager, variable::VarType},
    statement::CodeExecError,
};

use super::{IModule, Module, NativeModule};

pub struct AiModule {
    tpu: VarType,
}

impl AiModule {
    pub const NAME: &str = "ai";

    fn read_line(_ctx: &ContextRc, _args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        let stdin = std::io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).ok();
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }
        Ok(VarType::String(line))
    }

    pub fn new(parent: &ContextRc) -> AiModule {
        AiModule {
            tpu: NativeNode::as_vartype(parent, Self::read_line),
        }
    }

    pub fn register(manager: &mut ModuleFactoryManager) {
        manager.add_factory(
            AiModule::NAME,
            Rc::new(move |parent: &ContextRc| -> Module {
                Module::Native(NativeModule::new(
                    AiModule::NAME,
                    AiModule::NAME,
                    parent,
                    Box::new(AiModule::new(parent)),
                ))
            }),
        );
    }
}

impl IModule for AiModule {
    fn exec(&self, _ctx: &ContextRc) -> Result<Mess, CodeExecError> {
        let mut mess = Mess::new();
        mess.set("tpu", self.tpu.clone());
        Ok(mess)
    }
}
