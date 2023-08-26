// For package name lIIl.
#![allow(non_snake_case)]

use clap::Parser;
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
    let module = parser::parse_file(&args.source_file, None).unwrap();
}
