use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum NameOrConst {
    Named(String),
    Value(f64),
}

impl Default for NameOrConst {
    fn default() -> Self {
        Self::Value(0.0)
    }
}

impl From<f64> for NameOrConst {
    fn from(value: f64) -> Self {
        NameOrConst::Value(value)
    }
}

impl From<String> for NameOrConst {
    fn from(value: String) -> Self {
        NameOrConst::Named(value)
    }
}

impl NameOrConst {
    pub fn is_named(&self) -> bool {
        match &self {
            NameOrConst::Named(_) => true,
            _ => false,
        }
    }
}

macro_rules! push_named_to_vec {
    ($vec:expr, $val:expr) => {
        match &$val {
            NameOrConst::Named(x) => $vec.push(x.clone()),
            _ => {}
        }
    };
}

pub(crate) use push_named_to_vec;

macro_rules! add_task_to_builder {
    ($val:expr, $builder:expr, $func:ident, $named:ident, $tree:expr) => {
        match &$val {
            NameOrConst::Named(x) => $builder.$named(x),
            NameOrConst::Value(x) => $builder.$func(T::as_float(*x)),
        };
    };
}

pub(crate) use add_task_to_builder;
