use std::rc::Rc;

use crate::{
    data::node::NativeNode,
    data::{context::ContextRc, data::Mess, module::ModuleFactoryManager, variable::VarType},
    statement::CodeExecError,
};

use super::{IModule, Module, NativeModule};

pub struct TpModule {
    i: VarType,
    f: VarType,
    b: VarType,
}

impl TpModule {
    pub const NAME: &str = "tp";

    fn parse_int(_ctx: &ContextRc, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        if let Some(VarType::String(s)) = args.first() {
            if let Ok(n) = s.trim().parse::<i64>() {
                return Ok(VarType::Int(n));
            }
        }
        Ok(VarType::Nzero)
    }

    fn parse_float(_ctx: &ContextRc, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        if let Some(VarType::String(s)) = args.first() {
            if let Ok(n) = s.trim().parse::<f64>() {
                return Ok(VarType::Float(n));
            }
        }
        Ok(VarType::Nzero)
    }

    fn parse_bool(_ctx: &ContextRc, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        if let Some(VarType::String(s)) = args.first() {
            let t = s.trim();
            let result = matches!(t, "O" | "true" | "True" | "TRUE" | "1");
            return Ok(VarType::Bool(result));
        }
        Ok(VarType::Bool(false))
    }

    pub fn new(parent: &ContextRc) -> TpModule {
        TpModule {
            i: NativeNode::as_vartype(parent, Self::parse_int),
            f: NativeNode::as_vartype(parent, Self::parse_float),
            b: NativeNode::as_vartype(parent, Self::parse_bool),
        }
    }

    pub fn register(manager: &mut ModuleFactoryManager) {
        manager.add_factory(
            TpModule::NAME,
            Rc::new(move |parent: &ContextRc| -> Module {
                Module::Native(NativeModule::new(
                    TpModule::NAME,
                    TpModule::NAME,
                    parent,
                    Box::new(TpModule::new(parent)),
                ))
            }),
        );
    }
}

impl IModule for TpModule {
    fn exec(&self, _ctx: &ContextRc) -> Result<Mess, CodeExecError> {
        let mut mess = Mess::new();
        mess.set("i", self.i.clone());
        mess.set("f", self.f.clone());
        mess.set("b", self.b.clone());
        Ok(mess)
    }
}
