use std::{fmt::Debug, mem};

use crate::{
    data::context::Context,
    data::{
        context::ContextRc,
        data::{Array, MemData},
        global::DataItemRc,
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
    Literal(LiteralExpr),
    Identifier(IdentifierExpr),
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
            Expr::Literal(_expr) => write!(f, "LiteralExpr"),
            Expr::Identifier(_expr) => write!(f, "IdentifierExpr"),
            Expr::Add(_expr) => write!(f, "AddExpr"),
            Expr::Sub(_expr) => write!(f, "SubExpr"),
            Expr::Mul(_expr) => write!(f, "MulExpr"),
            Expr::Div(_expr) => write!(f, "DivExpr"),
            Expr::Mod(_expr) => write!(f, "ModExpr"),
            Expr::Neg(_expr) => write!(f, "NegExpr"),
            Expr::Tuple(_expr) => write!(f, "TupleExpr"),
            Expr::Member(_expr) => write!(f, "MemberExpr"),
            Expr::NodeCall(_expr) => write!(f, "NodeCallExpr"),
        }
    }
}

impl Expr {
    pub fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        match self {
            Expr::Literal(expr) => expr.eval(ctx),
            Expr::Identifier(expr) => expr.eval(ctx),
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

pub struct LiteralExpr {
    pub value: VarType,
}

impl LiteralExpr {
    fn eval(&self, _: &ContextRc) -> Result<VarType, CodeExecError> {
        Ok(self.value.clone())
    }
}

pub struct IdentifierExpr {
    pub name: String,
}

impl IdentifierExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        ctx.borrow().get_symbol_or_err(&self.name)
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
    fn get_data(&self, ctx: &ContextRc) -> Result<DataItemRc, CodeExecError> {
        let vl = self.lhs.eval(ctx)?;
        if let VarType::Ref(id) = vl {
            if let Some(data) = ctx.borrow().get_mem_by_ref(id) {
                Ok(data)
            } else {
                Err(CodeExecError::new(
                    &ctx.borrow(),
                    format!("Ref {:?} not found", id),
                ))
            }
        } else {
            Err(CodeExecError::new(
                &ctx.borrow(),
                format!("Expected ref, got {:?}", vl),
            ))
        }
    }
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let key = self.rhs.eval(ctx)?.to_string();
        Ok(self.get_data(ctx)?.borrow().data.get(&key))
    }

    pub fn set(&self, ctx: &ContextRc, val: VarType) -> Result<(), CodeExecError> {
        let key = self.rhs.eval(ctx)?.to_string();
        self.get_data(ctx)?
            .borrow_mut()
            .data
            .set(&ctx.borrow(), &key, val)
    }
}

pub struct NodeCallExpr {
    pub node_name: Box<Expr>,
    pub args: Box<Expr>,
}

impl NodeCallExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let node_name = self.node_name.eval(ctx)?;
        let args = self.args.eval(ctx)?;
        if let VarType::Ref(id) = node_name {
            if let Some(mem) = ctx.borrow().get_mem_by_ref(id) {
                if let MemData::Node(node) = &mut mem.borrow_mut().data {
                    if let VarType::Tuple(args_tuple) = args {
                        node.exec(&args_tuple.items)
                    } else {
                        node.exec(&vec![args])
                    }
                } else {
                    Err(CodeExecError::new(
                        &ctx.borrow(),
                        format!("Expected node, got {:?}", mem),
                    ))
                }
            } else {
                Err(CodeExecError::new(
                    &ctx.borrow(),
                    format!("Cannot find node {:?}", node_name),
                ))
            }
        } else {
            Err(CodeExecError::new(
                &ctx.borrow(),
                format!("Cannot call non-node {:?}", node_name),
            ))
        }
    }
}
