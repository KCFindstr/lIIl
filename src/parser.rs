use pest::{
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};
use pest_derive::Parser;

use crate::{module::CodeModule, parser::module::parse_module, statement::CodeExecError};

pub mod expr;
pub mod module;
pub mod stmt;

#[derive(Parser)]
#[grammar = "lIIl.pest"]
#[allow(non_camel_case_types)]
struct lIIlParser;

pub fn parse(module: &mut CodeModule, input: &str) -> Result<(), CodeExecError> {
    let pairs = lIIlParser::parse(Rule::module, input)
        .map_err(|e| CodeExecError::new_str(format!("Syntax error: {:?}", e)))?;
    println!("{:?}", pairs);
    parse_module(module, pairs)
}
