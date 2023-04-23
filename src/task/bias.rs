mod builder;

pub use builder::*;

use crate::{float::Float, math::ease_in_out};

use super::{task::TaskSource, Task};

pub struct Bias<T: Float> {
    bias: TaskSource<T>,
    source: TaskSource<T>,
    // The `min` field represents the minimum value of an exponent used
    min: T,
    // The `max` field represents the maximum value of an exponent used
    max: T,
}

impl<T: Float> Bias<T> {
    fn eval<F: Fn(&mut dyn Task<T>) -> T>(&mut self, sampler: F) -> T {
        let v = sampler(&mut self.source);
        let b = sampler(&mut self.bias);
        // ease in out with higher exponents will push the values further towards the extremes
        let p = (b * self.max) + self.min;
        ease_in_out(v, p)
    }
}

impl<T: Float> Task<T> for Bias<T> {
    fn sample_1d(&mut self, x: T) -> T {
        self.eval(|t| t.sample_1d(x))
    }

    fn sample_2d(&mut self, x: T, y: T) -> T {
        self.eval(|t| t.sample_2d(x, y))
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        self.eval(|t| t.sample_3d(x, y, z))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_type_bias_tests() {
        let mut result = BiasBuilder::new().bias(1.0).source(0.5).build();

        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = BiasBuilder::<f32>::new().bias(0.5).source(0.25).build();

        assert_eq!(result.sample_1d(1.0_f32), 0.0625_f32);
        assert_eq!(result.sample_1d(2.0_f32), 0.0625_f32);
        assert_eq!(result.sample_1d(3.0_f32), 0.0625_f32);

        assert_eq!(result.sample_2d(1.0_f32, 1.0_f32), 0.0625_f32);
        assert_eq!(result.sample_2d(2.0_f32, 2.0_f32), 0.0625_f32);
        assert_eq!(result.sample_2d(3.0_f32, 3.0_f32), 0.0625_f32);

        assert_eq!(result.sample_3d(1.0_f32, 1.0_f32, 1.0_f32), 0.0625_f32);
        assert_eq!(result.sample_3d(2.0_f32, 2.0_f32, 2.0_f32), 0.0625_f32);
        assert_eq!(result.sample_3d(3.0_f32, 3.0_f32, 3.0_f32), 0.0625_f32);
    }
}
