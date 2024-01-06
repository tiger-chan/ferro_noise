macro_rules! aggregate_config {
    ($type: ty) => {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
        #[serde(rename_all = "snake_case", default)]
        pub struct AggregateConfig {
            pub operator: Operation,
            pub initial: $type,
            pub source: Vec<NameOrConst>,
            pub cached: bool,
        }

        impl Default for AggregateConfig {
            fn default() -> Self {
                Self {
                    operator: Operation::default(),
                    initial: 0.0,
                    source: vec![0.0.into()],
                    cached: false,
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

        impl IntoTaskSource for AggregateConfig {
            fn config_into(&self, tree: &TaskTree) -> TaskSource {
                let mut builder = AggregatorBuilder::default();

                builder.initial(self.initial).operation(self.operator);

                for source in &self.source {
                    match source {
                        NameOrConst::Named(x) => {
                            builder.add_named_task(x);
                        }
                        NameOrConst::Value(x) => {
                            builder.add_task(<$type>::from(*x));
                        }
                    }
                }

                builder.link(tree).build().into()
            }
        }
    };
}

pub mod f32 {
    use crate::ser::f32::{IntoTaskSource, NameOrConst, TaskDependencies};
    use crate::task::f32::{AggregatorBuilder, Operation, TaskSource, TaskTree};
    aggregate_config!(f32);
}

pub mod f64 {
    use crate::ser::f64::{IntoTaskSource, NameOrConst, TaskDependencies};
    use crate::task::f64::{AggregatorBuilder, Operation, TaskSource, TaskTree};
    aggregate_config!(f64);
}

#[cfg(test)]
mod test {
    mod f32 {
        use crate::ser::f32::{AggregateConfig, TaskConfig};
        use crate::task::f32::Operation;
        use std::collections::HashMap;

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [aggregate_a]
                aggregate.operator = "sub"
                aggregate.initial = 1.0
                aggregate.source = [1.0, "other"]
                aggregate.cached = true

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
                    cached: true,
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

    mod f64 {
        use crate::ser::f64::{AggregateConfig, TaskConfig};
        use crate::task::f64::Operation;
        use std::collections::HashMap;

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [aggregate_a]
                aggregate.operator = "sub"
                aggregate.initial = 1.0
                aggregate.source = [1.0, "other"]
                aggregate.cached = true

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
                    cached: true,
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
}
