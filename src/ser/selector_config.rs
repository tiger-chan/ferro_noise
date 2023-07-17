macro_rules! selector_config {
    ($type: ty) => {
        use serde::{Deserialize, Serialize};

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

        impl IntoTaskSource for SelectorConfig {
            fn config_into(&self, tree: &TaskTree) -> TaskSource {
                let mut builder = SelectorBuilder::new();

                let blender: Blender = match self.interp {
                    FractalBlender::Cubic => math::cubic_curve,
                    FractalBlender::Linear => math::linear_curve,
                    FractalBlender::Quintic => math::quintic_curve,
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
    };
}

pub mod f32 {
    use crate::math::f32 as math;
    use crate::ser::f32::{
        add_task_to_builder, push_named_to_vec, FractalBlender, IntoTaskSource, NameOrConst,
        TaskDependencies,
    };
    use crate::source::f32::Blender;
    use crate::task::f32::{SelectorBuilder, TaskSource, TaskTree};

    selector_config!(f32);
}

pub mod f64 {
    use crate::math::f64 as math;
    use crate::ser::f64::{
        add_task_to_builder, push_named_to_vec, FractalBlender, IntoTaskSource, NameOrConst,
        TaskDependencies,
    };
    use crate::source::f64::Blender;
    use crate::task::f64::{SelectorBuilder, TaskSource, TaskTree};

    selector_config!(f64);
}

#[cfg(test)]
mod test {
    mod f32 {
        use crate::ser::f32::{FractalBlender, SelectorConfig, TaskConfig};
        use std::collections::HashMap;

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
}
