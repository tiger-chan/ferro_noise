use crate::float::Float;

use super::{task::TaskSource, Task, TaskTree};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Operation {
    Add,
    Avg,
    Sub,
    Mul,
    Div,
    Max,
    Min,
}

#[derive(Clone)]
pub struct Aggregator<T: Float> {
    op: Operation,
    initial: T,
    sources: Vec<TaskSource<T>>,
}

impl<T: Float> Default for Aggregator<T> {
    fn default() -> Self {
        Self {
            op: Operation::Add,
            initial: T::default(),
            sources: vec![],
        }
    }
}

impl<T: Float> Aggregator<T> {
    #[allow(dead_code)]
    pub fn new(op: Operation, initial: T, sources: Vec<TaskSource<T>>) -> Self {
        Self {
            op,
            initial,
            sources,
        }
    }

    fn eval<F>(&mut self, sampler: F) -> T
    where
        F: Fn(&mut TaskSource<T>) -> T,
    {
        let mut result = self.initial;
        for source in self.sources.iter_mut() {
            let val = sampler(source);
            result = match self.op {
                Operation::Add => result + val,
                Operation::Avg => result + val,
                Operation::Div => result / val,
                Operation::Max => {
                    if result < val {
                        val
                    } else {
                        result
                    }
                }
                Operation::Min => {
                    if result < val {
                        result
                    } else {
                        val
                    }
                }
                Operation::Mul => result * val,
                Operation::Sub => result - val,
            };
        }
        if self.op == Operation::Avg && self.sources.len() > 0 {
            result / T::from(self.sources.len() as f32)
        } else {
            result
        }
    }
}

impl<T: Float> Task<T> for Aggregator<T> {
    fn sample_1d(&mut self, x: T) -> T {
        self.eval(|s| (*s).sample_1d(x))
    }

    fn sample_2d(&mut self, x: T, y: T) -> T {
        self.eval(|s| (*s).sample_2d(x, y))
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        self.eval(|s| (*s).sample_3d(x, y, z))
    }
}

pub struct AggregatorBuilder<T: Float> {
    op: Operation,
    initial: Option<T>,
    tasks: Vec<TaskSource<T>>,
    refs: Vec<String>,
}

#[allow(dead_code)]
impl<T: Float> AggregatorBuilder<T> {
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

    pub fn add_task<V: Into<TaskSource<T>>>(&mut self, task: V) -> &mut Self {
        self.tasks.push(task.into());
        self
    }

    pub fn build(&self) -> Aggregator<T> {
        Aggregator {
            op: self.op,
            initial: match self.initial {
                Some(x) => x,
                _ => match self.op {
                    Operation::Div | Operation::Mul => T::ONE,
                    Operation::Min => T::MAX,
                    Operation::Max => T::MIN,
                    _ => T::ZERO,
                },
            },
            sources: self.tasks.clone(),
        }
    }

    /// Link named tasks to their task tree values
    pub fn link(&mut self, tree: &TaskTree<T>) -> &mut Self {
        for name in &self.refs {
            if let Some(task) = tree.get(name) {
                self.tasks.push(task.clone());
            }
        }
        self.refs.clear();
        self
    }

    pub fn initial<V: Into<T>>(&mut self, value: V) -> &mut Self {
        self.initial = Some(value.into());
        self
    }

    pub fn operation(&mut self, op: Operation) -> &mut Self {
        self.op = op;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregator_add_tests() {
        let mut result = AggregatorBuilder::<f64>::new()
            .operation(Operation::Add)
            .initial(0)
            .add_task(0.5)
            .add_task(0.5)
            .build();

        assert_eq!(result.sample_1d(1.0), 1.0);
        assert_eq!(result.sample_1d(2.0), 1.0);
        assert_eq!(result.sample_1d(3.0), 1.0);

        assert_eq!(result.sample_2d(1.0, 1.0), 1.0);
        assert_eq!(result.sample_2d(2.0, 2.0), 1.0);
        assert_eq!(result.sample_2d(3.0, 3.0), 1.0);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 1.0);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 1.0);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 1.0);

        let mut result = AggregatorBuilder::<f32>::new()
            .operation(Operation::Add)
            .initial(0.0)
            .add_task(0.5)
            .add_task(0.25)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.75);
        assert_eq!(result.sample_1d(2.0), 0.75);
        assert_eq!(result.sample_1d(3.0), 0.75);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.75);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.75);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.75);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.75);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.75);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.75);
    }

    #[test]
    fn aggregator_avg_tests() {
        let mut result = AggregatorBuilder::<f64>::new()
            .operation(Operation::Avg)
            .initial(0)
            .add_task(0.5)
            .add_task(0.5)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = AggregatorBuilder::<f32>::new()
            .operation(Operation::Avg)
            .initial(0u16)
            .add_task(0.5)
            .add_task(0.5)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);
    }

    #[test]
    fn aggregator_div_tests() {
        let mut result = AggregatorBuilder::<f64>::new()
            .operation(Operation::Div)
            .initial(2)
            .add_task(1.0)
            .add_task(4.0)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = AggregatorBuilder::<f32>::new()
            .operation(Operation::Div)
            .initial(2.0)
            .add_task(4.0)
            .add_task(1.0)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);
    }

    #[test]
    fn aggregator_max_tests() {
        let mut result = AggregatorBuilder::<f64>::new()
            .operation(Operation::Max)
            .add_task(0.2)
            .add_task(0.5)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = AggregatorBuilder::<f32>::new()
            .operation(Operation::Max)
            .initial(-2.0)
            .add_task(-0.05)
            .add_task(0.5)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);
    }

    #[test]
    fn aggregator_min_tests() {
        let mut result = AggregatorBuilder::<f64>::new()
            .operation(Operation::Min)
            .add_task(0.5)
            .add_task(3.0)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = AggregatorBuilder::<f32>::new()
            .operation(Operation::Min)
            .initial(2.0)
            .add_task(5.0)
            .add_task(0.5)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);
    }

    #[test]
    fn aggregator_mul_tests() {
        let mut result = AggregatorBuilder::<f64>::new()
            .operation(Operation::Mul)
            .add_task(0.5)
            .add_task(0.5)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.25);
        assert_eq!(result.sample_1d(2.0), 0.25);
        assert_eq!(result.sample_1d(3.0), 0.25);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.25);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.25);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.25);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.25);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.25);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.25);

        let mut result = AggregatorBuilder::<f32>::new()
            .operation(Operation::Mul)
            .add_task(0.5)
            .add_task(0.5)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.25);
        assert_eq!(result.sample_1d(2.0), 0.25);
        assert_eq!(result.sample_1d(3.0), 0.25);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.25);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.25);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.25);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.25);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.25);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.25);
    }

    #[test]
    fn aggregator_sub_tests() {
        let mut result = AggregatorBuilder::<f64>::new()
            .operation(Operation::Sub)
            .initial(1)
            .add_task(0.5)
            .add_task(0.5)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.0);
        assert_eq!(result.sample_1d(2.0), 0.0);
        assert_eq!(result.sample_1d(3.0), 0.0);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.0);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.0);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.0);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.0);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.0);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.0);

        let mut result = AggregatorBuilder::<f32>::new()
            .operation(Operation::Sub)
            .initial(1.0)
            .add_task(0.5)
            .add_task(0.25)
            .build();

        assert_eq!(result.sample_1d(1.0), 0.25);
        assert_eq!(result.sample_1d(2.0), 0.25);
        assert_eq!(result.sample_1d(3.0), 0.25);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.25);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.25);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.25);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.25);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.25);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.25);
    }
}
