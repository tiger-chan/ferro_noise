use crate::float::Float;

use super::{Bias, Cache, Gradient, Perlin, Task};

#[allow(dead_code)]
pub enum TaskType<T: Float> {
    Constant(T),
    Bias(Box<Bias<T>>),
    Cache(Box<Cache<T>>),
    Gradient(Gradient<T>),
    Perlin(Box<Perlin<T>>),
}

impl<T: Float> Task<T> for TaskType<T> {
    fn sample_1d(&mut self, x: T) -> T {
        match self {
            Self::Bias(t) => t.sample_1d(x),
            Self::Constant(v) => v.clone(),
            Self::Cache(t) => t.sample_1d(x),
            Self::Gradient(t) => t.sample_1d(x),
            Self::Perlin(t) => t.sample_1d(x),
        }
    }

    fn sample_2d(&mut self, x: T, y: T) -> T {
        match self {
            Self::Bias(t) => t.sample_2d(x, y),
            Self::Constant(v) => v.clone(),
            Self::Cache(t) => t.sample_2d(x, y),
            Self::Gradient(t) => t.sample_2d(x, y),
            Self::Perlin(t) => t.sample_2d(x, y),
        }
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        match self {
            Self::Bias(t) => t.sample_3d(x, y, z),
            Self::Constant(v) => v.clone(),
            Self::Cache(t) => t.sample_3d(x, y, z),
            Self::Gradient(t) => t.sample_3d(x, y, z),
            Self::Perlin(t) => t.sample_3d(x, y, z),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_type_constant_tests() {
        let mut result = TaskType::Constant(0.5);
        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = TaskType::Constant(0.5_f32);
        assert_eq!(result.sample_1d(1.0_f32), 0.5_f32);
        assert_eq!(result.sample_1d(2.0_f32), 0.5_f32);
        assert_eq!(result.sample_1d(3.0_f32), 0.5_f32);

        assert_eq!(result.sample_2d(1.0_f32, 1.0_f32), 0.5_f32);
        assert_eq!(result.sample_2d(2.0_f32, 2.0_f32), 0.5_f32);
        assert_eq!(result.sample_2d(3.0_f32, 3.0_f32), 0.5_f32);

        assert_eq!(result.sample_3d(1.0_f32, 1.0_f32, 1.0_f32), 0.5_f32);
        assert_eq!(result.sample_3d(2.0_f32, 2.0_f32, 2.0_f32), 0.5_f32);
        assert_eq!(result.sample_3d(3.0_f32, 3.0_f32, 3.0_f32), 0.5_f32);
    }

    #[test]
    fn task_type_cache_tests() {
        let mut result = TaskType::Cache(Box::new(Cache::new(TaskType::Constant(0.5))));
        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = TaskType::Cache(Box::new(Cache::new(TaskType::Constant(0.5_f32))));
        assert_eq!(result.sample_1d(1.0_f32), 0.5_f32);
        assert_eq!(result.sample_1d(2.0_f32), 0.5_f32);
        assert_eq!(result.sample_1d(3.0_f32), 0.5_f32);

        assert_eq!(result.sample_2d(1.0_f32, 1.0_f32), 0.5_f32);
        assert_eq!(result.sample_2d(2.0_f32, 2.0_f32), 0.5_f32);
        assert_eq!(result.sample_2d(3.0_f32, 3.0_f32), 0.5_f32);

        assert_eq!(result.sample_3d(1.0_f32, 1.0_f32, 1.0_f32), 0.5_f32);
        assert_eq!(result.sample_3d(2.0_f32, 2.0_f32, 2.0_f32), 0.5_f32);
        assert_eq!(result.sample_3d(3.0_f32, 3.0_f32, 3.0_f32), 0.5_f32);
    }
}
