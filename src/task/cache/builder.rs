use crate::{float::Float, task::TaskSource};

use super::Cache;

pub const MAX_CACHE_ENTRY: usize = 3;

enum NameOrSource<T: Float> {
    Named(String),
    Source(TaskSource<T>),
}

pub struct CacheBuilder<T: Float> {
    source: NameOrSource<T>,
}

impl<T: Float> Default for CacheBuilder<T> {
    fn default() -> Self {
        Self {
            source: NameOrSource::Source(TaskSource::Constant(T::ZERO)),
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

    pub fn named_source<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.source = NameOrSource::Named(name.into());
        self
    }

    pub fn source(&mut self, task: TaskSource<T>) -> &mut Self {
        self.source = NameOrSource::Source(task);
        self
    }
}
