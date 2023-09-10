use once_cell::sync::Lazy;
use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
};

use crate::{
    data::lvalue::LValue,
    expr::{
        AddExpr, CompareExpr, CompareOp, DivExpr, Expr, IdentifierExpr, MemberExpr, ModExpr,
        MulExpr, NegExpr, NodeCallExpr, NotExpr, SubExpr, TupleExpr,
    },
    statement::CodeExecError,
};

use super::{literal::parse_literal, Rule};

static PRATT_PARSER: Lazy<PrattParser<Rule>> = Lazy::new(|| {
    PrattParser::new()
        .op(Op::infix(Rule::tuple_op, Assoc::Left))
        .op(Op::infix(Rule::greater_op, Assoc::Left)
            | Op::infix(Rule::geq_op, Assoc::Left)
            | Op::infix(Rule::less_op, Assoc::Left)
            | Op::infix(Rule::leq_op, Assoc::Left)
            | Op::infix(Rule::equal_op, Assoc::Left)
            | Op::infix(Rule::neq_op, Assoc::Left))
        .op(Op::infix(Rule::add_op, Assoc::Left) | Op::infix(Rule::sub_op, Assoc::Left))
        .op(Op::infix(Rule::mul_op, Assoc::Left)
            | Op::infix(Rule::div_op, Assoc::Left)
            | Op::infix(Rule::mod_op, Assoc::Left))
        .op(Op::prefix(Rule::pos_neg_op) | Op::prefix(Rule::not_op))
        .op(Op::infix(Rule::node_call_op, Assoc::Left))
        .op(Op::infix(Rule::member_op, Assoc::Right))
});

pub fn parse_lvalue(pairs: Pairs<Rule>) -> Result<LValue, CodeExecError> {
    let mut pairs = pairs.into_iter();
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::identifier => Ok(LValue::Identifier(pair.as_str().to_string())),
        Rule::expr => {
            let expr = parse_expr(pairs);
            if let Expr::Member(member) = expr {
                Ok(LValue::MemberExpr(member))
            } else {
                Err(CodeExecError::new_str(format!(
                    "Expected member expression, found {:?}",
                    expr
                )))
            }
        }
        _ => panic!("parse_lvalue: {:?}", pair),
    }
}

fn parse_expr_primary(pairs: Pairs<Rule>) -> Expr {
    for pair in pairs {
        match pair.as_rule() {
            Rule::literal_expr => return parse_literal(pair.into_inner()),
            Rule::identifier => {
                return Expr::Identifier(IdentifierExpr {
                    name: pair.as_str().to_string(),
                })
            }
            Rule::expr => return parse_expr(pair.into_inner()), // from "(" ~ expr ~ ")"
            _ => panic!("parse_expr_primary: {:?}", pair),
        }
    }
    panic!("parse_expr_primary: Reached end of input")
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| parse_expr_primary(primary.into_inner()))
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::pos_neg_op => {
                if op.as_str() == "-" {
                    Expr::Neg(NegExpr {
                        value: Box::new(rhs),
                    })
                } else {
                    rhs
                }
            }
            Rule::not_op => Expr::Not(NotExpr {
                value: Box::new(rhs),
            }),
            _ => panic!("parse_expr (prefix): {:?}", op),
        })
        .map_infix(|mut lhs, op, rhs| match op.as_rule() {
            Rule::add_op => Expr::Add(AddExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
            Rule::sub_op => Expr::Sub(SubExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
            Rule::mul_op => Expr::Mul(MulExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
            Rule::div_op => Expr::Div(DivExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
            Rule::mod_op => Expr::Mod(ModExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
            Rule::less_op => Expr::Cmp(CompareExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: CompareOp::Less,
            }),
            Rule::leq_op => Expr::Cmp(CompareExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: CompareOp::LessEqual,
            }),
            Rule::greater_op => Expr::Cmp(CompareExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: CompareOp::Greater,
            }),
            Rule::geq_op => Expr::Cmp(CompareExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: CompareOp::GreaterEqual,
            }),
            Rule::equal_op => Expr::Cmp(CompareExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: CompareOp::Equal,
            }),
            Rule::neq_op => Expr::Cmp(CompareExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: CompareOp::NotEqual,
            }),
            Rule::member_op => Expr::Member(MemberExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
            Rule::node_call_op => Expr::NodeCall(NodeCallExpr {
                node: Box::new(rhs),
                args: Box::new(lhs),
            }),
            Rule::tuple_op => {
                if let Expr::Tuple(tuple) = &mut lhs {
                    tuple.values.push(rhs);
                    lhs
                } else {
                    Expr::Tuple(TupleExpr {
                        values: vec![lhs, rhs],
                    })
                }
            }
            _ => panic!("parse_expr (infix): {:?}", op),
        })
        .parse(pairs)
}
