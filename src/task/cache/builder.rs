pub const MAX_CACHE_ENTRY: usize = 3;

macro_rules! cache_builder {
    ($type: ty) => {
        pub struct CacheBuilder {
            source: NameOrSource,
        }

        impl Default for CacheBuilder {
            fn default() -> Self {
                Self {
                    source: NameOrSource::Source(0.0.into()),
                }
            }
        }

        impl CacheBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn build(&self) -> Cache {
                Cache {
                    store: [None; MAX_CACHE_ENTRY],
                    source: match &self.source {
                        NameOrSource::Source(x) => x.clone(),
                        _ => panic!(
                            "CacheBuilder::link must be called if CacheBuilder::source is used"
                        ),
                    },
                }
            }

            /// Link named tasks to their task tree values
            pub fn link(&mut self, tree: &TaskTree) -> &mut Self {
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

            pub fn named_source<S: Into<String>>(&mut self, name: S) -> &mut Self {
                self.source = NameOrSource::Named(name.into());
                self
            }

			#[allow(dead_code)]
            pub fn source<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
                self.source = NameOrSource::Source(task.into());
                self
            }
        }
    };
}

pub mod f32 {
    use super::MAX_CACHE_ENTRY;
    use crate::task::f32::{NameOrSource, TaskSource};
    use crate::task::{cache::f32::Cache, task_tree::f32::TaskTree};
    cache_builder!(f32);
}

pub mod f64 {
    use super::MAX_CACHE_ENTRY;
    use crate::task::f64::{NameOrSource, TaskSource};
    use crate::task::{cache::f64::Cache, task_tree::f64::TaskTree};
    cache_builder!(f64);
}
