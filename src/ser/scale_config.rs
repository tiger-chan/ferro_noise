macro_rules! scale_config {
    () => {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
        #[serde(default)]
        pub struct ScaleConfig {
            pub scale: NameOrConst,
            pub source: NameOrConst,
            pub cached: bool,
        }

        impl Default for ScaleConfig {
            fn default() -> Self {
                Self {
                    scale: 1.0.into(),
                    source: 0.0.into(),
                    cached: false,
                }
            }
        }

        impl TaskDependencies for ScaleConfig {
            fn dependencies(&self) -> Vec<String> {
                let mut r = vec![];
                push_named_to_vec!(r, self.scale);
                push_named_to_vec!(r, self.source);
                r
            }
        }

        impl IntoTaskSource for ScaleConfig {
            fn config_into(&self, tree: &TaskTree) -> TaskSource {
                let mut builder = ScaleBuilder::new();

                add_task_to_builder!(self.source, builder, source, named_source, tree);
                add_task_to_builder!(self.scale, builder, scale, named_scale, tree);

                builder.link(tree).build().into()
            }
        }
    };
}

pub mod f32 {
    use crate::ser::f32::{
        add_task_to_builder, push_named_to_vec, IntoTaskSource, NameOrConst, TaskDependencies,
    };
    use crate::task::f32::{ScaleBuilder, TaskSource, TaskTree};
    scale_config!();
}

pub mod f64 {
    use crate::ser::f64::{
        add_task_to_builder, push_named_to_vec, IntoTaskSource, NameOrConst, TaskDependencies,
    };
    use crate::task::f64::{ScaleBuilder, TaskSource, TaskTree};
    scale_config!();
}

#[cfg(test)]
mod test {
    mod f32 {
        use std::collections::HashMap;

        use crate::ser::f32::{ScaleConfig, TaskConfig};

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [scale_a]
                scale.source = "other"
                scale.scale = 1.0
                scale.cached = true

                [scale_b]
                scale = { scale = "other", source = 1 }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

            assert_eq!(config.len(), 2);
            assert_eq!(
                config["scale_a"],
                TaskConfig::Scale(ScaleConfig {
                    source: "other".to_owned().into(),
                    scale: 1.0.into(),
                    cached: true,
                })
            );

            assert_eq!(
                config["scale_b"],
                TaskConfig::Scale(ScaleConfig {
                    source: 1.0.into(),
                    scale: "other".to_owned().into(),
                    ..Default::default()
                })
            );
        }
    }

    mod f64 {
        use std::collections::HashMap;

        use crate::ser::f64::{ScaleConfig, TaskConfig};

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [scale_a]
                scale.source = "other"
                scale.scale = 1.0
                scale.cached = true

                [scale_b]
                scale = { scale = "other", source = 1 }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

            assert_eq!(config.len(), 2);
            assert_eq!(
                config["scale_a"],
                TaskConfig::Scale(ScaleConfig {
                    source: "other".to_owned().into(),
                    scale: 1.0.into(),
                    cached: true,
                })
            );

            assert_eq!(
                config["scale_b"],
                TaskConfig::Scale(ScaleConfig {
                    source: 1.0.into(),
                    scale: "other".to_owned().into(),
                    ..Default::default()
                })
            );
        }
    }
}
