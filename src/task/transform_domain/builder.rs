use crate::{
    float::Float,
    task::{named_to_task, source_or_message, NameOrSource, TaskSource, TaskTree},
};

use super::{DomainOperation, TransformDomain};

pub struct TransformDomainBuilder<T: Float> {
    operation: DomainOperation,
    value_x: NameOrSource<T>,
    value_y: NameOrSource<T>,
    value_z: NameOrSource<T>,
    source: NameOrSource<T>,
}

impl<T: Float> Default for TransformDomainBuilder<T> {
    fn default() -> Self {
        Self {
            operation: DomainOperation::Translate,
            value_x: NameOrSource::Source(T::ZERO.into()),
            value_y: NameOrSource::Source(T::ZERO.into()),
            value_z: NameOrSource::Source(T::ZERO.into()),
            source: NameOrSource::Source(T::ZERO.into()),
        }
    }
}

impl<T: Float> TransformDomainBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> TransformDomain<T> {
        TransformDomain {
            source: source_or_message!(self.source, TransformDomainBuilder<T>),
            operation: self.operation,
            value: [
                source_or_message!(self.value_x, TransformDomainBuilder<T>),
                source_or_message!(self.value_y, TransformDomainBuilder<T>),
                source_or_message!(self.value_z, TransformDomainBuilder<T>),
            ],
        }
    }

    pub fn link(&mut self, tree: &TaskTree<T>) -> &mut Self {
        named_to_task!(self.value_x, tree);
        named_to_task!(self.value_y, tree);
        named_to_task!(self.value_z, tree);
        named_to_task!(self.source, tree);
        self
    }

    pub fn named_source<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.source = NameOrSource::Named(name.into());
        self
    }

    pub fn named_value_x<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.value_z = NameOrSource::Named(name.into());
        self
    }

    pub fn named_value_y<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.value_y = NameOrSource::Named(name.into());
        self
    }

    pub fn named_value_z<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.value_z = NameOrSource::Named(name.into());
        self
    }

    pub fn operation(&mut self, operation: DomainOperation) -> &mut Self {
        self.operation = operation;
        self
    }

    pub fn source<V: Into<TaskSource<T>>>(&mut self, task: V) -> &mut Self {
        self.source = NameOrSource::Source(task.into());
        self
    }

    pub fn value_x<V: Into<TaskSource<T>>>(&mut self, task: V) -> &mut Self {
        self.value_x = NameOrSource::Source(task.into());
        self
    }

    pub fn value_y<V: Into<TaskSource<T>>>(&mut self, task: V) -> &mut Self {
        self.value_y = NameOrSource::Source(task.into());
        self
    }

    pub fn value_z<V: Into<TaskSource<T>>>(&mut self, task: V) -> &mut Self {
        self.value_z = NameOrSource::Source(task.into());
        self
    }
}
