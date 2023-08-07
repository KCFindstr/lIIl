use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};
use pest_derive::Parser;

use crate::{
    expr::{AddExpr, DivExpr, Expr, IntLiteral, ModExpr, MulExpr, NegExpr, SubExpr, TupleExpr},
    module::CodeModule,
};

#[derive(Parser)]
#[grammar = "lIIl.pest"]
#[allow(non_camel_case_types)]
struct lIIlParser;

fn parse_expr(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> Box<Expr> {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::int_literal => Box::new(Expr::Int(IntLiteral {
                value: primary.as_str().parse::<i64>().unwrap(),
            })),
            Rule::expr => parse_expr(primary.into_inner(), pratt), // from "(" ~ expr ~ ")"
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
            Rule::add_op => Box::new(Expr::Add(AddExpr { lhs, rhs })),
            Rule::sub_op => Box::new(Expr::Sub(SubExpr { lhs, rhs })),
            Rule::mul_op => Box::new(Expr::Mul(MulExpr { lhs, rhs })),
            Rule::div_op => Box::new(Expr::Div(DivExpr { lhs, rhs })),
            Rule::mod_op => Box::new(Expr::Mod(ModExpr { lhs, rhs })),
            _ => unreachable!(),
        })
        .parse(pairs)
}

fn parse_module(module: &mut CodeModule, pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) {
    // pass
}

pub fn parse(module: &mut CodeModule, input: &str) -> Result<(), pest::error::Error<Rule>> {
    let pairs = lIIlParser::parse(Rule::program, input)?;
    println!("{:?}", pairs);
    let pratt = PrattParser::new()
        .op(Op::infix(Rule::tuple_op, Assoc::Left))
        .op(Op::infix(Rule::add_op, Assoc::Left) | Op::infix(Rule::sub_op, Assoc::Left))
        .op(Op::infix(Rule::mul_op, Assoc::Left)
            | Op::infix(Rule::div_op, Assoc::Left)
            | Op::infix(Rule::mod_op, Assoc::Left))
        .op(Op::prefix(Rule::pos_neg_op))
        .op(Op::infix(Rule::node_call_op, Assoc::Left));
    Ok(parse_module(module, pairs, &pratt))
}
