use serde::{Deserialize, Serialize};

use crate::{float::Float, task::BiasBuilder};

use super::{name_or_const::*, IntoTaskSource, TaskDependencies};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(default)]
pub struct BiasConfig {
    pub bias: NameOrConst,
    pub source: NameOrConst,
    pub min: f64,
    pub max: f64,
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

impl TaskDependencies for BiasConfig {
    fn dependencies(&self) -> Vec<String> {
        let mut r = vec![];
        push_named_to_vec!(r, self.bias);
        push_named_to_vec!(r, self.source);
        r
    }
}

impl<T: Float> IntoTaskSource<T> for BiasConfig {
    fn config_into(&self, tree: &crate::task::TaskTree<T>) -> crate::task::TaskSource<T> {
        let mut builder = BiasBuilder::<T>::new();

        builder
            .min(T::as_float(self.min))
            .max(T::as_float(self.max));

        match &self.source {
            NameOrConst::Named(x) => {
                builder.named_source(x);
            }
            NameOrConst::Value(x) => {
                builder.source(T::as_float(*x));
            }
        }

        match &self.bias {
            NameOrConst::Named(x) => {
                builder.named_bias(x);
            }
            NameOrConst::Value(x) => {
                builder.bias(T::as_float(*x));
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
