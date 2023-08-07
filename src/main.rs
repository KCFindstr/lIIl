#![allow(non_snake_case)] // For package name lIIl.
use clap::Parser;
use module::CodeModule;
mod builtin;
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
    let mut module = CodeModule::new("lIIl", source_file.as_str());
    let _program = parser::parse(&mut module, &source).unwrap();
}
