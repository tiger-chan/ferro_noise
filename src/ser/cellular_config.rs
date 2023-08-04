macro_rules! gradient_config {
    ($T: ty) => {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
        #[serde(default)]
        pub struct CellularConfig {
            pub x: $T,
            pub y: $T,
            pub z: $T,
            pub seed: u64,
            pub cached: bool,
        }

        impl Default for CellularConfig {
            fn default() -> Self {
                Self {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                    seed: 0,
                    cached: false,
                }
            }
        }

        impl TaskDependencies for CellularConfig {
            fn dependencies(&self) -> Vec<String> {
                vec![]
            }
        }

        impl IntoTaskSource for CellularConfig {
            fn config_into(&self, _: &TaskTree) -> TaskSource {
                let mut builder = CellularBuilder::new();

                builder.seed(self.seed).spacing([self.x, self.y, self.z]);

                builder.build().into()
            }
        }
    };
}

pub mod f32 {
    use crate::ser::f32::{IntoTaskSource, TaskDependencies};
    use crate::task::f32::{CellularBuilder, TaskSource, TaskTree};
    gradient_config!(f32);
}

pub mod f64 {
    use crate::ser::f64::{IntoTaskSource, TaskDependencies};
    use crate::task::f64::{CellularBuilder, TaskSource, TaskTree};
    gradient_config!(f64);
}

#[cfg(test)]
mod test {
    mod f32 {
        use crate::ser::f32::{CellularConfig, TaskConfig};
        use std::collections::HashMap;

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [cellular_a]
                cellular.x = 5
                cellular.cached = true

                [cellular_b]
                cellular = { y = 4, z = 45, seed = 1234 }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

            assert_eq!(config.len(), 2);
            assert_eq!(
                config["cellular_a"],
                TaskConfig::Cellular(CellularConfig {
                    x: 5.0,
                    cached: true,
                    ..Default::default()
                })
            );

            assert_eq!(
                config["cellular_b"],
                TaskConfig::Cellular(CellularConfig {
                    y: 4.0,
                    z: 45.0,
                    seed: 1234,
                    ..Default::default()
                })
            );
        }
    }

    mod f64 {
        use crate::ser::f64::{CellularConfig, TaskConfig};
        use std::collections::HashMap;

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [cellular_a]
                cellular.x = 5
                cellular.cached = true

                [cellular_b]
                cellular = { y = 4, z = 45, seed = 1234 }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

            assert_eq!(config.len(), 2);
            assert_eq!(
                config["cellular_a"],
                TaskConfig::Cellular(CellularConfig {
                    x: 5.0,
                    cached: true,
                    ..Default::default()
                })
            );

            assert_eq!(
                config["cellular_b"],
                TaskConfig::Cellular(CellularConfig {
                    y: 4.0,
                    z: 45.0,
                    seed: 1234,
                    ..Default::default()
                })
            );
        }
    }
}
