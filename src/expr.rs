use std::mem;

use crate::{
    data::context::Context,
    data::{context::ContextRc, data::Array, variable::VarType},
    statement::CodeExecError,
};

fn expr_type_error_2(ctx: &Context, lhs: VarType, rhs: VarType) -> CodeExecError {
    CodeExecError::new(ctx, format!("Type error: {:?}, {:?}", lhs, rhs))
}

fn expr_type_error_1(ctx: &Context, value: VarType) -> CodeExecError {
    CodeExecError::new(ctx, format!("Type error: {:?}", value))
}

fn promote_type(
    ctx: &Context,
    lhs: VarType,
    rhs: VarType,
) -> Result<(VarType, VarType), CodeExecError> {
    if mem::discriminant(&lhs) == mem::discriminant(&rhs) {
        return Ok((lhs, rhs));
    }
    match (&lhs, &rhs) {
        (VarType::Int(l), VarType::Float(_r)) => Ok((VarType::Float(*l as f64), rhs)),
        (VarType::Float(_l), VarType::Int(r)) => Ok((lhs, VarType::Float(*r as f64))),
        _ => Err(expr_type_error_2(ctx, lhs, rhs)),
    }
}

pub enum Expr {
    Int(IntLiteral),
    String(StringLiteral),
    Float(FloatLiteral),
    Add(AddExpr),
    Sub(SubExpr),
    Mul(MulExpr),
    Div(DivExpr),
    Mod(ModExpr),
    Neg(NegExpr),
    Tuple(TupleExpr),
    NodeCall(NodeCallExpr),
}

impl Expr {
    pub fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        match self {
            Expr::Int(expr) => expr.eval(ctx),
            Expr::String(expr) => expr.eval(ctx),
            Expr::Float(expr) => expr.eval(ctx),
            Expr::Add(expr) => expr.eval(ctx),
            Expr::Sub(expr) => expr.eval(ctx),
            Expr::Mul(expr) => expr.eval(ctx),
            Expr::Div(expr) => expr.eval(ctx),
            Expr::Mod(expr) => expr.eval(ctx),
            Expr::Neg(expr) => expr.eval(ctx),
            Expr::Tuple(expr) => expr.eval(ctx),
            Expr::NodeCall(expr) => expr.eval(ctx),
        }
    }
}

pub struct IntLiteral {
    pub value: i64,
}

impl IntLiteral {
    fn eval(&self, _: &ContextRc) -> Result<VarType, CodeExecError> {
        Ok(VarType::Int(self.value))
    }
}

pub struct StringLiteral {
    pub value: String,
}

impl StringLiteral {
    fn eval(&self, _: &ContextRc) -> Result<VarType, CodeExecError> {
        Ok(VarType::String(self.value.clone()))
    }
}

pub struct FloatLiteral {
    pub value: f64,
}

impl FloatLiteral {
    fn eval(&self, _: &ContextRc) -> Result<VarType, CodeExecError> {
        Ok(VarType::Float(self.value))
    }
}

pub struct AddExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl AddExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let vl = self.lhs.eval(ctx)?;
        let vr = self.rhs.eval(ctx)?;
        let (lhs, rhs) = promote_type(&ctx.borrow(), vl, vr)?;
        match (lhs, rhs) {
            (VarType::Int(l), VarType::Int(r)) => Ok(VarType::Int(l + r)),
            (VarType::Float(l), VarType::Float(r)) => Ok(VarType::Float(l + r)),
            (VarType::String(l), VarType::String(r)) => Ok(VarType::String(l + &r)),
            (l, r) => Err(expr_type_error_2(&ctx.borrow(), l, r)),
        }
    }
}

pub struct SubExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl SubExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let vl = self.lhs.eval(ctx)?;
        let vr = self.rhs.eval(ctx)?;
        let (lhs, rhs) = promote_type(&ctx.borrow(), vl, vr)?;
        match (lhs, rhs) {
            (VarType::Int(l), VarType::Int(r)) => Ok(VarType::Int(l - r)),
            (VarType::Float(l), VarType::Float(r)) => Ok(VarType::Float(l - r)),
            (l, r) => Err(expr_type_error_2(&ctx.borrow(), l, r)),
        }
    }
}

pub struct MulExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl MulExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let vl = self.lhs.eval(ctx)?;
        let vr = self.rhs.eval(ctx)?;
        let (lhs, rhs) = promote_type(&ctx.borrow(), vl, vr)?;
        match (lhs, rhs) {
            (VarType::Int(l), VarType::Int(r)) => Ok(VarType::Int(l * r)),
            (VarType::Float(l), VarType::Float(r)) => Ok(VarType::Float(l * r)),
            (l, r) => Err(expr_type_error_2(&ctx.borrow(), l, r)),
        }
    }
}

pub struct DivExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl DivExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let vl = self.lhs.eval(ctx)?;
        let vr = self.rhs.eval(ctx)?;
        let (lhs, rhs) = promote_type(&ctx.borrow(), vl, vr)?;
        match (lhs, rhs) {
            (VarType::Int(l), VarType::Int(r)) => Ok(VarType::Int(l / r)),
            (VarType::Float(l), VarType::Float(r)) => Ok(VarType::Float(l / r)),
            (l, r) => Err(expr_type_error_2(&ctx.borrow(), l, r)),
        }
    }
}

pub struct ModExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl ModExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let vl = self.lhs.eval(ctx)?;
        let vr = self.rhs.eval(ctx)?;
        let (lhs, rhs) = promote_type(&ctx.borrow(), vl, vr)?;
        match (lhs, rhs) {
            (VarType::Int(l), VarType::Int(r)) => Ok(VarType::Int(l % r)),
            (VarType::Float(l), VarType::Float(r)) => Ok(VarType::Float(l % r)),
            (l, r) => Err(expr_type_error_2(&ctx.borrow(), l, r)),
        }
    }
}

pub struct NegExpr {
    pub value: Box<Expr>,
}

impl NegExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let value = self.value.eval(ctx)?;
        match value {
            VarType::Int(value) => Ok(VarType::Int(-value)),
            VarType::Float(value) => Ok(VarType::Float(-value)),
            _ => Err(expr_type_error_1(&ctx.borrow(), value)),
        }
    }
}

pub struct TupleExpr {
    pub values: Vec<Box<Expr>>,
}

impl TupleExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let mut items = Vec::new();
        for value in &self.values {
            items.push(value.eval(ctx)?);
        }
        Ok(VarType::Tuple(Array { items }))
    }
}

pub struct NodeCallExpr {
    pub node_name: Box<Expr>,
    pub args: VarType,
}

impl NodeCallExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let borrowed_ctx = &*ctx.borrow();
        match &self.args {
            VarType::Tuple(args) => {
                let node_name = self.node_name.eval(ctx)?;
                match node_name {
                    VarType::String(node_name) => {
                        let var = (*borrowed_ctx
                            .get_symbol_or_err(borrowed_ctx, &node_name)?
                            .borrow())
                        .clone();
                        if let VarType::Node(node) = var {
                            let result = node.exec(&args.items)?;
                            Ok(result)
                        } else {
                            return Err(CodeExecError::new(
                                borrowed_ctx,
                                format!("Expected node, got {:?}", var),
                            ));
                        }
                    }
                    _ => Err(CodeExecError::new(
                        borrowed_ctx,
                        format!("Expected string, got {:?}", node_name),
                    )),
                }
            }
            _ => Err(CodeExecError::new(
                borrowed_ctx,
                format!("Expected array, got {:?}", self.args),
            )),
        }
    }
}
