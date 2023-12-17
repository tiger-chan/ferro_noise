use crate::task::{named_to_task, source_or_message};

macro_rules! selector_builder_type {
    ($type: ty) => {
        pub struct SelectorBuilder {
            blender: Blender,
            condition: NameOrSource,
            lower: NameOrSource,
            upper: NameOrSource,
            falloff: NameOrSource,
            /// threshold/pivot/boundry to determine when lower or upper is used
            threshold: NameOrSource,
        }

        impl Default for SelectorBuilder {
            fn default() -> Self {
                use NameOrSource::Source;
                Self {
                    blender: math::linear_curve,
                    condition: Source(0.0.into()),
                    lower: Source((-1.0).into()),
                    upper: Source(1.0.into()),
                    falloff: Source(0.0.into()),
                    threshold: Source(0.5.into()),
                }
            }
        }

        impl SelectorBuilder {
            pub fn blender(&mut self, blender: Blender) -> &mut Self {
                self.blender = blender;
                self
            }

            pub fn build(&mut self) -> Selector {
                Selector {
                    blender: self.blender,
                    condition: source_or_message!(self.condition, SelectorBuilder),
                    lower: source_or_message!(self.lower, SelectorBuilder),
                    upper: source_or_message!(self.upper, SelectorBuilder),
                    falloff: source_or_message!(self.falloff, SelectorBuilder),
                    threshold: source_or_message!(self.threshold, SelectorBuilder),
                }
            }

            pub fn condition<V: Into<TaskSource>>(&mut self, condition: V) -> &mut Self {
                self.condition = NameOrSource::Source(condition.into());
                self
            }

            pub fn falloff<V: Into<TaskSource>>(&mut self, falloff: V) -> &mut Self {
                self.falloff = NameOrSource::Source(falloff.into());
                self
            }

            /// Link named tasks to their task tree values
            pub fn link(&mut self, tree: &TaskTree) -> &mut Self {
                named_to_task!(self.condition, tree);
                named_to_task!(self.falloff, tree);
                named_to_task!(self.lower, tree);
                named_to_task!(self.threshold, tree);
                named_to_task!(self.upper, tree);

                self
            }

            pub fn lower<V: Into<TaskSource>>(&mut self, lower: V) -> &mut Self {
                self.lower = NameOrSource::Source(lower.into());
                self
            }

            pub fn named_condition<S: Into<String>>(&mut self, name: S) -> &mut Self {
                self.condition = NameOrSource::Named(name.into());
                self
            }

            pub fn named_falloff<S: Into<String>>(&mut self, name: S) -> &mut Self {
                self.falloff = NameOrSource::Named(name.into());
                self
            }

            pub fn named_lower<S: Into<String>>(&mut self, name: S) -> &mut Self {
                self.lower = NameOrSource::Named(name.into());
                self
            }

            pub fn named_threshold<S: Into<String>>(&mut self, name: S) -> &mut Self {
                self.threshold = NameOrSource::Named(name.into());
                self
            }

            pub fn named_upper<S: Into<String>>(&mut self, name: S) -> &mut Self {
                self.upper = NameOrSource::Named(name.into());
                self
            }

            pub fn threshold<V: Into<TaskSource>>(&mut self, threshold: V) -> &mut Self {
                self.threshold = NameOrSource::Source(threshold.into());
                self
            }

            pub fn upper<V: Into<TaskSource>>(&mut self, upper: V) -> &mut Self {
                self.upper = NameOrSource::Source(upper.into());
                self
            }
        }
    };
}

pub mod f32 {
    use super::*;
    use crate::math::f32 as math;
    use crate::source::f32::Blender;
    use crate::task::f32::{NameOrSource, Selector, TaskSource, TaskTree};
    selector_builder_type!(f32);
}

pub mod f64 {
    use super::*;
    use crate::math::f64 as math;
    use crate::source::f64::Blender;
    use crate::task::f64::{NameOrSource, Selector, TaskSource, TaskTree};
    selector_builder_type!(f64);
}
