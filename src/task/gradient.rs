mod builder;

use crate::{
    float::Float,
    source::{self, Noise},
};

use super::Task;

pub use builder::GradientBuilder;

#[derive(Clone)]
pub struct Gradient<T: Float> {
    noise: source::Gradient<T>,
}

impl<T: Float> Default for Gradient<T> {
    fn default() -> Self {
        Self {
            noise: source::Gradient::default(),
        }
    }
}

impl<T: Float> Task<T> for Gradient<T> {
    /// Calculates the dot product of the x value scaled to the range [-1, 1].
    fn sample_1d(&mut self, x: T) -> T {
        self.noise.sample_1d(x)
    }

    /// Calculates the dot product of the x, y values scaled to the range [-1, 1].
    fn sample_2d(&mut self, x: T, y: T) -> T {
        self.noise.sample_2d(x, y)
    }

    /// Calculates the dot product of the x, y, and z values scaled to the range [-1, 1].
    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        self.noise.sample_3d(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_type_gradient_tests() {
        let mut result = GradientBuilder::<f64>::new().build();
        assert_eq!(result.sample_1d(1.0), 1.0);

        let mut result = GradientBuilder::<f32>::default()
            .s2([1.0, 0.0, 0.0])
            .build();
        assert_eq!(result.sample_1d(1.0), 1.0);
    }
}
