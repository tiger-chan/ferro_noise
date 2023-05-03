use serde::{Deserialize, Serialize};

use crate::{
    float::Float,
    math::{cubic_curve, linear_curve, quintic_curve},
    source::Blender,
    task::SelectorBuilder,
};

use super::{fractal_config::FractalBlender, name_or_const::*, IntoTaskSource, TaskDependencies};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(default)]
pub struct SelectorConfig {
    #[serde(alias = "blender")]
    #[serde(alias = "curve")]
    pub interp: FractalBlender,
    pub condition: NameOrConst,
    pub lower: NameOrConst,
    pub upper: NameOrConst,
    pub falloff: NameOrConst,
    /// threadhold/pivot/boundry to determine when lower or upper is used
    pub threshold: NameOrConst,
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

impl TaskDependencies for SelectorConfig {
    fn dependencies(&self) -> Vec<String> {
        let mut r = vec![];
        push_named_to_vec!(r, self.condition);
        push_named_to_vec!(r, self.lower);
        push_named_to_vec!(r, self.upper);
        push_named_to_vec!(r, self.falloff);
        push_named_to_vec!(r, self.threshold);
        r
    }
}

impl<T: Float> IntoTaskSource<T> for SelectorConfig {
    fn config_into(&self, tree: &crate::task::TaskTree<T>) -> crate::task::TaskSource<T> {
        let mut builder = SelectorBuilder::<T>::new();

        let blender: Blender<T> = match self.interp {
            FractalBlender::Cubic => cubic_curve::<T>,
            FractalBlender::Linear => linear_curve::<T>,
            FractalBlender::Quintic => quintic_curve::<T>,
        };

        builder.blender(blender);
        add_task_to_builder!(self.condition, builder, condition, named_condition, tree);
        add_task_to_builder!(self.lower, builder, lower, named_lower, tree);
        add_task_to_builder!(self.upper, builder, upper, named_upper, tree);
        add_task_to_builder!(self.falloff, builder, falloff, named_falloff, tree);
        add_task_to_builder!(self.threshold, builder, threshold, named_threshold, tree);

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
