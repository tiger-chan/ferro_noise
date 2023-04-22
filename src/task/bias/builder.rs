use crate::float::Float;

use super::{super::TaskTree, Bias, TaskSource};

enum NameOrSource<T: Float> {
    Named(String),
    Source(TaskSource<T>),
}

pub struct BiasBuilder<T: Float> {
    bias: NameOrSource<T>,
    source: NameOrSource<T>,
    min: T,
    max: T,
}

impl<T: Float> Default for BiasBuilder<T> {
    fn default() -> Self {
        Self {
            bias: NameOrSource::Source(TaskSource::Constant(T::ZERO)),
            source: NameOrSource::Source(TaskSource::Constant(T::ZERO)),
            min: T::ONE,
            max: T::from(4),
        }
    }
}

#[allow(dead_code)]
impl<T: Float> BiasBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bias(&mut self, task: TaskSource<T>) -> &mut Self {
        self.bias = NameOrSource::Source(task);
        self
    }

    pub fn build(&self) -> Bias<T> {
        Bias {
            bias: match &self.bias {
                NameOrSource::Source(x) => x.clone(),
                _ => panic!("BiasBuilder::link must be called if BiasBuilder::named_bias is used"),
            },
            source: match &self.source {
                NameOrSource::Source(x) => x.clone(),
                _ => panic!("BiasBuilder::link must be called if BiasBuilder::named_bias is used"),
            },
            min: T::ONE,
            max: T::from(4),
        }
    }

    /// Link named tasks to their task tree values
    pub fn link(&mut self, tree: &TaskTree<T>) -> &mut Self {
        match &self.bias {
            NameOrSource::Named(name) => {
                if let Some(task) = tree.get(name) {
                    self.bias = NameOrSource::Source(task.clone());
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

    pub fn max(&mut self, max: T) -> &mut Self {
        self.max = max;
        self
    }

    pub fn min(&mut self, min: T) -> &mut Self {
        self.min = min;
        self
    }

    pub fn named_bias<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.bias = NameOrSource::Named(name.into());
        self
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
