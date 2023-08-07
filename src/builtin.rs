use crate::{data::context::Context, module::CodeModule};

pub mod cpu;

pub fn all_builtin_modules() -> Vec<Module> {
    vec![Module::Cpu(cpu::CpuModule::new())]
}

pub enum Module {
    Code(CodeModule),
    Cpu(cpu::CpuModule),
}

impl Module {
    pub fn exec(&self) -> Result<(), crate::statement::CodeExecError> {
        match self {
            Module::Code(module) => module.exec(),
            Module::Cpu(module) => module.exec(),
        }
    }
    pub fn get_ctx_mut(&mut self) -> &mut Context {
        match self {
            Module::Code(module) => &mut module.ctx,
            Module::Cpu(module) => &mut module.ctx,
        }
    }
}
