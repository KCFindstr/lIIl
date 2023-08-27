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

// TODO: Change to this implementation after compound atomic is supported.
pub fn _parse_string_literal(pairs: Pairs<Rule>) -> String {
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

pub fn parse_string_literal(pairs: Pairs<Rule>) -> String {
    let mut s = "".to_owned();
    let mut escape_next = false;
    for c in pairs.as_str()[1..(pairs.as_str().len() - 1)].chars() {
        if escape_next {
            s.push(match c {
                'r' => '\r',
                'n' => '\n',
                't' => '\t',
                '0' => '\0',
                other => other,
            });
            escape_next = false;
        } else if c == '\\' {
            escape_next = true;
        } else {
            s.push(c);
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
            Expr::literal(VarType::String(_parse_string_literal(pair.into_inner())))
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
