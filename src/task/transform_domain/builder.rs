use super::DomainOperation;

macro_rules! transform_domain_builder {
    ($type: ty) => {
        pub struct TransformDomainBuilder {
            operation: DomainOperation,
            value_x: NameOrSource,
            value_y: NameOrSource,
            value_z: NameOrSource,
            source: NameOrSource,
        }

        impl Default for TransformDomainBuilder {
            fn default() -> Self {
                Self {
                    operation: DomainOperation::Translate,
                    value_x: NameOrSource::Source(0.0.into()),
                    value_y: NameOrSource::Source(0.0.into()),
                    value_z: NameOrSource::Source(0.0.into()),
                    source: NameOrSource::Source(0.0.into()),
                }
            }
        }

        impl TransformDomainBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn build(&self) -> TransformDomain {
                TransformDomain {
                    source: source_or_message!(self.source, TransformDomainBuilder),
                    operation: self.operation,
                    value: [
                        source_or_message!(self.value_x, TransformDomainBuilder),
                        source_or_message!(self.value_y, TransformDomainBuilder),
                        source_or_message!(self.value_z, TransformDomainBuilder),
                    ],
                }
            }

            pub fn link(&mut self, tree: &TaskTree) -> &mut Self {
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

            pub fn source<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
                self.source = NameOrSource::Source(task.into());
                self
            }

            pub fn value_x<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
                self.value_x = NameOrSource::Source(task.into());
                self
            }

            pub fn value_y<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
                self.value_y = NameOrSource::Source(task.into());
                self
            }

            pub fn value_z<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
                self.value_z = NameOrSource::Source(task.into());
                self
            }
        }
    };
}

pub mod f32 {
    use super::DomainOperation;
    use crate::task::{
        f32::{NameOrSource, TaskSource, TaskTree, TransformDomain},
        named_to_task, source_or_message,
    };
    transform_domain_builder!(f32);
}

pub mod f64 {
    use super::DomainOperation;
    use crate::task::{
        f64::{NameOrSource, TaskSource, TaskTree, TransformDomain},
        named_to_task, source_or_message,
    };
    transform_domain_builder!(f64);
}
