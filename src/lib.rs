// For package name lIIl.
#![allow(non_snake_case)]

pub mod data;
pub mod expr;
pub mod module;
pub mod parser;
pub mod statement;
pub mod utils;

#[cfg(test)]
mod test_utils {
    use crate::{module::CodeModule, parser::parse_file};

    pub fn exec_program(file: &str) -> CodeModule {
        let path = format!("tests/{}", file);
        let mut module = parse_file(&path, None).unwrap();
        module.exec().unwrap();
        module
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::test_utils::exec_program;

    #[test]
    fn test_assert() {
        exec_program("assert.lIIl");
    }

    #[test]
    fn test_that() {
        exec_program("that.lIIl");
    }

    #[test]
    fn test_return() {
        exec_program("return.lIIl");
    }

    #[test]
    fn test_not() {
        exec_program("not.lIIl");
    }

    #[test]
    fn test_make() {
        exec_program("make.lIIl");
    }

    #[test]
    fn test_lib() {
        exec_program("lib/main.lIIl");
    }
}

#[cfg(test)]
mod program_tests {
    use crate::test_utils::exec_program;

    #[test]
    fn test_gcd() {
        exec_program("programs/gcd.lIIl");
    }
}
