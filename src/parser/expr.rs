use once_cell::sync::Lazy;
use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
};

use crate::expr::{
    AddExpr, DivExpr, Expr, IntLiteral, MemberExpr, ModExpr, MulExpr, NegExpr, SubExpr, TupleExpr,
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

pub fn parse_expr(pairs: Pairs<Rule>) -> Box<Expr> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::int_literal => Box::new(Expr::Int(IntLiteral {
                value: primary.as_str().parse::<i64>().unwrap(),
            })),
            Rule::expr => parse_expr(primary.into_inner()), // from "(" ~ expr ~ ")"
            _ => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::pos_neg_op => {
                if op.as_str() == "-" {
                    Box::new(Expr::Neg(NegExpr { value: rhs }))
                } else {
                    rhs
                }
            }
            _ => unreachable!(),
        })
        .map_infix(|mut lhs, op, rhs| match op.as_rule() {
            Rule::tuple_op => {
                if let Expr::Tuple(tuple) = lhs.as_mut() {
                    tuple.values.push(rhs);
                    lhs
                } else {
                    Box::new(Expr::Tuple(TupleExpr {
                        values: vec![lhs, rhs],
                    }))
                }
            }
            Rule::member_op => Box::new(Expr::Member(MemberExpr { lhs, rhs })),
            Rule::add_op => Box::new(Expr::Add(AddExpr { lhs, rhs })),
            Rule::sub_op => Box::new(Expr::Sub(SubExpr { lhs, rhs })),
            Rule::mul_op => Box::new(Expr::Mul(MulExpr { lhs, rhs })),
            Rule::div_op => Box::new(Expr::Div(DivExpr { lhs, rhs })),
            Rule::mod_op => Box::new(Expr::Mod(ModExpr { lhs, rhs })),
            _ => unreachable!(),
        })
        .parse(pairs)
}
