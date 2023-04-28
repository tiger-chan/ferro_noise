use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(default)]
pub struct GradientConfig {
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
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
            [gradient_a]
            gradient.x1 = 1
            gradient.y1 = 2
            gradient.z1 = 3
            gradient.x2 = 4
            gradient.y2 = 5
            gradient.z2 = 6

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
