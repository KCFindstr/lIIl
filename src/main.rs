#![allow(non_snake_case)]
use std::fs;

// For package name lIIl.
use clap::Parser;
use data::context::Context;
use module::CodeModule;
mod data;
mod expr;
mod module;
mod parser;
mod statement;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    source_file: String,
}

fn main() {
    let args = Args::parse();
    let source_file_path = fs::canonicalize(args.source_file).unwrap();
    let source_file = source_file_path.to_str().unwrap();
    let source = std::fs::read_to_string(source_file).unwrap();
    let context = Context::root_rc();
    let mut module = CodeModule::new("lIIl", source_file, &context);
    let _program = parser::parse(&mut module, &source).unwrap();
}
