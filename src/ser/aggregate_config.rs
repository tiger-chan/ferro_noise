use super::{name_or_const::NameOrConst, IntoTaskSource, TaskDependencies};
use crate::{
    float::Float,
    task::{AggregatorBuilder, Operation},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case", default)]
pub struct AggregateConfig {
    pub operator: Operation,
    pub initial: f64,
    pub source: Vec<NameOrConst>,
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

impl TaskDependencies for AggregateConfig {
    fn dependencies(&self) -> Vec<String> {
        self.source
            .iter()
            .filter(|x| x.is_named())
            .map(|x| match x {
                NameOrConst::Named(x) => x.clone(),
                _ => String::new(),
            })
            .collect()
    }
}

impl<T: Float> IntoTaskSource<T> for AggregateConfig {
    fn config_into(&self, tree: &crate::task::TaskTree<T>) -> crate::task::TaskSource<T> {
        let mut builder = AggregatorBuilder::<T>::new();

        builder
            .initial(T::as_float(self.initial))
            .operation(self.operator);

        for source in &self.source {
            match source {
                NameOrConst::Named(x) => {
                    builder.add_named_task(x);
                }
                NameOrConst::Value(x) => {
                    builder.add_task(T::as_float(*x));
                }
            }
        }

        builder.link(tree).build().into()
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
