use std::collections::HashMap;

use crate::float::Float;

use super::{task::TaskSource, Task};

pub struct TaskTree<T: Float> {
    tasks: HashMap<String, TaskSource<T>>,
    rendered_task: String,
}

#[allow(dead_code)]
impl<T: Float> TaskTree<T> {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            rendered_task: String::new(),
        }
    }

    pub fn add_task<S: Into<String>>(&mut self, name: S, task: TaskSource<T>) {
        self.tasks.insert(name.into(), task);
    }

    pub fn set_rednered_task<S: Into<String>>(&mut self, name: S) {
        self.rendered_task = name.into();
    }

    pub fn get<S: Into<String>>(&self, name: S) -> Option<&TaskSource<T>> {
        self.tasks.get(&name.into())
    }
}

impl<T: Float> Task<T> for TaskTree<T> {
    fn sample_1d(&mut self, x: T) -> T {
        if let Some(task) = &mut self.tasks.get_mut(&self.rendered_task) {
            task.sample_1d(x)
        } else {
            T::ZERO
        }
    }
    fn sample_2d(&mut self, x: T, y: T) -> T {
        if let Some(task) = &mut self.tasks.get_mut(&self.rendered_task) {
            task.sample_2d(x, y)
        } else {
            T::ZERO
        }
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        if let Some(task) = &mut self.tasks.get_mut(&self.rendered_task) {
            task.sample_3d(x, y, z)
        } else {
            T::ZERO
        }
    }
}

#[cfg(test)]
mod tests {
    //use crate::task::FractalType;

    use crate::task::{
        task::TaskSource::{Aggregate, Constant},
        AggregatorBuilder, Operation,
    };

    use super::*;

    #[test]
    fn aggregate_named_result() {
        let mut tree = TaskTree::<f32>::new();

        tree.add_task("task 1", Constant(1.0));

        tree.add_task(
            "task 2",
            Aggregate(
                AggregatorBuilder::new()
                    .operation(Operation::Add)
                    .add_named_task("task 1")
                    .add_named_task("task 1")
                    .link(&tree)
                    .build(),
            ),
        );

        tree.set_rednered_task("task 2");

        assert_eq!(tree.sample_1d(1.0), 2.0);
    }

    #[test]
    fn constant_result() {
        let mut tree = TaskTree::<f32>::new();

        assert_eq!(tree.sample_1d(1.0), 0.0);

        tree.add_task("task 1", Constant(1.0));

        tree.set_rednered_task("task 1");

        assert_eq!(tree.sample_1d(1.0), 1.0);
    }
}
