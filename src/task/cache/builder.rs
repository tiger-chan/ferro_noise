use crate::{
    float::Float,
    task::{TaskSource, TaskTree, NameOrSource},
};

use super::Cache;

pub const MAX_CACHE_ENTRY: usize = 3;

pub struct CacheBuilder<T: Float> {
    source: NameOrSource<T>,
}

impl<T: Float> Default for CacheBuilder<T> {
    fn default() -> Self {
        Self {
            source: NameOrSource::Source(T::ZERO.into()),
        }
    }
}

#[allow(dead_code)]
impl<T: Float> CacheBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> Cache<T> {
        Cache {
            store: [None; MAX_CACHE_ENTRY],
            source: match &self.source {
                NameOrSource::Source(x) => x.clone(),
                _ => panic!("CacheBuilder::link must be called if CacheBuilder::source is used"),
            },
        }
    }

    /// Link named tasks to their task tree values
    pub fn link(&mut self, tree: &TaskTree<T>) -> &mut Self {
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

    pub fn source<V: Into<TaskSource<T>>>(&mut self, task: V) -> &mut Self {
        self.source = NameOrSource::Source(task.into());
        self
    }
}
