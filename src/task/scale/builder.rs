macro_rules! scale_builder {
    ($type: ty) => {
        pub struct ScaleBuilder {
            scale: NameOrSource,
            source: NameOrSource,
        }

        impl Default for ScaleBuilder {
            fn default() -> Self {
                Self {
                    scale: NameOrSource::Source(0.0.into()),
                    source: NameOrSource::Source(0.0.into()),
                }
            }
        }

        #[allow(dead_code)]
        impl ScaleBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn scale<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
                self.scale = NameOrSource::Source(task.into());
                self
            }

            pub fn build(&self) -> Scale {
                Scale {
                    scale: source_or_message!(self.scale, ScaleBuilder),
                    source: source_or_message!(self.source, ScaleBuilder),
                }
            }

            /// Link named tasks to their task tree values
            pub fn link(&mut self, tree: &TaskTree) -> &mut Self {
                match &self.scale {
                    NameOrSource::Named(name) => {
                        if let Some(task) = tree.get(name) {
                            self.scale = NameOrSource::Source(task.clone());
                        }
                    }
                    _ => {}
                }

                match &self.source {
                    NameOrSource::Named(name) => {
                        if let Some(task) = tree.get(name) {
                            self.source = NameOrSource::Source(task.clone());
                        }
                    }
                    _ => {}
                }

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
    use crate::task::f32::{NameOrSource, Scale, TaskSource, TaskTree};
    use crate::task::task::source_or_message;
    scale_builder!(f32);
}

pub mod f64 {
    use crate::task::f64::{NameOrSource, Scale, TaskSource, TaskTree};
    use crate::task::task::source_or_message;
    scale_builder!(f64);
}
