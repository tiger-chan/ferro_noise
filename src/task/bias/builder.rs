use crate::{float::Float, task::NameOrSource};

use super::{super::TaskTree, Bias, TaskSource};

pub struct BiasBuilder<T: Float> {
    bias: NameOrSource<T>,
    source: NameOrSource<T>,
    min: T,
    max: T,
}

impl<T: Float> Default for BiasBuilder<T> {
    fn default() -> Self {
        Self {
            bias: NameOrSource::Source(T::ZERO.into()),
            source: NameOrSource::Source(T::ZERO.into()),
            min: T::ONE,
            max: T::from(4.0),
        }
    }
}

#[allow(dead_code)]
impl<T: Float> BiasBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bias<V: Into<TaskSource<T>>>(&mut self, task: V) -> &mut Self {
        self.bias = NameOrSource::Source(task.into());
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
            max: T::from(4.0),
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

    pub fn source<V: Into<TaskSource<T>>>(&mut self, task: V) -> &mut Self {
        self.source = NameOrSource::Source(task.into());
        self
    }
}
