macro_rules! bias_config {
    ($type: ty) => {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
        #[serde(default)]
        pub struct BiasConfig {
            pub bias: NameOrConst,
            pub source: NameOrConst,
            pub min: $type,
            pub max: $type,
            pub cached: bool,
        }

        impl Default for BiasConfig {
            fn default() -> Self {
                Self {
                    bias: 0.0.into(),
                    source: 0.0.into(),
                    min: 1.0,
                    max: 4.0,
                    cached: false,
                }
            }
        }

        impl TaskDependencies for BiasConfig {
            fn dependencies(&self) -> Vec<String> {
                let mut r = vec![];
                push_named_to_vec!(r, self.bias);
                push_named_to_vec!(r, self.source);
                r
            }
        }

        impl IntoTaskSource for BiasConfig {
            fn config_into(&self, tree: &TaskTree) -> TaskSource {
                let mut builder = BiasBuilder::new();

                builder.min(self.min).max(self.max);

                match &self.source {
                    NameOrConst::Named(x) => {
                        builder.named_source(x);
                    }
                    NameOrConst::Value(x) => {
                        builder.source(<$type>::from(*x));
                    }
                }

                match &self.bias {
                    NameOrConst::Named(x) => {
                        builder.named_bias(x);
                    }
                    NameOrConst::Value(x) => {
                        builder.bias(<$type>::from(*x));
                    }
                }

                builder.link(tree).build().into()
            }
        }
    };
}

pub mod f32 {
    use crate::ser::f32::{push_named_to_vec, IntoTaskSource, NameOrConst, TaskDependencies};
    use crate::task::f32::{BiasBuilder, TaskSource, TaskTree};
    bias_config!(f32);
}

pub mod f64 {
    use crate::ser::f64::{push_named_to_vec, IntoTaskSource, NameOrConst, TaskDependencies};
    use crate::task::f64::{BiasBuilder, TaskSource, TaskTree};
    bias_config!(f64);
}

#[cfg(test)]
mod test {
    mod f32 {
        use std::collections::HashMap;

        use crate::ser::f32::{BiasConfig, TaskConfig};

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [bias_a]
                bias.min = 2
                bias.max = 5.0
                bias.source = "other"
                bias.bias = 1.0
                bias.cached = true

                [bias_b]
                bias = { bias = "other", source = 1 }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

            assert_eq!(config.len(), 2);
            assert_eq!(
                config["bias_a"],
                TaskConfig::Bias(BiasConfig {
                    min: 2.0,
                    max: 5.0,
                    source: "other".to_owned().into(),
                    bias: 1.0.into(),
                    cached: true,
                    ..Default::default()
                })
            );

            assert_eq!(
                config["bias_b"],
                TaskConfig::Bias(BiasConfig {
                    source: 1.0.into(),
                    bias: "other".to_owned().into(),
                    ..Default::default()
                })
            );
        }
    }

    mod f64 {
        use std::collections::HashMap;

        use crate::ser::f64::{BiasConfig, TaskConfig};

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [bias_a]
                bias.min = 2
                bias.max = 5.0
                bias.source = "other"
                bias.bias = 1.0
                bias.cached = true

                [bias_b]
                bias = { bias = "other", source = 1 }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

            assert_eq!(config.len(), 2);
            assert_eq!(
                config["bias_a"],
                TaskConfig::Bias(BiasConfig {
                    min: 2.0,
                    max: 5.0,
                    source: "other".to_owned().into(),
                    bias: 1.0.into(),
                    cached: true,
                    ..Default::default()
                })
            );

            assert_eq!(
                config["bias_b"],
                TaskConfig::Bias(BiasConfig {
                    source: 1.0.into(),
                    bias: "other".to_owned().into(),
                    ..Default::default()
                })
            );
        }
    }
}
