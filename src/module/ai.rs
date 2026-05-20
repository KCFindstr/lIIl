use std::{cell::RefCell, collections::VecDeque, io::BufRead, rc::Rc};

use crate::{
    data::node::NativeNode,
    data::{context::ContextRc, data::Mess, module::ModuleFactoryManager, variable::VarType},
    statement::CodeExecError,
};

use super::{IModule, Module, NativeModule};

thread_local! {
    static MOCK_STDIN: RefCell<VecDeque<String>> = RefCell::new(VecDeque::new());
}

/// Queue lines to be returned by `tpu` in place of real stdin.
/// Each call to `tpu` pops one line from the front of the queue.
/// When the queue is empty, `tpu` falls back to the real stdin.
#[allow(dead_code)]
pub fn mock_input(lines: impl IntoIterator<Item = impl Into<String>>) {
    MOCK_STDIN.with(|mock| {
        *mock.borrow_mut() = lines.into_iter().map(|s| s.into()).collect();
    });
}

pub struct AiModule {
    tpu: VarType,
}

impl AiModule {
    pub const NAME: &str = "ai";

    fn read_line(_ctx: &ContextRc, _args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        if let Some(line) = MOCK_STDIN.with(|mock| mock.borrow_mut().pop_front()) {
            return Ok(VarType::String(line));
        }
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
