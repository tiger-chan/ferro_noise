use crate::{float::Float, math::lerp, source::Blender};

use super::{task::TaskSource, Task};

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
    #[allow(dead_code)]
    pub fn new(
        condition: TaskSource<T>,
        lower: TaskSource<T>,
        upper: TaskSource<T>,
        blender: Blender<T>,
        falloff: TaskSource<T>,
        threshold: TaskSource<T>,
    ) -> Self {
        Self {
            blender,
            condition,
            lower,
            upper,
            falloff,
            threshold,
        }
    }

    #[allow(dead_code)]
    pub fn set_blender(&mut self, blender: Blender<T>) -> &mut Self {
        self.blender = blender;
        self
    }

    #[allow(dead_code)]
    pub fn set_lower(&mut self, lower: TaskSource<T>) -> &mut Self {
        self.lower = lower;
        self
    }

    #[allow(dead_code)]
    pub fn set_upper(&mut self, upper: TaskSource<T>) -> &mut Self {
        self.upper = upper;
        self
    }

    #[allow(dead_code)]
    pub fn set_condition(&mut self, condition: TaskSource<T>) -> &mut Self {
        self.condition = condition;
        self
    }

    #[allow(dead_code)]
    pub fn set_falloff(&mut self, falloff: TaskSource<T>) -> &mut Self {
        self.falloff = falloff;
        self
    }

    #[allow(dead_code)]
    pub fn set_threshold(&mut self, threshold: TaskSource<T>) -> &mut Self {
        self.threshold = threshold;
        self
    }

    fn eval<F>(&mut self, sampler: F) -> T
    where
        F: Fn(&mut TaskSource<T>) -> T,
    {
        let c = sampler(&mut self.condition);
        let f = sampler(&mut self.falloff);
        let t = sampler(&mut self.threshold);
        if f > T::from(0) {
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

impl<T: Float> Default for Selector<T> {
    fn default() -> Self {
        use super::super::math::cubic_curve;
        Self {
            blender: cubic_curve,
            condition: TaskSource::Constant(T::default()),
            lower: TaskSource::Constant(T::default()),
            upper: TaskSource::Constant(T::default()),
            falloff: TaskSource::Constant(T::default()),
            threshold: TaskSource::Constant(T::default()),
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
        let mut result = Selector::<f64>::default();
        result
            .set_upper(TaskSource::Constant(1.0))
            .set_lower(TaskSource::Constant(0.0))
            .set_threshold(TaskSource::Constant(0.5));

        result.set_condition(TaskSource::Constant(1.0));
        assert_eq!(result.sample_1d(0.0), 1.0);
        assert_eq!(result.sample_2d(0.0, 0.0), 1.0);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 1.0);

        result.set_condition(TaskSource::Constant(0.0));
        assert_eq!(result.sample_1d(0.0), 0.0);
        assert_eq!(result.sample_2d(0.0, 0.0), 0.0);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.0);

        result
            .set_condition(TaskSource::Constant(0.5))
            .set_falloff(TaskSource::Constant(0.25));
        assert_eq!(result.sample_1d(0.0), 0.5);
        assert_eq!(result.sample_2d(0.0, 0.0), 0.5);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.5);

        let mut result = Selector::<f32>::default();
        result
            .set_upper(TaskSource::Constant(1.0))
            .set_lower(TaskSource::Constant(0.0))
            .set_threshold(TaskSource::Constant(0.5));

        result.set_condition(TaskSource::Constant(1.0));
        assert_eq!(result.sample_1d(0.0), 1.0);
        assert_eq!(result.sample_2d(0.0, 0.0), 1.0);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 1.0);

        result.set_condition(TaskSource::Constant(0.0));
        assert_eq!(result.sample_1d(0.0), 0.0);
        assert_eq!(result.sample_2d(0.0, 0.0), 0.0);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.0);

        result
            .set_condition(TaskSource::Constant(0.5))
            .set_falloff(TaskSource::Constant(0.25));
        assert_eq!(result.sample_1d(0.0), 0.5);
        assert_eq!(result.sample_2d(0.0, 0.0), 0.5);
        assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.5);
    }
}
