macro_rules! aggregator_builder {
    ($type: ty) => {
        pub struct AggregatorBuilder {
            op: Operation,
            initial: Option<$type>,
            tasks: Vec<TaskSource>,
            refs: Vec<String>,
        }

        #[allow(dead_code)]
        impl AggregatorBuilder {
            pub fn new() -> Self {
                Self {
                    op: Operation::Add,
                    initial: None,
                    tasks: vec![],
                    refs: vec![],
                }
            }
            pub fn add_named_task<S: Into<String>>(&mut self, name: S) -> &mut Self {
                self.refs.push(name.into());
                self
            }

            pub fn add_task<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
                self.tasks.push(task.into());
                self
            }

            pub fn build(&self) -> Aggregator {
                Aggregator {
                    op: self.op,
                    initial: match self.initial {
                        Some(x) => x,
                        _ => match self.op {
                            Operation::Div | Operation::Mul => 1.0,
                            Operation::Min => <$type>::MAX,
                            Operation::Max => <$type>::MIN,
                            _ => 0.0,
                        },
                    },
                    sources: self.tasks.clone(),
                }
            }

            /// Link named tasks to their task tree values
            pub fn link(&mut self, tree: &TaskTree) -> &mut Self {
                for name in &self.refs {
                    if let Some(task) = tree.get(name) {
                        self.tasks.push(task.clone());
                    }
                }
                self.refs.clear();
                self
            }

            pub fn initial<V: Into<$type>>(&mut self, value: V) -> &mut Self {
                self.initial = Some(value.into());
                self
            }

            pub fn operation(&mut self, op: Operation) -> &mut Self {
                self.op = op;
                self
            }
        }
    };
}

pub mod f32 {
    use super::super::{f32::Aggregator, Operation};
    use crate::task::f32::{TaskSource, TaskTree};
    aggregator_builder!(f32);
}

pub mod f64 {
    use super::super::{f64::Aggregator, Operation};
    use crate::task::f64::{TaskSource, TaskTree};
    aggregator_builder!(f64);
}
