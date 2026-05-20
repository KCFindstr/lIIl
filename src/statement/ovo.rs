use crate::{
    data::{
        context::ContextRc,
        data::MemData,
        variable::VarType,
    },
    expr::Expr,
};

use super::{CodeExecError, Statement};

#[derive(Debug, Clone)]
pub struct OvoStatement {
    pub var: String,
    pub obj: Expr,
    pub body: Box<Statement>,
}

impl OvoStatement {
    #[inline]
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        let val = self.obj.eval(ctx)?;
        let keys = match val {
            VarType::Ref(data) => match &*data.borrow() {
                MemData::Object(obj) => obj.keys(),
                _ => {
                    return Err(CodeExecError::new(
                        &ctx.borrow(),
                        "ovo: expected an object (lol) to iterate over".to_string(),
                    ))
                }
            },
            _ => {
                return Err(CodeExecError::new(
                    &ctx.borrow(),
                    "ovo: expected a reference to an object (lol)".to_string(),
                ))
            }
        };
        for key in keys {
            ctx.borrow().set_symbol(&self.var, VarType::String(key));
            if let Some(ret) = self.body.exec(ctx)? {
                return Ok(Some(ret));
            }
        }
        Ok(None)
    }
}
