use pest::iterators::Pairs;

use crate::{
    module::CodeModule,
    parser::{expr::parse_lvalue, literal::parse_identifier_tuple},
    statement::{
        ass::AssStatement, expr::ExprStatement, if_stmt::IfStatement, loli::LoliStatement,
        maybe::MaybeStatement, node_def::NodeDefStatement, ret::ReturnStatement, rm::RmStatement,
        CodeExecError, Statement, Statements,
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
            Rule::stmt => stmts.push(parse_stmt(module, pair.into_inner())?),
            _ => panic!("parse_stmt_block: {:?}", pair),
        }
    }
    Ok(stmts)
}

fn parse_rm(module: &mut CodeModule, pairs: Pairs<Rule>) -> Result<RmStatement, CodeExecError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::package_name => return Ok(RmStatement::new(module, pair.as_str())),
            _ => panic!("parse_rm: {:?}", pair),
        }
    }
    panic!("parse_rm: Reached end of input")
}

fn parse_ass(pairs: Pairs<Rule>) -> Result<AssStatement, CodeExecError> {
    let mut lhs = None;
    let mut rhs = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::lvalue => lhs = Some(parse_lvalue(pair.into_inner())?),
            Rule::expr => rhs = Some(parse_expr(pair.into_inner())),
            _ => panic!("parse_ass: {:?}", pair),
        }
    }
    Ok(AssStatement {
        lhs: lhs.unwrap(),
        rhs: rhs.unwrap(),
    })
}

fn parse_if(module: &mut CodeModule, pairs: Pairs<Rule>) -> Result<IfStatement, CodeExecError> {
    let mut cond = None;
    let mut body = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => cond = Some(parse_expr(pair.into_inner())),
            Rule::stmt => body = Some(parse_stmt(module, pair.into_inner())?),
            _ => panic!("parse_ass: {:?}", pair),
        }
    }
    Ok(IfStatement {
        cond: cond.unwrap(),
        body: Box::new(body.unwrap()),
    })
}

fn parse_loli(module: &mut CodeModule, pairs: Pairs<Rule>) -> Result<LoliStatement, CodeExecError> {
    let mut cond = None;
    let mut body = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => cond = Some(parse_expr(pair.into_inner())),
            Rule::stmt => body = Some(parse_stmt(module, pair.into_inner())?),
            _ => panic!("parse_loli: {:?}", pair),
        }
    }
    Ok(LoliStatement {
        cond: cond.unwrap(),
        body: Box::new(body.unwrap()),
    })
}

fn parse_ret(pairs: Pairs<Rule>) -> Result<ReturnStatement, CodeExecError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                return Ok(ReturnStatement {
                    value: parse_expr(pair.into_inner()),
                })
            }
            _ => panic!("parse_ret: {:?}", pair),
        }
    }
    panic!("parse_ret: Reached end of input")
}

fn parse_maybe(
    module: &mut CodeModule,
    pairs: Pairs<Rule>,
) -> Result<MaybeStatement, CodeExecError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::stmt => {
                return Ok(MaybeStatement {
                    body: Box::new(parse_stmt(module, pair.into_inner())?),
                })
            }
            _ => panic!("parse_maybe: {:?}", pair),
        }
    }
    panic!("parse_maybe: Reached end of input")
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
            Rule::stmt => body = Some(parse_stmt(module, pair.into_inner())?),
            _ => panic!("parse_node_def: {:?}", pair),
        }
    }
    Ok(NodeDefStatement {
        name: name.unwrap(),
        args: args.unwrap(),
        body: Box::new(body.unwrap()),
    })
}

pub fn parse_stmt(module: &mut CodeModule, pairs: Pairs<Rule>) -> Result<Statement, CodeExecError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::rm_stmt => return Ok(Statement::Rm(parse_rm(module, pair.into_inner())?)),
            Rule::ass_stmt => return Ok(Statement::Ass(parse_ass(pair.into_inner())?)),
            Rule::expr => {
                return Ok(Statement::Expr(ExprStatement {
                    value: parse_expr(pair.into_inner()),
                }))
            }
            Rule::node_def_stmt => {
                return Ok(Statement::NodeDef(parse_node_def(
                    module,
                    pair.into_inner(),
                )?))
            }
            Rule::if_stmt => return Ok(Statement::If(parse_if(module, pair.into_inner())?)),
            Rule::return_stmt => return Ok(Statement::Ret(parse_ret(pair.into_inner())?)),
            Rule::maybe_stmt => {
                return Ok(Statement::Maybe(parse_maybe(module, pair.into_inner())?))
            }
            Rule::loli_stmt => return Ok(Statement::Loli(parse_loli(module, pair.into_inner())?)),
            Rule::stmt_block => {
                return Ok(Statement::Stmts(parse_stmt_block(
                    module,
                    pair.into_inner(),
                )?))
            }
            Rule::stmt_end => continue,
            _ => panic!("parse_stmt: {:?}", pair),
        }
    }
    Ok(Statement::Stmts(Statements::new()))
}
