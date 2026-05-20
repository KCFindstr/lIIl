use std::rc::Rc;

use crate::{
    data::{context::ContextRc, module::ModuleFactoryManager},
    parser::parse_str,
};

use super::Module;

pub struct LlModule;

impl LlModule {
    pub const NAME: &str = "ll";

    pub fn register(manager: &mut ModuleFactoryManager) {
        manager.add_factory(
            LlModule::NAME,
            Rc::new(|parent: &ContextRc| -> Module {
                const SOURCE: &str = include_str!("../builtin/ll.lIIl");
                let module = parse_str(LlModule::NAME, SOURCE, parent)
                    .expect("built-in ll.lIIl parse error");
                Module::Code(module)
            }),
        );
    }
}
