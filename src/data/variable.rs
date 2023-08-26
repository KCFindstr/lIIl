use std::fmt::{self, Display};

use super::data::Array;

#[derive(Debug, Clone, Default)]
pub enum VarType {
    #[default]
    Nzero,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Tuple(Array),
    Ref(i64),
}

impl Display for VarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VarType::Nzero => write!(f, "N0"),
            VarType::Bool(val) => write!(f, "{}", val),
            VarType::Int(val) => write!(f, "{}", val),
            VarType::Float(val) => write!(f, "{}", val),
            VarType::String(val) => write!(f, "{}", val),
            VarType::Tuple(val) => write!(f, "{:?}", val),
            VarType::Ref(val) => write!(f, "Ref({})", val),
        }
    }
}
