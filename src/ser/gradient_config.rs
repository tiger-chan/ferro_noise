macro_rules! gradient_config {
    ($type: ty) => {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
        #[serde(default)]
        pub struct GradientConfig {
            pub x1: $type,
            pub y1: $type,
            pub z1: $type,
            pub x2: $type,
            pub y2: $type,
            pub z2: $type,
            pub cached: bool,
        }

        impl Default for GradientConfig {
            fn default() -> Self {
                Self {
                    x1: 0.0,
                    y1: 0.0,
                    z1: 0.0,
                    x2: 1.0,
                    y2: 1.0,
                    z2: 0.0,
                    cached: false,
                }
            }
        }

        impl TaskDependencies for GradientConfig {
            fn dependencies(&self) -> Vec<String> {
                vec![]
            }
        }

        impl IntoTaskSource for GradientConfig {
            fn config_into(&self, _: &TaskTree) -> TaskSource {
                let mut builder = GradientBuilder::new();

                builder
                    .s1([self.x1, self.y1, self.z1])
                    .s2([self.x2, self.y2, self.z2]);

                builder.build().into()
            }
        }
    };
}

pub mod f32 {
    use crate::ser::f32::{IntoTaskSource, TaskDependencies};
    use crate::task::f32::{GradientBuilder, TaskSource, TaskTree};
    gradient_config!(f32);
}

pub mod f64 {
    use crate::ser::f64::{IntoTaskSource, TaskDependencies};
    use crate::task::f64::{GradientBuilder, TaskSource, TaskTree};
    gradient_config!(f64);
}

#[cfg(test)]
mod test {
    mod f32 {
        use crate::ser::f32::{GradientConfig, TaskConfig};
        use std::collections::HashMap;

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [gradient_a]
                gradient.x1 = 1
                gradient.y1 = 2
                gradient.z1 = 3
                gradient.x2 = 4
                gradient.y2 = 5
                gradient.z2 = 6
                gradient.cached = true

                [gradient_b]
                gradient = { x1 = 2, y1 = 2, x2 = 4, y2 = 4 }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

            assert_eq!(config.len(), 2);
            assert_eq!(
                config["gradient_a"],
                TaskConfig::Gradient(GradientConfig {
                    x1: 1.0,
                    x2: 4.0,
                    y1: 2.0,
                    y2: 5.0,
                    z1: 3.0,
                    z2: 6.0,
                    cached: true,
                    ..Default::default()
                })
            );

            assert_eq!(
                config["gradient_b"],
                TaskConfig::Gradient(GradientConfig {
                    x1: 2.0,
                    x2: 4.0,
                    y1: 2.0,
                    y2: 4.0,
                    ..Default::default()
                })
            );
        }
    }

    mod f64 {
        use crate::ser::f64::{GradientConfig, TaskConfig};
        use std::collections::HashMap;

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [gradient_a]
                gradient.x1 = 1
                gradient.y1 = 2
                gradient.z1 = 3
                gradient.x2 = 4
                gradient.y2 = 5
                gradient.z2 = 6
                gradient.cached = true

                [gradient_b]
                gradient = { x1 = 2, y1 = 2, x2 = 4, y2 = 4 }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

            assert_eq!(config.len(), 2);
            assert_eq!(
                config["gradient_a"],
                TaskConfig::Gradient(GradientConfig {
                    x1: 1.0,
                    x2: 4.0,
                    y1: 2.0,
                    y2: 5.0,
                    z1: 3.0,
                    z2: 6.0,
                    cached: true,
                    ..Default::default()
                })
            );

            assert_eq!(
                config["gradient_b"],
                TaskConfig::Gradient(GradientConfig {
                    x1: 2.0,
                    x2: 4.0,
                    y1: 2.0,
                    y2: 4.0,
                    ..Default::default()
                })
            );
        }
    }
}
