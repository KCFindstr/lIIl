use pest::iterators::Pairs;

use crate::{
    module::CodeModule,
    statement::{rm::RmStatement, CodeExecError, Statement, Statements},
};

use super::Rule;

fn parse_stmt_block(
    module: &mut CodeModule,
    pairs: Pairs<Rule>,
) -> Result<Statements, CodeExecError> {
    let mut stmts = Statements::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::left_brace => continue,
            Rule::stmt => stmts.push(parse_stmt(module, pair.into_inner())?),
            Rule::right_brace => continue,
            _ => unreachable!(),
        }
    }
    Ok(stmts)
}

fn parse_rm(module: &mut CodeModule, pairs: Pairs<Rule>) -> Result<RmStatement, CodeExecError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::rm_stmt_prefix => continue,
            Rule::package_name => return Ok(RmStatement::new(module, pair.as_str())),
            _ => unreachable!(),
        }
    }
    unreachable!()
}

pub fn parse_stmt(module: &mut CodeModule, pairs: Pairs<Rule>) -> Result<Statement, CodeExecError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::stmt_end => continue,
            Rule::rm_stmt => return Ok(Statement::Rm(parse_rm(module, pair.into_inner())?)),
            Rule::stmt_block => {
                return Ok(Statement::Stmts(parse_stmt_block(
                    module,
                    pair.into_inner(),
                )?))
            }
            _ => unreachable!(),
        }
    }
    Ok(Statement::Stmts(Statements::new()))
}
