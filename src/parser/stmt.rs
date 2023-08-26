use pest::iterators::Pairs;

use crate::{
    data::lvalue,
    module::CodeModule,
    parser::{expr::parse_lvalue, literal::parse_identifier_tuple},
    statement::{
        ass::AssStatement, node_def::NodeDefStatement, rm::RmStatement, CodeExecError,
        ExprStatement, Statement, Statements,
    },
};

use super::{expr::parse_expr, Rule};

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

fn parse_ass(pairs: Pairs<Rule>) -> Result<AssStatement, CodeExecError> {
    let mut lhs = None;
    let mut rhs = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::ass_stmt_prefix => continue,
            Rule::lvalue => lhs = Some(parse_lvalue(pair.into_inner())?),
            Rule::expr => rhs = Some(parse_expr(pair.into_inner())),
            _ => unreachable!(),
        }
    }
    Ok(AssStatement {
        lhs: lhs.unwrap(),
        rhs: rhs.unwrap(),
    })
}

fn parse_node_def(
    module: &mut CodeModule,
    pairs: Pairs<Rule>,
) -> Result<NodeDefStatement, CodeExecError> {
    let mut name = None;
    let mut args = None;
    let mut body = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::identifier => name = Some(pair.as_str().to_owned()),
            Rule::identifier_tuple => args = Some(parse_identifier_tuple(pair.into_inner())?),
            Rule::stmt_block => body = Some(parse_stmt_block(module, pair.into_inner())?),
            _ => unreachable!(),
        }
    }
    Ok(NodeDefStatement {
        name: name.unwrap(),
        args: args.unwrap(),
        body: body.unwrap(),
    })
}

pub fn parse_stmt(module: &mut CodeModule, pairs: Pairs<Rule>) -> Result<Statement, CodeExecError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::stmt_end => continue,
            Rule::rm_stmt => return Ok(Statement::Rm(parse_rm(module, pair.into_inner())?)),
            Rule::ass_stmt => return Ok(Statement::Ass(parse_ass(pair.into_inner())?)),
            Rule::expr => {
                return Ok(Statement::Expr(ExprStatement {
                    expr: parse_expr(pair.into_inner()),
                }))
            }
            Rule::node_def_stmt => {
                return Ok(Statement::NodeDef(parse_node_def(
                    module,
                    pair.into_inner(),
                )?))
            }
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
