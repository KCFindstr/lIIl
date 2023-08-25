use pest::iterators::Pairs;

use crate::{module::CodeModule, statement::CodeExecError};

use super::{stmt::parse_stmt, Rule};

pub fn parse_module(module: &mut CodeModule, pairs: Pairs<Rule>) -> Result<(), CodeExecError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::stmt => {
                let stmt = parse_stmt(module, pair.into_inner())?;
                module.stmts.push(stmt);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    Ok(())
}
