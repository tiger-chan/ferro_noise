mod builder;

use crate::{float::Float, math::lerp, source::Blender};

use super::{task::TaskSource, Task};

pub use builder::SelectorBuilder;

pub struct Selector<T: Float> {
    blender: Blender<T>,
    condition: TaskSource<T>,
    lower: TaskSource<T>,
    upper: TaskSource<T>,
    falloff: TaskSource<T>,
    /// threadhold/pivot/boundry to determine when lower or upper is used
    threshold: TaskSource<T>,
}

impl<T: Float> Selector<T> {
    fn eval<F>(&mut self, sampler: F) -> T
    where
        F: Fn(&mut TaskSource<T>) -> T,
    {
        let c = sampler(&mut self.condition);
        let f = sampler(&mut self.falloff);
        let t = sampler(&mut self.threshold);
        if f > T::ZERO {
            if c < t - f {
                // outside of the threshold on the lower side
                sampler(&mut self.lower)
            } else if c > t + f {
                // outside of the threshold on the upper side
                sampler(&mut self.upper)
            } else {
                // lower bound
                let l = t - f;
                // upper bound
                let u = t + f;
                let a = (c - l) / (u - l);
                let b = (self.blender)(a);
                let lower = sampler(&mut self.lower);
                let upper = sampler(&mut self.upper);
                lerp(lower, upper, b)
            }
        } else {
            if c < t {
                // outside of the threshold on the lower side
                sampler(&mut self.lower)
            } else {
                // outside of the threshold on the upper side
                sampler(&mut self.upper)
            }
        }
    }
}

impl<T: Float> Task<T> for Selector<T> {
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
    fn task_type_selector_tests() {
        use TaskSource::Constant;
        let mut result = SelectorBuilder::<f64>::new()
            .lower(Constant(0.0))
            .upper(Constant(1.0))
            .condition(Constant(1.0))
            .build();

        assert_eq!(result.sample_1d(0.0), 1.0);
        assert_eq!(result.sample_2d(0.0, 0.0), 1.0);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 1.0);

        let mut result = SelectorBuilder::<f64>::new()
            .lower(Constant(0.0))
            .upper(Constant(1.0))
            .condition(Constant(0.0))
            .build();
        assert_eq!(result.sample_1d(0.0), 0.0);
        assert_eq!(result.sample_2d(0.0, 0.0), 0.0);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.0);

        let mut result = SelectorBuilder::<f64>::new()
            .lower(Constant(0.0))
            .condition(Constant(0.5))
            .falloff(Constant(0.25))
            .build();

        assert_eq!(result.sample_1d(0.0), 0.5);
        assert_eq!(result.sample_2d(0.0, 0.0), 0.5);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.5);

        let mut result = SelectorBuilder::<f32>::new()
            .lower(Constant(0.0))
            .upper(Constant(1.0))
            .threadhold(Constant(0.5))
            .condition(Constant(1.0))
            .build();

        assert_eq!(result.sample_1d(0.0), 1.0);
        assert_eq!(result.sample_2d(0.0, 0.0), 1.0);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 1.0);

        let mut result = SelectorBuilder::<f32>::new()
            .lower(Constant(0.0))
            .upper(Constant(1.0))
            .threadhold(Constant(0.5))
            .condition(Constant(0.0))
            .build();

        assert_eq!(result.sample_1d(0.0), 0.0);
        assert_eq!(result.sample_2d(0.0, 0.0), 0.0);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.0);

        let mut result = SelectorBuilder::<f32>::new()
            .lower(Constant(0.0))
            .upper(Constant(1.0))
            .threadhold(Constant(0.5))
            .falloff(Constant(0.25))
            .condition(Constant(0.5))
            .build();

        assert_eq!(result.sample_1d(0.0), 0.5);
        assert_eq!(result.sample_2d(0.0, 0.0), 0.5);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.5);
    }
}
