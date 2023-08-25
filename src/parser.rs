use std::fs;

use pest::Parser;
use pest_derive::Parser;

use crate::{
    data::context::Context, module::CodeModule, parser::module::parse_module,
    statement::CodeExecError,
};

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

pub fn parse_file(file: &str) -> Result<CodeModule, CodeExecError> {
    let abs_file_path = fs::canonicalize(file).unwrap();
    let abs_file = abs_file_path.to_str().unwrap();
    let context = Context::root_rc();
    let input = std::fs::read_to_string(file)
        .map_err(|e| CodeExecError::new_str(format!("IO error: {:?}", e)))?;
    let mut module = CodeModule::new("lIIl", abs_file, &context);
    parse(&mut module, &input)?;
    Ok(module)
}
