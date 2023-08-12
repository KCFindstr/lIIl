#![allow(non_snake_case)]
use std::{cell::RefCell, rc::Rc};

// For package name lIIl.
use clap::Parser;
use data::context::Context;
use module::CodeModule;
mod data;
mod expr;
mod module;
mod parser;
mod statement;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    source_file: String,
}

fn main() {
    let args = Args::parse();
    let source_file: String = args.source_file;
    let source = std::fs::read_to_string(source_file.as_str()).unwrap();
    let context = Context::root_rc();
    let mut module = CodeModule::new("lIIl", source_file.as_str(), &context);
    let _program = parser::parse(&mut module, &source).unwrap();
}
