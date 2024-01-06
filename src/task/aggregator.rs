mod builder;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Operation {
    #[default]
    Add,
    Avg,
    Sub,
    Mul,
    Div,
    Max,
    Min,
}

macro_rules! aggregator {
    ($type: ty) => {
        #[derive(Clone, Debug)]
        pub struct Aggregator {
            pub(crate) op: Operation,
            pub(crate) initial: $type,
            pub(crate) sources: Vec<TaskSource>,
        }

        impl Default for Aggregator {
            fn default() -> Self {
                Self {
                    op: Operation::Add,
                    initial: 0.0,
                    sources: vec![],
                }
            }
        }

        impl Aggregator {
            pub fn new(op: Operation, initial: $type, sources: Vec<TaskSource>) -> Self {
                Self {
                    op,
                    initial,
                    sources,
                }
            }

            fn eval<F>(&mut self, sampler: F) -> $type
            where
                F: Fn(&mut TaskSource) -> $type,
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
                    result / self.sources.len() as $type
                } else {
                    result
                }
            }
        }

        impl Task for Aggregator {
            fn sample_1d(&mut self, x: $type) -> $type {
                self.eval(|s| (*s).sample_1d(x))
            }

            fn sample_2d(&mut self, x: $type, y: $type) -> $type {
                self.eval(|s| (*s).sample_2d(x, y))
            }

            fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type {
                self.eval(|s| (*s).sample_3d(x, y, z))
            }
        }
    };
}

pub mod f32 {
    pub use super::builder::f32::AggregatorBuilder;
    pub use super::Operation;
    use crate::task::f32::{Task, TaskSource};
    aggregator!(f32);
}

pub mod f64 {
    pub use super::builder::f64::AggregatorBuilder;
    pub use super::Operation;
    use crate::task::f64::{Task, TaskSource};
    aggregator!(f64);
}

#[cfg(test)]
mod tests {
    mod f32 {
        use crate::task::f32::{AggregatorBuilder, Operation, Task};
        #[test]
        fn aggregator_add_tests() {
            let mut result = AggregatorBuilder::default()
                .operation(Operation::Add)
                .initial(0.0)
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
                .operation(Operation::Avg)
                .initial(0.0)
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
                .operation(Operation::Div)
                .initial(2.0)
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
                .operation(Operation::Sub)
                .initial(1.0)
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

            let mut result = AggregatorBuilder::default()
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

    mod f64 {
        use crate::task::f64::{AggregatorBuilder, Operation, Task};
        #[test]
        fn aggregator_add_tests() {
            let mut result = AggregatorBuilder::default()
                .operation(Operation::Add)
                .initial(0.0)
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
                .operation(Operation::Avg)
                .initial(0.0)
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
                .operation(Operation::Div)
                .initial(2.0)
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
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

            let mut result = AggregatorBuilder::default()
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
            let mut result = AggregatorBuilder::default()
                .operation(Operation::Sub)
                .initial(1.0)
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

            let mut result = AggregatorBuilder::default()
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
}
