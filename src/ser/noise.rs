use crate::{float::Float, task::TaskSource};

use super::{
    aggregate_config::*, bias_config::*, fractal_config::*, gradient_config::*, selector_config::*,
    transform_domain_config::*,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TaskConfig {
    Aggregate(AggregateConfig),
    Bias(BiasConfig),
    Cache(String),
    Constant(f64),
    Fractal(FractalConfig),
    Gradient(GradientConfig),
    Selector(SelectorConfig),
    TransformDomain(TransformDomainConfig),
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self::Constant(0.0)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct NoiseConfig {
    pub task: Vec<TaskConfig>,
}

pub fn from_str(data: impl Into<String>) -> Result<Box<TaskSource<f64>>, String> {
    let _config: NoiseConfig = toml::from_str(&data.into()).unwrap();
    Ok(Box::new(TaskSource::Constant(f64::ZERO)))
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn generic_parse() {
        let data = toml::to_string(&toml::toml! {
            [const_a]
            constant = 1.0

            [cache_b]
            cache = "fractal_a"

            [fractal_a]
            fractal = { octaves = 1, frequency = 0.5, source = "perlin" }
        })
        .unwrap();
        let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

        assert_eq!(config.len(), 3);
        assert_eq!(config["const_a"], TaskConfig::Constant(1.0));
        assert_eq!(config["cache_b"], TaskConfig::Cache("fractal_a".to_owned()));
        assert_eq!(
            config["fractal_a"],
            TaskConfig::Fractal(FractalConfig {
                octaves: 1,
                frequency: 0.5,
                source: FractalSource::Perlin,
                ..Default::default()
            })
        );
    }
}
