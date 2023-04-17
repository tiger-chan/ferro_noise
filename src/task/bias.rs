use crate::{float::Float, math::ease_in_out};

use super::{task::TaskType, Task};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct BiasConfig<T> {
    // The `min` field represents the minimum value of an exponent used
    pub min: T,
    // The `max` field represents the maximum value of an exponent used
    pub max: T,
}

pub struct Bias<T: Float> {
    bias: TaskType<T>,
    source: TaskType<T>,
    config: BiasConfig<T>,
}

impl<T: Float> Bias<T> {
    #[allow(dead_code)]
    pub fn new(source: TaskType<T>, bias: TaskType<T>) -> Self {
        Self {
            bias,
            source,
            config: BiasConfig {
                min: T::from(1),
                max: T::from(4),
            },
        }
    }
}

impl<T: Float> Task<T> for Bias<T> {
    fn sample_1d(&mut self, x: T) -> T {
        let v = self.source.sample_1d(x);
        let b = self.bias.sample_1d(x);
        // ease in out with higher exponents will push the values further towards the extremes
        let p = (b * self.config.max) + self.config.min;
        return ease_in_out(v, p);
    }

    fn sample_2d(&mut self, x: T, y: T) -> T {
        let v = self.source.sample_2d(x, y);
        let b = self.bias.sample_2d(x, y);
        // ease in out with higher exponents will push the values further towards the extremes
        let p = (b * self.config.max) + self.config.min;
        return ease_in_out(v, p);
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        let v = self.source.sample_3d(x, y, z);
        let b = self.bias.sample_3d(x, y, z);
        // ease in out with higher exponents will push the values further towards the extremes
        let p = (b * self.config.max) + self.config.min;
        return ease_in_out(v, p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_type_bias_tests() {
        let mut result = Bias::new(TaskType::Constant(0.5), TaskType::Constant(1.0));
        assert_eq!(result.sample_1d(1.0), 0.5);
        assert_eq!(result.sample_1d(2.0), 0.5);
        assert_eq!(result.sample_1d(3.0), 0.5);

        assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
        assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

        assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
        assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);

        let mut result = Bias::new(TaskType::Constant(0.25_f32), TaskType::Constant(0.5_f32));
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
