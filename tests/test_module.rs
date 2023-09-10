use std::rc::Rc;

use lIIl::{
    data::node::NativeNode,
    data::{
        context::ContextRc, data::Mess, module_manager::ModuleFactoryManager, variable::VarType,
    },
    module::{IModule, Module, NativeModule},
    statement::CodeExecError,
};

pub struct TestModule {
    assert: VarType,
}

impl TestModule {
    pub const NAME: &str = "ut";

    fn assert(args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        let is_true: bool = (&args[0]).into();
        assert!(
            is_true,
            "{}",
            args.iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        Ok(VarType::Nzero)
    }

    pub fn new(parent: &ContextRc) -> TestModule {
        TestModule {
            assert: NativeNode::as_vartype(parent, |_: &ContextRc, args: &Vec<VarType>| {
                Self::assert(args)
            }),
        }
    }

    pub fn register(manager: &mut ModuleFactoryManager) {
        manager.add_factory(
            Self::NAME,
            Rc::new(move |parent: &ContextRc| -> Module {
                Module::Native(NativeModule::new(
                    Self::NAME,
                    Self::NAME,
                    parent,
                    Box::new(Self::new(parent)),
                ))
            }),
        );
    }
}

impl IModule for TestModule {
    fn exec(&self, _ctx: &ContextRc) -> Result<Mess, CodeExecError> {
        let mut mess = Mess::new();
        mess.set("assert", self.assert.clone());
        Ok(mess)
    }
}
