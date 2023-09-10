use std::{fmt::Debug, mem};

use rand::Rng;

use crate::{
    data::context::Context,
    data::{
        context::ContextRc,
        data::{Array, MemData, MemDataRc},
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

fn promote_add(
    ctx: &Context,
    lhs: VarType,
    rhs: VarType,
) -> Result<(VarType, VarType), CodeExecError> {
    if let VarType::String(_) = &lhs {
        return Ok((lhs, VarType::String(rhs.to_string())));
    }
    if let VarType::String(_) = &rhs {
        return Ok((VarType::String(lhs.to_string()), rhs));
    }
    promote_type(ctx, lhs, rhs)
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

#[derive(Clone)]
pub enum Expr {
    Literal(LiteralExpr),
    Identifier(IdentifierExpr),
    Add(AddExpr),
    Sub(SubExpr),
    Mul(MulExpr),
    Div(DivExpr),
    Mod(ModExpr),
    Cmp(CompareExpr),
    Not(NotExpr),
    Neg(NegExpr),
    Tuple(TupleExpr),
    Array(TupleExpr),
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
            Expr::Cmp(_expr) => write!(f, "CmpExpr"),
            Expr::Not(_expr) => write!(f, "NotExpr"),
            Expr::Neg(_expr) => write!(f, "NegExpr"),
            Expr::Tuple(_expr) => write!(f, "TupleExpr"),
            Expr::Array(_expr) => write!(f, "ArrayExpr"),
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
            Expr::Cmp(expr) => expr.eval(ctx),
            Expr::Not(expr) => expr.eval(ctx),
            Expr::Neg(expr) => expr.eval(ctx),
            Expr::Tuple(expr) => expr.eval(ctx),
            Expr::Array(expr) => expr.eval(ctx),
            Expr::Member(expr) => expr.eval(ctx),
            Expr::NodeCall(expr) => expr.eval(ctx),
        }
    }

    pub fn literal(value: VarType) -> Expr {
        Expr::Literal(LiteralExpr { value })
    }
}

#[derive(Clone)]
pub struct LiteralExpr {
    pub value: VarType,
}

impl LiteralExpr {
    fn eval(&self, _: &ContextRc) -> Result<VarType, CodeExecError> {
        Ok(self.value.clone())
    }
}

#[derive(Clone)]
pub struct IdentifierExpr {
    pub name: String,
}

impl IdentifierExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        ctx.borrow().get_symbol_or_err(&self.name)
    }
}

#[derive(Clone)]
pub struct AddExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl AddExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let vl = self.lhs.eval(ctx)?;
        let vr = self.rhs.eval(ctx)?;
        let (lhs, rhs) = promote_add(&ctx.borrow(), vl, vr)?;
        match (lhs, rhs) {
            (VarType::Int(l), VarType::Int(r)) => Ok(VarType::Int(l + r)),
            (VarType::Float(l), VarType::Float(r)) => Ok(VarType::Float(l + r)),
            (VarType::String(l), VarType::String(r)) => Ok(VarType::String(l + &r)),
            (l, r) => Err(expr_type_error_2(&ctx.borrow(), l, r)),
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone, Debug)]
pub enum CompareOp {
    Less,
    Greater,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
}

#[derive(Clone)]
pub struct CompareExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub op: CompareOp,
}

impl CompareExpr {
    fn compare<T>(&self, lhs: T, rhs: T) -> Result<VarType, CodeExecError>
    where
        T: PartialOrd + Debug,
    {
        match self.op {
            CompareOp::Less => Ok(VarType::Bool(lhs < rhs)),
            CompareOp::Greater => Ok(VarType::Bool(lhs > rhs)),
            CompareOp::Equal => Ok(VarType::Bool(lhs == rhs)),
            CompareOp::NotEqual => Ok(VarType::Bool(lhs != rhs)),
            CompareOp::LessEqual => Ok(VarType::Bool(lhs <= rhs)),
            CompareOp::GreaterEqual => Ok(VarType::Bool(lhs >= rhs)),
        }
    }

    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let vl = self.lhs.eval(ctx)?;
        let vr = self.rhs.eval(ctx)?;
        let (lhs, rhs) = promote_type(&ctx.borrow(), vl, vr)?;
        match (lhs, rhs) {
            (VarType::Bool(l), VarType::Bool(r)) => self.compare(l, r),
            (VarType::Int(l), VarType::Int(r)) => self.compare(l, r),
            (VarType::Float(l), VarType::Float(r)) => self.compare(l, r),
            (VarType::String(l), VarType::String(r)) => self.compare(l, r),
            (l, r) => Err(expr_type_error_2(&ctx.borrow(), l, r)),
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct NotExpr {
    pub value: Box<Expr>,
}

impl NotExpr {
    fn sample_int(value: i64) -> i64 {
        let mut rng = rand::thread_rng();
        loop {
            let sample = rng.gen_range(i64::MIN..=i64::MAX);
            if sample != value {
                return sample;
            }
        }
    }
    fn sample_float(value: f64) -> f64 {
        let mut rng = rand::thread_rng();
        loop {
            let sample = rng.gen_range(0.0..1.0);
            if sample != value {
                return sample;
            }
        }
    }
    fn sample_ref() -> i64 {
        Self::sample_int(0)
    }
    pub fn not(ctx: &ContextRc, value: VarType) -> Result<VarType, CodeExecError> {
        match value {
            VarType::Int(value) => Ok(VarType::Int(Self::sample_int(value))),
            VarType::Float(value) => Ok(VarType::Float(Self::sample_float(value))),
            VarType::String(value) => Ok(VarType::String("!".to_owned() + &value)),
            VarType::Bool(value) => Ok(VarType::Bool(!value)),
            VarType::Ref(_) => Ok(VarType::Nzero),
            VarType::Nzero => Ok(VarType::Int(Self::sample_ref())),
            _ => Err(expr_type_error_1(&ctx.borrow(), value)),
        }
    }
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        Self::not(ctx, self.value.eval(ctx)?)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct MemberExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl MemberExpr {
    fn get_data(&self, ctx: &ContextRc) -> Result<MemDataRc, CodeExecError> {
        let parent = self.rhs.eval(ctx)?;
        if let VarType::Ref(data) = parent {
            Ok(data)
        } else {
            Err(CodeExecError::new(
                &ctx.borrow(),
                format!("Expected ref, got {:?}", parent),
            ))
        }
    }
    fn get_key(&self, ctx: &ContextRc) -> Result<String, CodeExecError> {
        if let Expr::Identifier(id) = &*self.lhs {
            Ok(id.name.clone())
        } else if let Expr::Array(array) = &*self.lhs {
            let arr = array.eval(ctx)?;
            if let VarType::Tuple(tuple) = arr {
                if tuple.len() == 1 {
                    Ok(tuple.items[0].to_string())
                } else {
                    Err(CodeExecError::new(
                        &ctx.borrow(),
                        format!("Cannot index with {:?}", tuple),
                    ))
                }
            } else {
                panic!("Expected tuple, got {:?}", arr)
            }
        } else {
            Ok(self.lhs.eval(ctx)?.to_string())
        }
    }
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let key = self.get_key(ctx)?;
        let data = self.get_data(ctx)?;
        let borrowed_data = data.borrow();
        Ok(borrowed_data.get(&key))
    }

    pub fn set(&self, ctx: &ContextRc, val: VarType) -> Result<(), CodeExecError> {
        let key = self.get_key(ctx)?;
        let data = self.get_data(ctx)?;
        let mut borrowed_data = data.borrow_mut();
        borrowed_data.set(&ctx.borrow(), &key, val)
    }
}

#[derive(Clone)]
pub struct NodeCallExpr {
    pub node: Box<Expr>,
    pub args: Box<Expr>,
}

impl NodeCallExpr {
    fn eval(&self, ctx: &ContextRc) -> Result<VarType, CodeExecError> {
        let node = self.node.eval(ctx)?;
        let args = self.args.eval(ctx)?;
        if let VarType::Ref(data) = node {
            let mut node_copy;
            if let MemData::Node(node) = &*data.borrow() {
                node_copy = node.clone();
            } else {
                return Err(CodeExecError::new(
                    &ctx.borrow(),
                    format!("Expected node, got {:?}", data),
                ));
            }
            if let VarType::Tuple(args_tuple) = args {
                node_copy.exec(&args_tuple.items)
            } else {
                node_copy.exec(&vec![args])
            }
        } else {
            Err(CodeExecError::new(
                &ctx.borrow(),
                format!("Cannot call non-node {:?}", node),
            ))
        }
    }
}
