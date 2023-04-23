use std::{cell::RefCell, rc::Rc};

use crate::float::Float;

use super::{Aggregator, Bias, Cache, Fractal, Gradient, Selector, Task};

#[allow(dead_code)]
#[derive(Clone)]
pub enum TaskSource<T: Float> {
    Aggregate(Aggregator<T>),
    Bias(Rc<RefCell<Bias<T>>>),
    Cache(Rc<RefCell<Cache<T>>>),
    Constant(T),
    Fractal(Rc<RefCell<Fractal<T>>>),
    Gradient(Gradient<T>),
    Selector(Rc<RefCell<Selector<T>>>),
}

impl<T: Float> From<Aggregator<T>> for TaskSource<T> {
    fn from(value: Aggregator<T>) -> Self {
        Self::Aggregate(value)
    }
}

impl<T: Float> From<Bias<T>> for TaskSource<T> {
    fn from(value: Bias<T>) -> Self {
        Self::Bias(Rc::new(RefCell::new(value)))
    }
}

impl<T: Float> From<Cache<T>> for TaskSource<T> {
    fn from(value: Cache<T>) -> Self {
        Self::Cache(Rc::new(RefCell::new(value)))
    }
}

impl<T: Float> From<T> for TaskSource<T> {
    fn from(value: T) -> Self {
        Self::Constant(value)
    }
}

impl<T: Float> From<Fractal<T>> for TaskSource<T> {
    fn from(value: Fractal<T>) -> Self {
        Self::Fractal(Rc::new(RefCell::new(value)))
    }
}

impl<T: Float> From<Gradient<T>> for TaskSource<T> {
    fn from(value: Gradient<T>) -> Self {
        Self::Gradient(value)
    }
}

impl<T: Float> From<Selector<T>> for TaskSource<T> {
    fn from(value: Selector<T>) -> Self {
        Self::Selector(Rc::new(RefCell::new(value)))
    }
}

impl<T: Float> Task<T> for TaskSource<T> {
    fn sample_1d(&mut self, x: T) -> T {
        match self {
            Self::Aggregate(t) => t.sample_1d(x),
            Self::Bias(t) => t.borrow_mut().sample_1d(x),
            Self::Cache(t) => t.borrow_mut().sample_1d(x),
            Self::Constant(v) => v.clone(),
            Self::Fractal(t) => t.borrow_mut().sample_1d(x),
            Self::Gradient(t) => t.sample_1d(x),
            Self::Selector(t) => t.borrow_mut().sample_1d(x),
        }
    }

    fn sample_2d(&mut self, x: T, y: T) -> T {
        match self {
            Self::Aggregate(t) => t.sample_2d(x, y),
            Self::Bias(t) => t.borrow_mut().sample_2d(x, y),
            Self::Cache(t) => t.borrow_mut().sample_2d(x, y),
            Self::Constant(v) => v.clone(),
            Self::Fractal(t) => t.borrow_mut().sample_2d(x, y),
            Self::Gradient(t) => t.sample_2d(x, y),
            Self::Selector(t) => t.borrow_mut().sample_2d(x, y),
        }
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        match self {
            Self::Aggregate(t) => t.sample_3d(x, y, z),
            Self::Bias(t) => t.borrow_mut().sample_3d(x, y, z),
            Self::Cache(t) => t.borrow_mut().sample_3d(x, y, z),
            Self::Constant(v) => v.clone(),
            Self::Fractal(t) => t.borrow_mut().sample_3d(x, y, z),
            Self::Gradient(t) => t.sample_3d(x, y, z),
            Self::Selector(t) => t.borrow_mut().sample_3d(x, y, z),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::task::CacheBuilder;

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
        let mut result = TaskSource::<f64>::from(CacheBuilder::new().source(0.5).build());
        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = TaskSource::<f32>::from(CacheBuilder::new().source(0.5).build());
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
