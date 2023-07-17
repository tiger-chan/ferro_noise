macro_rules! task_tree {
    ($type: ty) => {
        pub struct TaskTree {
            tasks: HashMap<String, TaskSource>,
        }

        impl TaskTree {
            pub fn new() -> Self {
                Self {
                    tasks: HashMap::new(),
                }
            }

            pub fn add_task<S: Into<String>, V: Into<TaskSource>>(&mut self, name: S, task: V) {
                self.tasks.insert(name.into(), task.into());
            }

            pub fn get<S: Into<String>>(&self, name: S) -> Option<&TaskSource> {
                self.tasks.get(&name.into())
            }

            pub fn get_mut<S: Into<String>>(&mut self, name: S) -> Option<&mut TaskSource> {
                self.tasks.get_mut(&name.into())
            }

            pub fn sample_1d<S: Into<String>>(&mut self, name: S, x: $type) -> $type {
                if let Some(task) = &mut self.tasks.get_mut(&name.into()) {
                    task.sample_1d(x)
                } else {
                    0.0
                }
            }
            pub fn sample_2d<S: Into<String>>(&mut self, name: S, x: $type, y: $type) -> $type {
                if let Some(task) = &mut self.tasks.get_mut(&name.into()) {
                    task.sample_2d(x, y)
                } else {
                    0.0
                }
            }

            pub fn sample_3d<S: Into<String>>(
                &mut self,
                name: S,
                x: $type,
                y: $type,
                z: $type,
            ) -> $type {
                if let Some(task) = &mut self.tasks.get_mut(&name.into()) {
                    task.sample_3d(x, y, z)
                } else {
                    0.0
                }
            }
        }
    };
}

pub mod f32 {
    use super::super::f32::{Task, TaskSource};
    use std::collections::HashMap;
    task_tree!(f32);
}

pub mod f64 {
    use super::super::f64::{Task, TaskSource};
    use std::collections::HashMap;
    task_tree!(f64);
}

#[cfg(test)]
mod tests {

    mod f32 {
        use crate::task::f32::{
            AggregatorBuilder, BiasBuilder, CacheBuilder, Operation, SelectorBuilder, TaskTree,
        };

        #[test]
        fn aggregate_named_result() {
            let mut tree = TaskTree::new();

            tree.add_task("task 1", 1.0);

            tree.add_task(
                "task 2",
                AggregatorBuilder::new()
                    .operation(Operation::Add)
                    .add_named_task("task 1")
                    .add_named_task("task 1")
                    .link(&tree)
                    .build(),
            );

            assert_eq!(tree.sample_1d("task 2", 1.0), 2.0);
        }

        #[test]
        fn constant_result() {
            let mut tree = TaskTree::new();

            tree.add_task("task 1", 1.0);

            assert_eq!(tree.sample_1d("task 1", 1.0), 1.0);
        }

        #[test]
        fn bias_result() {
            let mut tree = TaskTree::new();

            tree.add_task("task 1", 1.0);
            tree.add_task("task 2", 0.0);

            tree.add_task(
                "task 3",
                BiasBuilder::new()
                    .named_source("task 1")
                    .named_bias("task 2")
                    .link(&tree)
                    .build(),
            );

            assert_eq!(tree.sample_1d("task 3", 1.0), 1.0);
        }

        #[test]
        fn cache_result() {
            let mut tree = TaskTree::new();

            tree.add_task("task 1", 1.0);

            tree.add_task(
                "task 2",
                CacheBuilder::new()
                    .named_source("task 1")
                    .link(&tree)
                    .build(),
            );

            assert_eq!(tree.sample_1d("task 2", 1.0), 1.0);
        }

        #[test]
        fn selector_result() {
            let mut tree = TaskTree::new();

            tree.add_task("task 1", 1.0);
            tree.add_task("task 2", 0.0);
            tree.add_task("task 3", 0.0);
            tree.add_task("task 4", 0.5);
            tree.add_task("task 5", 1.0);

            tree.add_task(
                "task 6",
                SelectorBuilder::new()
                    .named_condition("task 1")
                    .named_falloff("task 2")
                    .named_lower("task 3")
                    .named_threshold("task 4")
                    .named_upper("task 5")
                    .link(&tree)
                    .build(),
            );

            assert_eq!(tree.sample_1d("task 6", 1.0), 1.0);
        }
    }

    mod f64 {
        use crate::task::f64::{
            AggregatorBuilder, BiasBuilder, CacheBuilder, Operation, SelectorBuilder, TaskTree,
        };

        #[test]
        fn aggregate_named_result() {
            let mut tree = TaskTree::new();

            tree.add_task("task 1", 1.0);

            tree.add_task(
                "task 2",
                AggregatorBuilder::new()
                    .operation(Operation::Add)
                    .add_named_task("task 1")
                    .add_named_task("task 1")
                    .link(&tree)
                    .build(),
            );

            assert_eq!(tree.sample_1d("task 2", 1.0), 2.0);
        }

        #[test]
        fn constant_result() {
            let mut tree = TaskTree::new();

            tree.add_task("task 1", 1.0);

            assert_eq!(tree.sample_1d("task 1", 1.0), 1.0);
        }

        #[test]
        fn bias_result() {
            let mut tree = TaskTree::new();

            tree.add_task("task 1", 1.0);
            tree.add_task("task 2", 0.0);

            tree.add_task(
                "task 3",
                BiasBuilder::new()
                    .named_source("task 1")
                    .named_bias("task 2")
                    .link(&tree)
                    .build(),
            );

            assert_eq!(tree.sample_1d("task 3", 1.0), 1.0);
        }

        #[test]
        fn cache_result() {
            let mut tree = TaskTree::new();

            tree.add_task("task 1", 1.0);

            tree.add_task(
                "task 2",
                CacheBuilder::new()
                    .named_source("task 1")
                    .link(&tree)
                    .build(),
            );

            assert_eq!(tree.sample_1d("task 2", 1.0), 1.0);
        }

        #[test]
        fn selector_result() {
            let mut tree = TaskTree::new();

            tree.add_task("task 1", 1.0);
            tree.add_task("task 2", 0.0);
            tree.add_task("task 3", 0.0);
            tree.add_task("task 4", 0.5);
            tree.add_task("task 5", 1.0);

            tree.add_task(
                "task 6",
                SelectorBuilder::new()
                    .named_condition("task 1")
                    .named_falloff("task 2")
                    .named_lower("task 3")
                    .named_threshold("task 4")
                    .named_upper("task 5")
                    .link(&tree)
                    .build(),
            );

            assert_eq!(tree.sample_1d("task 6", 1.0), 1.0);
        }
    }
}
