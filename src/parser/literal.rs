use pest::iterators::{Pair, Pairs};

use crate::{
    data::variable::VarType,
    expr::{Expr, TupleExpr},
    statement::CodeExecError,
};

use super::{expr::parse_expr, Rule};

pub fn parse_identifier_tuple(pairs: Pairs<Rule>) -> Result<Vec<String>, CodeExecError> {
    let mut ids = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::identifier => ids.push(pair.as_str().to_string()),
            Rule::tuple_op => continue,
            _ => panic!("parse_identifier_tuple: {:?}", pair),
        }
    }
    Ok(ids)
}

pub fn parse_string_literal(pairs: Pairs<Rule>) -> String {
    let mut s = "".to_owned();
    for pair in pairs {
        match pair.as_rule() {
            Rule::char_in_double | Rule::char_in_single => {
                s.push(match pair.as_str() {
                    "\\\"" => '"',
                    "\\\\" => '\\',
                    "\\r" => '\r',
                    "\\n" => '\n',
                    "\\t" => '\t',
                    "\\0" => '\0',
                    "\\'" => '\'',
                    "\\\n" => '\n',
                    other => other.chars().nth(0).unwrap(),
                });
            }
            _ => panic!("parse_string_literal: {:?}", pair),
        }
    }
    s
}

pub fn parse_array_literal(pairs: Pair<Rule>) -> TupleExpr {
    let mut items = Vec::new();
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::expr => items.push(parse_expr(pair.into_inner())),
            _ => panic!("parse_array_literal: {:?}", pair),
        }
    }
    TupleExpr { values: items }
}

pub fn parse_literal(pairs: Pairs<Rule>) -> Expr {
    let pair = pairs.peek().unwrap();
    match pair.as_rule() {
        Rule::string_literal => {
            Expr::literal(VarType::String(parse_string_literal(pair.into_inner())))
        }
        Rule::int_literal => Expr::literal(VarType::Int(pair.as_str().parse::<i64>().unwrap())),
        Rule::float_literal => Expr::literal(VarType::Float(
            pair.as_str().replace(",", ".").parse::<f64>().unwrap(),
        )),
        Rule::bool_literal => Expr::literal(VarType::Bool(if pair.as_str() == "O" {
            true
        } else {
            false
        })),
        Rule::nzero_literal => Expr::literal(VarType::Nzero),
        Rule::array_literal => Expr::Array(parse_array_literal(pair)),
        _ => panic!("parse_literal: {:?}", pair),
    }
}
