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
