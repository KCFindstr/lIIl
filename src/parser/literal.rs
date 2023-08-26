use pest::iterators::Pairs;

use crate::{data::variable::VarType, expr::LiteralExpr, statement::CodeExecError};

use super::Rule;

pub fn parse_identifier_tuple(pairs: Pairs<Rule>) -> Result<Vec<String>, CodeExecError> {
    let mut ids = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::identifier => ids.push(pair.as_str().to_string()),
            Rule::tuple_op => continue,
            _ => unreachable!(),
        }
    }
    Ok(ids)
}

pub fn parse_string_literal(pairs: Pairs<Rule>) -> String {
    let mut s = "".to_owned();
    for pair in pairs {
        match pair.as_rule() {
            Rule::double_quote => continue,
            Rule::single_quote => continue,
            Rule::escape => s.push(match pair.as_str() {
                "\\r" => '\r',
                "\\n" => '\n',
                "\\t" => '\t',
                "\\0" => '\0',
                other => other.chars().nth(1).unwrap(),
            }),
            _ => unreachable!(),
        }
    }
    s
}

pub fn parse_literal(pairs: Pairs<Rule>) -> LiteralExpr {
    LiteralExpr {
        value: match pairs.peek().unwrap().as_rule() {
            Rule::string_literal => VarType::String(parse_string_literal(pairs)),
            Rule::int_literal => VarType::Int(pairs.as_str().parse::<i64>().unwrap()),
            Rule::float_literal => {
                VarType::Float(pairs.as_str().replace(",", ".").parse::<f64>().unwrap())
            }
            Rule::bool_literal => VarType::Bool(if pairs.as_str() == "O" { true } else { false }),
            _ => unreachable!(),
        },
    }
}
