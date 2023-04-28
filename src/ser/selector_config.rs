use serde::{Deserialize, Serialize};

use super::{fractal_config::FractalBlender, name_or_const::NameOrConst};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(default)]
pub struct SelectorConfig {
    #[serde(alias = "blender")]
    #[serde(alias = "curve")]
    interp: FractalBlender,
    condition: NameOrConst,
    lower: NameOrConst,
    upper: NameOrConst,
    falloff: NameOrConst,
    /// threadhold/pivot/boundry to determine when lower or upper is used
    threshold: NameOrConst,
}

impl Default for SelectorConfig {
    fn default() -> Self {
        Self {
            interp: FractalBlender::default(),
            condition: 0.0.into(),
            lower: (-1.0).into(),
            upper: 1.0.into(),
            falloff: 0.0.into(),
            threshold: 0.0.into(),
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
            [selector_a]
            selector.interp = "linear"
            selector.condition = -1
            selector.upper = 2.0

            [selector_b]
            selector = { falloff = "other", threshold = 1, condition = "con" }
        })
        .unwrap();
        let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

        assert_eq!(config.len(), 2);
        assert_eq!(
            config["selector_a"],
            TaskConfig::Selector(SelectorConfig {
                interp: FractalBlender::Linear,
                condition: (-1.0).into(),
                upper: 2.0.into(),
                ..Default::default()
            })
        );

        assert_eq!(
            config["selector_b"],
            TaskConfig::Selector(SelectorConfig {
                falloff: "other".to_owned().into(),
                threshold: 1.0.to_owned().into(),
                condition: "con".to_owned().into(),
                ..Default::default()
            })
        );
    }
}
