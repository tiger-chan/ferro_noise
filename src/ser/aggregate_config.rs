use super::name_or_const::NameOrConst;
use crate::task::Operation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case", default)]
pub struct AggregateConfig {
    operator: Operation,
    initial: f64,
    source: Vec<NameOrConst>,
}

impl Default for AggregateConfig {
    fn default() -> Self {
        Self {
            operator: Operation::default(),
            initial: 0.0,
            source: vec![0.0.into()],
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::ser::TaskConfig;

    use super::*;

    #[test]
    fn deserialize() {
        let data = toml::to_string(&toml::toml! {
            [aggregate_a]
            aggregate.operator = "sub"
            aggregate.initial = 1.0
            aggregate.source = [1.0, "other"]

            [aggregate_b]
            aggregate = { operator = "mul", source = [123] }
        })
        .unwrap();
        let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

        assert_eq!(config.len(), 2);
        assert_eq!(
            config["aggregate_a"],
            TaskConfig::Aggregate(AggregateConfig {
                initial: 1.0,
                operator: Operation::Sub,
                source: vec![1.0.into(), "other".to_owned().into()],
                ..Default::default()
            })
        );

        assert_eq!(
            config["aggregate_b"],
            TaskConfig::Aggregate(AggregateConfig {
                initial: 0.0,
                operator: Operation::Mul,
                source: vec![123.0.into(),],
                ..Default::default()
            })
        );
    }
}
