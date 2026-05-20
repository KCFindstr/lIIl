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
    ss: VarType,
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

    fn split_string(ctx: &ContextRc, args: &Vec<VarType>) -> Result<VarType, CodeExecError> {
        if args.len() < 2 {
            return Err(CodeExecError::new_str("tp@ss requires 2 arguments: string and delimiters".to_string()));
        }
        let s = match &args[0] {
            VarType::String(s) => s,
            _ => return Err(CodeExecError::new_str("First argument to tp@ss must be a string".to_string())),
        };
        let delims = match &args[1] {
            VarType::String(s) => s,
            _ => return Err(CodeExecError::new_str("Second argument to tp@ss must be a string".to_string())),
        };

        let parts: Vec<&str> = s.split(|c| delims.contains(c)).collect();

        let mut obj = crate::data::data::Object::new();
        for (i, part) in parts.iter().enumerate() {
            obj.set(&i.to_string(), VarType::String(part.to_string()));
        }
        obj.set(crate::data::data::LEN_KEY, VarType::Int(parts.len() as i64));

        let rc = crate::data::data::MemData::new_rc(crate::data::data::MemData::Object(obj));
        ctx.borrow().get_global().borrow_mut().register_object(&rc);

        Ok(VarType::Ref(rc))
    }

    pub fn new(parent: &ContextRc) -> TpModule {
        TpModule {
            i: NativeNode::as_vartype(parent, Self::parse_int),
            f: NativeNode::as_vartype(parent, Self::parse_float),
            b: NativeNode::as_vartype(parent, Self::parse_bool),
            ss: NativeNode::as_vartype(parent, Self::split_string),
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
        mess.set("ss", self.ss.clone());
        Ok(mess)
    }
}
