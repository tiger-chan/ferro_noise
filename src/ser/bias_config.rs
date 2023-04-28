use serde::{Deserialize, Serialize};

use super::name_or_const::NameOrConst;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(default)]
pub struct BiasConfig {
    bias: NameOrConst,
    source: NameOrConst,
    min: f64,
    max: f64,
}

impl Default for BiasConfig {
    fn default() -> Self {
        Self {
            bias: 0.0.into(),
            source: 0.0.into(),
            min: 1.0,
            max: 4.0,
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
            [bias_a]
            bias.min = 2
            bias.max = 5.0
            bias.source = "other"
            bias.bias = 1.0

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
