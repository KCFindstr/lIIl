use super::context::ContextRc;

pub struct ProgramStack {
    pub stack: Vec<ContextRc>,
}

impl ProgramStack {
    pub fn new() -> Self {
        ProgramStack { stack: Vec::new() }
    }

    pub fn push(&mut self, ctx: ContextRc) {
        self.stack.push(ctx);
    }

    pub fn pop(&mut self) -> Option<ContextRc> {
        self.stack.pop()
    }
}
