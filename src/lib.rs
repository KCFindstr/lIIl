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
        exec_program("unit/assert.lIIl");
    }

    #[test]
    fn test_that() {
        exec_program("unit/that.lIIl");
    }

    #[test]
    fn test_return() {
        exec_program("unit/return.lIIl");
    }

    #[test]
    fn test_not() {
        exec_program("unit/not.lIIl");
    }

    #[test]
    fn test_make() {
        exec_program("unit/make.lIIl");
    }

    #[test]
    fn test_object() {
        exec_program("unit/object.lIIl");
    }

    #[test]
    fn test_string_index() {
        exec_program("unit/string_index.lIIl");
    }

    #[test]
    fn test_rm() {
        exec_program("unit/rm.lIIl");
    }

    #[test]
    fn test_object_not() {
        exec_program("unit/object_not.lIIl");
    }

    #[test]
    fn test_lib() {
        exec_program("unit/lib/main.lIIl");
    }
}

#[cfg(test)]
mod program_tests {
    use crate::test_utils::exec_program;

    #[test]
    fn test_gcd() {
        exec_program("programs/gcd.lIIl");
    }

    #[test]
    fn test_tp() {
        exec_program("programs/tp.lIIl");
    }

    #[test]
    fn test_ll() {
        exec_program("programs/ll_test.lIIl");
    }

    #[test]
    fn test_empty_call() {
        exec_program("programs/empty_call.lIIl");
    }

    #[test]
    fn test_ai() {
        crate::module::ai::mock_input(["hello world", "42", "first", "second"]);
        exec_program("programs/ai.lIIl");
    }

    #[test]
    fn test_dijkstra_small() {
        let content = std::fs::read_to_string("tests/data/graph1.txt").unwrap();
        let lines: Vec<&str> = content.lines().collect();
        crate::module::ai::mock_input(lines);
        exec_program("programs/dijkstra.lIIl");
    }

    #[test]
    fn test_dijkstra_large() {
        let content = std::fs::read_to_string("tests/data/graph_large.txt").unwrap();
        let lines: Vec<&str> = content.lines().collect();
        crate::module::ai::mock_input(lines);
        exec_program("programs/dijkstra.lIIl");
    }

    #[test]
    fn test_map_reduce() {
        exec_program("programs/map_reduce.lIIl");
    }
}
