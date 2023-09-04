use rand::Rng;

use crate::data::{context::ContextRc, variable::VarType};

use super::{CodeExecError, Statement};

#[derive(Debug, Clone)]
pub struct MaybeStatement {
    pub body: Box<Statement>,
}

impl MaybeStatement {
    const PROBABILITY: f64 = 0.5;

    #[inline]
    pub fn exec(&self, ctx: &ContextRc) -> Result<Option<VarType>, CodeExecError> {
        if rand::thread_rng().gen_bool(Self::PROBABILITY) {
            self.body.exec(ctx)
        } else {
            Ok(None)
        }
    }
}
