use std::{fmt::Debug, mem};

use crate::{
    data::context::Context,
    data::{
        context::ContextRc,
        data::{Array, MemData},
        node,
        variable::VarType,
    },
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
    Member(MemberExpr),
    NodeCall(NodeCallExpr),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Int(expr) => write!(f, "IntExpr"),
            Expr::String(expr) => write!(f, "StringExpr"),
            Expr::Float(expr) => write!(f, "FloatExpr"),
            Expr::Add(expr) => write!(f, "AddExpr"),
            Expr::Sub(expr) => write!(f, "SubExpr"),
            Expr::Mul(expr) => write!(f, "MulExpr"),
            Expr::Div(expr) => write!(f, "DivExpr"),
            Expr::Mod(expr) => write!(f, "ModExpr"),
            Expr::Neg(expr) => write!(f, "NegExpr"),
            Expr::Tuple(expr) => write!(f, "TupleExpr"),
            Expr::Member(expr) => write!(f, "MemberExpr"),
            Expr::NodeCall(expr) => write!(f, "NodeCallExpr"),
        }
    }
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
            Expr::Member(expr) => expr.eval(ctx),
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
    pub values: Vec<Expr>,
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

pub struct MemberExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl MemberExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let vl = self.lhs.eval(ctx)?;
        let key = self.rhs.eval(ctx)?.to_string();
        if let VarType::Ref(id) = *&vl {
            let borrowed_ctx = &*ctx.borrow();
            let data = borrowed_ctx
                .get_global()
                .borrow()
                .data
                .get_or_err(borrowed_ctx, id)?;
            let data_item = &data.borrow().data;
            if let MemData::Mess(mess) = &data_item {
                if let Some(value) = mess.get(&key) {
                    return Ok(value.clone());
                }
                return Ok(VarType::Nzero);
            } else {
                return Err(CodeExecError::new(
                    borrowed_ctx,
                    format!("Expected node, got {:?}", data_item),
                ));
            }
        } else {
            return Err(CodeExecError::new(
                &ctx.borrow(),
                format!("Expected ref, got {:?}", vl),
            ));
        }
    }

    pub fn set(&self, ctx: &ContextRc, val: VarType) -> Result<(), CodeExecError> {
        let vl = self.lhs.eval(ctx)?;
        let key = self.rhs.eval(ctx)?.to_string();
        if let VarType::Ref(id) = *&vl {
            let borrowed_ctx = &*ctx.borrow();
            let data = borrowed_ctx
                .get_global()
                .borrow()
                .data
                .get_or_err(borrowed_ctx, id)?;
            let borrowed_data = &mut *data.borrow_mut();
            if let MemData::Mess(mess) = &mut borrowed_data.data {
                mess.set(&key, val);
                return Ok(());
            } else {
                return Err(CodeExecError::new(
                    borrowed_ctx,
                    format!("Expected node, got {:?}", data),
                ));
            }
        } else {
            return Err(CodeExecError::new(
                &ctx.borrow(),
                format!("Expected ref, got {:?}", vl),
            ));
        }
    }
}

pub struct NodeCallExpr {
    pub node_name: Box<Expr>,
    pub args: VarType,
}

impl NodeCallExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let node_name = self.node_name.eval(ctx)?;
        match &self.args {
            VarType::Tuple(args) => match node_name {
                VarType::String(node_name) => {
                    if let Some(mem) = ctx.borrow().get_mem(&node_name) {
                        if let MemData::Node(node) = &mem.borrow().data {
                            return node.exec(&args.items);
                        } else {
                            return Err(CodeExecError::new(
                                &ctx.borrow(),
                                format!("Expected node, got {:?}", mem),
                            ));
                        }
                    } else {
                        return Err(CodeExecError::new(
                            &ctx.borrow(),
                            format!("Cannot find node {:?}", node_name),
                        ));
                    }
                }
                _ => Err(CodeExecError::new(
                    &ctx.borrow(),
                    format!("Expected string, got {:?}", node_name),
                )),
            },
            _ => Err(CodeExecError::new(
                &ctx.borrow(),
                format!("Expected array, got {:?}", self.args),
            )),
        }
    }
}
