use crate::float::Float;

use super::{task::TaskSource, Task};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum AggregationMethod {
    Add,
    Avg,
    Sub,
    Mul,
    Div,
    Max,
    Min,
}

pub struct Aggregator<T: Float> {
    method: AggregationMethod,
    initial: T,
    sources: Vec<TaskSource<T>>,
}

impl<T: Float> Default for Aggregator<T> {
    fn default() -> Self {
        Self {
            method: AggregationMethod::Add,
            initial: T::default(),
            sources: vec![],
        }
    }
}

impl<T: Float> Aggregator<T> {
    #[allow(dead_code)]
    pub fn new(method: AggregationMethod, initial: T, sources: Vec<TaskSource<T>>) -> Self {
        Self {
            method,
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
            result = match self.method {
                AggregationMethod::Add => result + val,
                AggregationMethod::Avg => result + val,
                AggregationMethod::Div => result / val,
                AggregationMethod::Max => {
                    if result < val {
                        val
                    } else {
                        result
                    }
                }
                AggregationMethod::Min => {
                    if result < val {
                        result
                    } else {
                        val
                    }
                }
                AggregationMethod::Mul => result * val,
                AggregationMethod::Sub => result - val,
            };
        }
        if self.method == AggregationMethod::Avg && self.sources.len() > 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregator_add_tests() {
        let mut result = Aggregator::<f64>::new(
            AggregationMethod::Add,
            0.0,
            vec![TaskSource::Constant(0.5), TaskSource::Constant(0.5)],
        );

        assert_eq!(result.sample_1d(1.0), 1.0);
        assert_eq!(result.sample_1d(2.0), 1.0);
        assert_eq!(result.sample_1d(3.0), 1.0);

        assert_eq!(result.sample_2d(1.0, 1.0), 1.0);
        assert_eq!(result.sample_2d(2.0, 2.0), 1.0);
        assert_eq!(result.sample_2d(3.0, 3.0), 1.0);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 1.0);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 1.0);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 1.0);

        let mut result = Aggregator::<f32>::new(
            AggregationMethod::Add,
            0.0,
            vec![TaskSource::Constant(0.5), TaskSource::Constant(0.25)],
        );
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
        let mut result = Aggregator::<f64>::new(
            AggregationMethod::Avg,
            0.0,
            vec![TaskSource::Constant(0.5), TaskSource::Constant(0.5)],
        );

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = Aggregator::<f32>::new(
            AggregationMethod::Avg,
            0.0,
            vec![TaskSource::Constant(0.5), TaskSource::Constant(0.5)],
        );
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
        let mut result = Aggregator::<f64>::new(
            AggregationMethod::Div,
            2.0,
            vec![TaskSource::Constant(1.0), TaskSource::Constant(4.0)],
        );

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = Aggregator::<f32>::new(
            AggregationMethod::Div,
            2.0,
            vec![TaskSource::Constant(4.0), TaskSource::Constant(1.0)],
        );
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
        let mut result = Aggregator::<f64>::new(
            AggregationMethod::Max,
            0.0,
            vec![TaskSource::Constant(0.2), TaskSource::Constant(0.5)],
        );

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = Aggregator::<f32>::new(
            AggregationMethod::Max,
            -2.0,
            vec![TaskSource::Constant(-0.05), TaskSource::Constant(0.5)],
        );
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
        let mut result = Aggregator::<f64>::new(
            AggregationMethod::Min,
            10.0,
            vec![TaskSource::Constant(0.5), TaskSource::Constant(3.0)],
        );

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = Aggregator::<f32>::new(
            AggregationMethod::Min,
            2.0,
            vec![TaskSource::Constant(5.0), TaskSource::Constant(0.5)],
        );
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
        let mut result = Aggregator::<f64>::new(
            AggregationMethod::Mul,
            1.0,
            vec![TaskSource::Constant(0.5), TaskSource::Constant(0.5)],
        );

        assert_eq!(result.sample_1d(1.0), 0.25);
        assert_eq!(result.sample_1d(2.0), 0.25);
        assert_eq!(result.sample_1d(3.0), 0.25);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.25);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.25);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.25);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.25);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.25);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.25);

        let mut result = Aggregator::<f32>::new(
            AggregationMethod::Mul,
            1.0,
            vec![TaskSource::Constant(0.5), TaskSource::Constant(0.5)],
        );
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
        let mut result = Aggregator::<f64>::new(
            AggregationMethod::Sub,
            1.0,
            vec![TaskSource::Constant(0.5), TaskSource::Constant(0.5)],
        );

        assert_eq!(result.sample_1d(1.0), 0.0);
        assert_eq!(result.sample_1d(2.0), 0.0);
        assert_eq!(result.sample_1d(3.0), 0.0);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.0);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.0);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.0);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.0);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.0);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.0);

        let mut result = Aggregator::<f32>::new(
            AggregationMethod::Sub,
            1.0,
            vec![TaskSource::Constant(0.5), TaskSource::Constant(0.25)],
        );
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
