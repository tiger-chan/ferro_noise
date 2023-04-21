use std::{cell::RefCell, rc::Rc};

use crate::float::Float;

use super::{Aggregator, Bias, Cache, Fractal, Gradient, Task};

#[allow(dead_code)]
#[derive(Clone)]
pub enum TaskSource<T: Float> {
    Aggregate(Aggregator<T>),
    Constant(T),
    Bias(Rc<RefCell<Bias<T>>>),
    Cache(Rc<RefCell<Cache<T>>>),
    Gradient(Gradient<T>),
    Fractal(Rc<RefCell<Fractal<T>>>),
}

impl<T: Float> Task<T> for TaskSource<T> {
    fn sample_1d(&mut self, x: T) -> T {
        match self {
            Self::Aggregate(t) => t.sample_1d(x),
            Self::Bias(t) => t.borrow_mut().sample_1d(x),
            Self::Constant(v) => v.clone(),
            Self::Cache(t) => t.borrow_mut().sample_1d(x),
            Self::Gradient(t) => t.sample_1d(x),
            Self::Fractal(t) => t.borrow_mut().sample_1d(x),
        }
    }

    fn sample_2d(&mut self, x: T, y: T) -> T {
        match self {
            Self::Aggregate(t) => t.sample_2d(x, y),
            Self::Bias(t) => t.borrow_mut().sample_2d(x, y),
            Self::Constant(v) => v.clone(),
            Self::Cache(t) => t.borrow_mut().sample_2d(x, y),
            Self::Gradient(t) => t.sample_2d(x, y),
            Self::Fractal(t) => t.borrow_mut().sample_2d(x, y),
        }
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        match self {
            Self::Aggregate(t) => t.sample_3d(x, y, z),
            Self::Bias(t) => t.borrow_mut().sample_3d(x, y, z),
            Self::Constant(v) => v.clone(),
            Self::Cache(t) => t.borrow_mut().sample_3d(x, y, z),
            Self::Gradient(t) => t.sample_3d(x, y, z),
            Self::Fractal(t) => t.borrow_mut().sample_3d(x, y, z),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_type_constant_tests() {
        let mut result = TaskSource::<f64>::Constant(0.5);
        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = TaskSource::<f32>::Constant(0.5);
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
    fn task_type_cache_tests() {
        let mut result =
            TaskSource::<f64>::Cache(Rc::new(RefCell::new(Cache::new(TaskSource::Constant(0.5)))));
        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result =
            TaskSource::<f32>::Cache(Rc::new(RefCell::new(Cache::new(TaskSource::Constant(0.5)))));
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
}
