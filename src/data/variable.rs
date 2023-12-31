use std::fmt::{self, Debug};

use super::data::{Array, MemDataRc};

#[derive(Clone, Default)]
pub enum VarType {
    #[default]
    Nzero,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Tuple(Array),
    Ref(MemDataRc),
}

impl Debug for VarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VarType::Nzero => write!(f, "N0"),
            VarType::Bool(val) => write!(f, "Bool({})", val),
            VarType::Int(val) => write!(f, "Int({})", val),
            VarType::Float(val) => write!(f, "Float({})", val),
            VarType::String(val) => write!(f, "String({})", val),
            VarType::Tuple(val) => write!(f, "Tuple({:?})", val),
            VarType::Ref(val) => write!(f, "Ref({:?})", val.borrow()),
        }
    }
}

impl ToString for VarType {
    fn to_string(&self) -> String {
        match self {
            VarType::Nzero => "N0".to_owned(),
            VarType::Bool(val) => format!("{}", val),
            VarType::Int(val) => format!("{}", val),
            VarType::Float(val) => format!("{}", val),
            VarType::String(val) => format!("{}", val),
            VarType::Tuple(val) => format!("{:?}", val),
            VarType::Ref(val) => format!("{:?}", val.borrow()),
        }
    }
}

impl From<&VarType> for bool {
    fn from(value: &VarType) -> Self {
        match value {
            VarType::Nzero => false,
            VarType::Bool(val) => *val,
            VarType::Int(val) => *val != 0,
            VarType::Float(val) => *val != 0.0,
            VarType::String(val) => val != "",
            VarType::Tuple(val) => val.len() != 0,
            VarType::Ref(_val) => true,
        }
    }
}

impl From<VarType> for bool {
    fn from(value: VarType) -> Self {
        (&value).into()
    }
}
