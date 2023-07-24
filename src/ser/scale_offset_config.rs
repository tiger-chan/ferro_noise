macro_rules! scale_offset_config {
    () => {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
        #[serde(default)]
        pub struct ScaleOffsetConfig {
            pub offset: NameOrConst,
            pub scale: NameOrConst,
            pub source: NameOrConst,
            pub cached: bool,
        }

        impl Default for ScaleOffsetConfig {
            fn default() -> Self {
                Self {
                    offset: 0.0.into(),
                    scale: 1.0.into(),
                    source: 0.0.into(),
                    cached: false,
                }
            }
        }

        impl TaskDependencies for ScaleOffsetConfig {
            fn dependencies(&self) -> Vec<String> {
                let mut r = vec![];
                push_named_to_vec!(r, self.offset);
                push_named_to_vec!(r, self.scale);
                push_named_to_vec!(r, self.source);
                r
            }
        }

        impl IntoTaskSource for ScaleOffsetConfig {
            fn config_into(&self, tree: &TaskTree) -> TaskSource {
                let mut builder = ScaleOffsetBuilder::new();

                add_task_to_builder!(self.offset, builder, offset, named_offset, tree);
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
    use crate::task::f32::{ScaleOffsetBuilder, TaskSource, TaskTree};
    scale_offset_config!();
}

pub mod f64 {
    use crate::ser::f64::{
        add_task_to_builder, push_named_to_vec, IntoTaskSource, NameOrConst, TaskDependencies,
    };
    use crate::task::f64::{ScaleOffsetBuilder, TaskSource, TaskTree};
    scale_offset_config!();
}

#[cfg(test)]
mod test {
    mod f32 {
        use std::collections::HashMap;

        use crate::ser::f32::{ScaleOffsetConfig, TaskConfig};

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [scale_offset_a]
                scale_offset.source = "other"
                scale_offset.offset = 2.0
                scale_offset.scale = 1.0
                scale_offset.cached = true

                [scale_offset_b]
                scale_offset = { scale = "other", source = 1 }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

            assert_eq!(config.len(), 2);
            assert_eq!(
                config["scale_offset_a"],
                TaskConfig::ScaleOffset(ScaleOffsetConfig {
                    source: "other".to_owned().into(),
                    scale: 1.0.into(),
                    offset: 2.0.into(),
                    cached: true,
                    ..Default::default()
                })
            );

            assert_eq!(
                config["scale_offset_b"],
                TaskConfig::ScaleOffset(ScaleOffsetConfig {
                    source: 1.0.into(),
                    scale: "other".to_owned().into(),
                    ..Default::default()
                })
            );
        }
    }

    mod f64 {
        use std::collections::HashMap;

        use crate::ser::f64::{ScaleOffsetConfig, TaskConfig};

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [scale_offset_a]
                scale_offset.source = "other"
                scale_offset.offset = 2.0
                scale_offset.scale = 1.0
                scale_offset.cached = true

                [scale_offset_b]
                scale_offset = { scale = "other", source = 1 }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

            assert_eq!(config.len(), 2);
            assert_eq!(
                config["scale_offset_a"],
                TaskConfig::ScaleOffset(ScaleOffsetConfig {
                    source: "other".to_owned().into(),
                    scale: 1.0.into(),
                    offset: 2.0.into(),
                    cached: true,
                    ..Default::default()
                })
            );

            assert_eq!(
                config["scale_offset_b"],
                TaskConfig::ScaleOffset(ScaleOffsetConfig {
                    source: 1.0.into(),
                    scale: "other".to_owned().into(),
                    ..Default::default()
                })
            );
        }
    }
}
