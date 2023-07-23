mod named_or_source;
pub(crate) use named_or_source::*;

macro_rules! task_source {
    ($type: ty) => {
        #[allow(dead_code)]
        #[derive(Clone, Debug)]
        pub enum TaskSource {
            Aggregate(Aggregator),
            Bias(Rc<RefCell<Bias>>),
            Cache(Rc<RefCell<Cache>>),
            Constant($type),
            Fractal(Rc<RefCell<Fractal>>),
            Gradient(Rc<RefCell<Gradient>>),
            Scale(Rc<RefCell<Scale>>),
            ScaleOffset(Rc<RefCell<ScaleOffset>>),
            Selector(Rc<RefCell<Selector>>),
            Domain(Rc<RefCell<TransformDomain>>),
        }

        impl From<Aggregator> for TaskSource {
            fn from(value: Aggregator) -> Self {
                Self::Aggregate(value)
            }
        }

        impl From<Bias> for TaskSource {
            fn from(value: Bias) -> Self {
                Self::Bias(Rc::new(RefCell::new(value)))
            }
        }

        impl From<Cache> for TaskSource {
            fn from(value: Cache) -> Self {
                Self::Cache(Rc::new(RefCell::new(value)))
            }
        }

        impl From<$type> for TaskSource {
            fn from(value: $type) -> Self {
                Self::Constant(value)
            }
        }

        impl From<Fractal> for TaskSource {
            fn from(value: Fractal) -> Self {
                Self::Fractal(Rc::new(RefCell::new(value)))
            }
        }

        impl From<Gradient> for TaskSource {
            fn from(value: Gradient) -> Self {
                Self::Gradient(Rc::new(RefCell::new(value)))
            }
        }

        impl From<Scale> for TaskSource {
            fn from(value: Scale) -> Self {
                Self::Scale(Rc::new(RefCell::new(value)))
            }
        }

        impl From<ScaleOffset> for TaskSource {
            fn from(value: ScaleOffset) -> Self {
                Self::ScaleOffset(Rc::new(RefCell::new(value)))
            }
        }

        impl From<Selector> for TaskSource {
            fn from(value: Selector) -> Self {
                Self::Selector(Rc::new(RefCell::new(value)))
            }
        }

        impl From<TransformDomain> for TaskSource {
            fn from(value: TransformDomain) -> Self {
                Self::Domain(Rc::new(RefCell::new(value)))
            }
        }

        impl Task for TaskSource {
            fn sample_1d(&mut self, x: $type) -> $type {
                match self {
                    Self::Aggregate(t) => t.sample_1d(x),
                    Self::Bias(t) => t.borrow_mut().sample_1d(x),
                    Self::Cache(t) => t.borrow_mut().sample_1d(x),
                    Self::Constant(v) => v.clone(),
                    Self::Fractal(t) => t.borrow_mut().sample_1d(x),
                    Self::Gradient(t) => t.borrow_mut().sample_1d(x),
                    Self::Scale(t) => t.borrow_mut().sample_1d(x),
                    Self::ScaleOffset(t) => t.borrow_mut().sample_1d(x),
                    Self::Selector(t) => t.borrow_mut().sample_1d(x),
                    Self::Domain(t) => t.borrow_mut().sample_1d(x),
                }
            }

            fn sample_2d(&mut self, x: $type, y: $type) -> $type {
                match self {
                    Self::Aggregate(t) => t.sample_2d(x, y),
                    Self::Bias(t) => t.borrow_mut().sample_2d(x, y),
                    Self::Cache(t) => t.borrow_mut().sample_2d(x, y),
                    Self::Constant(v) => v.clone(),
                    Self::Fractal(t) => t.borrow_mut().sample_2d(x, y),
                    Self::Gradient(t) => t.borrow_mut().sample_2d(x, y),
                    Self::Scale(t) => t.borrow_mut().sample_2d(x, y),
                    Self::ScaleOffset(t) => t.borrow_mut().sample_2d(x, y),
                    Self::Selector(t) => t.borrow_mut().sample_2d(x, y),
                    Self::Domain(t) => t.borrow_mut().sample_2d(x, y),
                }
            }

            fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type {
                match self {
                    Self::Aggregate(t) => t.sample_3d(x, y, z),
                    Self::Bias(t) => t.borrow_mut().sample_3d(x, y, z),
                    Self::Cache(t) => t.borrow_mut().sample_3d(x, y, z),
                    Self::Constant(v) => v.clone(),
                    Self::Fractal(t) => t.borrow_mut().sample_3d(x, y, z),
                    Self::Gradient(t) => t.borrow_mut().sample_3d(x, y, z),
                    Self::Scale(t) => t.borrow_mut().sample_3d(x, y, z),
                    Self::ScaleOffset(t) => t.borrow_mut().sample_3d(x, y, z),
                    Self::Selector(t) => t.borrow_mut().sample_3d(x, y, z),
                    Self::Domain(t) => t.borrow_mut().sample_3d(x, y, z),
                }
            }
        }
    };
}

pub mod f32 {
    pub(crate) use super::named_or_source::f32::NameOrSource;
    use crate::task::f32::{
        Aggregator, Bias, Cache, Fractal, Gradient, Scale, ScaleOffset, Selector, Task,
        TransformDomain,
    };
    use std::{cell::RefCell, rc::Rc};
    task_source!(f32);
}

pub mod f64 {
    pub(crate) use super::named_or_source::f64::NameOrSource;
    use crate::task::f64::{
        Aggregator, Bias, Cache, Fractal, Gradient, Scale, ScaleOffset, Selector, Task,
        TransformDomain,
    };
    use std::{cell::RefCell, rc::Rc};
    task_source!(f64);
}

#[cfg(test)]
mod tests {

    mod f32 {
        use crate::task::f32::{CacheBuilder, Task, TaskSource};

        #[test]
        fn task_type_constant_tests() {
            let mut result = TaskSource::Constant(0.5);
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
            let mut result = TaskSource::from(CacheBuilder::new().source(0.5).build());
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

    mod f64 {
        use crate::task::f64::{CacheBuilder, Task, TaskSource};

        #[test]
        fn task_type_constant_tests() {
            let mut result = TaskSource::Constant(0.5);
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
            let mut result = TaskSource::from(CacheBuilder::new().source(0.5).build());
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
}
