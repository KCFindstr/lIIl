use once_cell::sync::Lazy;
use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
};

use crate::{
    data::lvalue::LValue,
    expr::{
        AddExpr, DivExpr, Expr, IntLiteral, MemberExpr, ModExpr, MulExpr, NegExpr, NodeCallExpr,
        SubExpr, TupleExpr,
    },
    statement::CodeExecError,
};

use super::Rule;

static PRATT_PARSER: Lazy<PrattParser<Rule>> = Lazy::new(|| {
    PrattParser::new()
        .op(Op::infix(Rule::tuple_op, Assoc::Left))
        .op(Op::infix(Rule::add_op, Assoc::Left) | Op::infix(Rule::sub_op, Assoc::Left))
        .op(Op::infix(Rule::mul_op, Assoc::Left)
            | Op::infix(Rule::div_op, Assoc::Left)
            | Op::infix(Rule::mod_op, Assoc::Left))
        .op(Op::prefix(Rule::pos_neg_op))
        .op(Op::infix(Rule::member_op, Assoc::Left))
        .op(Op::infix(Rule::node_call_op, Assoc::Left))
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
        _ => unreachable!(),
    }
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::int_literal => Expr::Int(IntLiteral {
                value: primary.as_str().parse::<i64>().unwrap(),
            }),
            Rule::expr => parse_expr(primary.into_inner()), // from "(" ~ expr ~ ")"
            _ => unreachable!(),
        })
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
            _ => unreachable!(),
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
            Rule::member_op => Expr::Member(MemberExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
            Rule::node_call_op => Expr::NodeCall(NodeCallExpr {
                node_name: Box::new(rhs),
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
            _ => unreachable!(),
        })
        .parse(pairs)
}
