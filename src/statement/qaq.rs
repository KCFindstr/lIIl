use crate::{
    data::{context::ContextRc, data::MemData, variable::VarType},
    expr::Expr,
};

use super::{CodeExecError, Statement};

#[derive(Debug, Clone)]
pub struct QaqStatement {
    pub var: String,
    pub obj: Expr,
    pub body: Box<Statement>,
}

impl QaqStatement {
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        let val = self.obj.eval(ctx)?;
        let (len, data_clone) = match val {
            VarType::Ref(data) => {
                let borrowed = data.borrow();
                match &*borrowed {
                    MemData::Object(obj) => {
                        let len = obj.get(crate::data::data::LEN_KEY);
                        let len_int = match len {
                            Some(VarType::Int(n)) => n,
                            None | Some(VarType::Nzero) => 0,
                            _ => return Err(CodeExecError::new(
                                &ctx.borrow(),
                                "qaq: expected object to have an integer 'len' property".to_string(),
                            )),
                        };
                        (len_int, data.clone())
                    }
                    _ => return Err(CodeExecError::new(
                        &ctx.borrow(),
                        "qaq: expected an object (lol)".to_string(),
                    )),
                }
            }
            _ => return Err(CodeExecError::new(
                &ctx.borrow(),
                "qaq: expected a reference to an object (lol)".to_string(),
            )),
        };

        for i in 0..len {
            let item_val = {
                let borrowed = data_clone.borrow();
                if let MemData::Object(obj) = &*borrowed {
                    obj.get(&i.to_string())
                } else {
                    None
                }
            };
            ctx.borrow().set_symbol(&self.var, item_val.unwrap_or(VarType::Nzero));
            if let Some(ret) = self.body.exec(ctx)? {
                return Ok(Some(ret));
            }
        }
        Ok(None)
    }
}
