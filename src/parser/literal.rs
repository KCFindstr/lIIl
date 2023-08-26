use pest::iterators::Pairs;

use crate::statement::CodeExecError;

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
