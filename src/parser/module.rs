use pest::iterators::Pairs;

use crate::{module::CodeModule, statement::CodeExecError};

use super::{stmt::parse_stmt, Rule};

pub fn parse_module(module: &mut CodeModule, pairs: Pairs<Rule>) -> Result<(), CodeExecError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::stmt => module.stmts.push(parse_stmt(pair.into_inner())?),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    Ok(())
}
