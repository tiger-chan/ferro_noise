use std::collections::HashMap;

use crate::float::Float;

use super::{task::TaskSource, Task};

pub struct TaskTree<T: Float> {
    tasks: HashMap<String, TaskSource<T>>,
}

#[allow(dead_code)]
impl<T: Float> TaskTree<T> {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    pub fn add_task<S: Into<String>, V: Into<TaskSource<T>>>(&mut self, name: S, task: V) {
        self.tasks.insert(name.into(), task.into());
    }

    pub fn get<S: Into<String>>(&self, name: S) -> Option<&TaskSource<T>> {
        self.tasks.get(&name.into())
    }

    pub fn sample_1d<S: Into<String>>(&mut self, name: S, x: T) -> T {
        if let Some(task) = &mut self.tasks.get_mut(&name.into()) {
            task.sample_1d(x)
        } else {
            T::ZERO
        }
    }
    pub fn sample_2d<S: Into<String>>(&mut self, name: S, x: T, y: T) -> T {
        if let Some(task) = &mut self.tasks.get_mut(&name.into()) {
            task.sample_2d(x, y)
        } else {
            T::ZERO
        }
    }

    pub fn sample_3d<S: Into<String>>(&mut self, name: S, x: T, y: T, z: T) -> T {
        if let Some(task) = &mut self.tasks.get_mut(&name.into()) {
            task.sample_3d(x, y, z)
        } else {
            T::ZERO
        }
    }
}

#[cfg(test)]
mod tests {
    //use crate::task::FractalType;

    use crate::task::{AggregatorBuilder, BiasBuilder, CacheBuilder, Operation, SelectorBuilder};

    use super::*;

    #[test]
    fn aggregate_named_result() {
        let mut tree = TaskTree::<f32>::new();

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
        let mut tree = TaskTree::<f32>::new();

        tree.add_task("task 1", 1.0);

        assert_eq!(tree.sample_1d("task 1", 1.0), 1.0);
    }

    #[test]
    fn bias_result() {
        let mut tree = TaskTree::<f32>::new();

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
        let mut tree = TaskTree::<f32>::new();

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
        let mut tree = TaskTree::<f32>::new();

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
