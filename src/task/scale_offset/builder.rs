macro_rules! scale_offset_builder {
    ($type: ty) => {
        pub struct ScaleOffsetBuilder {
            offset: NameOrSource,
            scale: NameOrSource,
            source: NameOrSource,
        }

        impl Default for ScaleOffsetBuilder {
            fn default() -> Self {
                Self {
                    offset: NameOrSource::Source(0.0.into()),
                    scale: NameOrSource::Source(1.0.into()),
                    source: NameOrSource::Source(0.0.into()),
                }
            }
        }

        #[allow(dead_code)]
        impl ScaleOffsetBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn offset<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
                self.offset = NameOrSource::Source(task.into());
                self
            }

            pub fn scale<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
                self.scale = NameOrSource::Source(task.into());
                self
            }

            pub fn build(&self) -> ScaleOffset {
                ScaleOffset {
                    offset: source_or_message!(self.offset, ScaleOffsetBuilder),
                    scale: source_or_message!(self.scale, ScaleOffsetBuilder),
                    source: source_or_message!(self.source, ScaleOffsetBuilder),
                }
            }

            /// Link named tasks to their task tree values
            pub fn link(&mut self, tree: &TaskTree) -> &mut Self {
                named_to_task!(self.offset, tree);
                named_to_task!(self.scale, tree);
                named_to_task!(self.source, tree);

                self
            }

            pub fn named_offset<S: Into<String>>(&mut self, name: S) -> &mut Self {
                self.offset = NameOrSource::Named(name.into());
                self
            }

            pub fn named_scale<S: Into<String>>(&mut self, name: S) -> &mut Self {
                self.scale = NameOrSource::Named(name.into());
                self
            }

            pub fn named_source<S: Into<String>>(&mut self, name: S) -> &mut Self {
                self.source = NameOrSource::Named(name.into());
                self
            }

            pub fn source<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
                self.source = NameOrSource::Source(task.into());
                self
            }
        }
    };
}

pub mod f32 {
    use crate::task::f32::{NameOrSource, ScaleOffset, TaskSource, TaskTree};
    use crate::task::{named_to_task, source_or_message};
    scale_offset_builder!(f32);
}

pub mod f64 {
    use crate::task::f64::{NameOrSource, ScaleOffset, TaskSource, TaskTree};
    use crate::task::{named_to_task, source_or_message};
    scale_offset_builder!(f64);
}
