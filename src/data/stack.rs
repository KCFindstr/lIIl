use super::context::ContextRc;

pub struct ProgramStack {
    pub stack: Vec<ContextRc>,
}

impl ProgramStack {
    pub fn new() -> Self {
        ProgramStack { stack: Vec::new() }
    }

    pub fn push(&mut self, ctx: &ContextRc) {
        self.stack.push(ctx.clone());
    }

    pub fn pop(&mut self) -> Option<ContextRc> {
        self.stack.pop()
    }

    pub fn top(&self) -> Option<&ContextRc> {
        self.stack.last()
    }
}
