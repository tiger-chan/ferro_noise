use crate::{
    float::Float,
    math::{clamp, lerp},
};

use super::Task;

const MAX_GRADIENT_ENTRY: usize = 3;
const GRADIENT_X: usize = 0;
const GRADIENT_Y: usize = 1;
const GRADIENT_Z: usize = 2;

type Points<T> = [T; MAX_GRADIENT_ENTRY];

#[derive(Clone)]
pub struct Gradient<T: Float> {
    p1: [T; MAX_GRADIENT_ENTRY],
    p2: [T; MAX_GRADIENT_ENTRY],
    delta: [T; MAX_GRADIENT_ENTRY],
    lengths: [T; MAX_GRADIENT_ENTRY],
}

impl<T: Float> Gradient<T> {
    #[allow(dead_code)]
    pub fn new(p1: Points<T>, p2: Points<T>) -> Self {
        let mut gradient = Self {
            p1,
            p2,
            delta: [T::default(); MAX_GRADIENT_ENTRY],
            lengths: [T::default(); MAX_GRADIENT_ENTRY],
        };

        gradient.reset(p1, p2);
        gradient
    }

    pub fn reset(&mut self, p1: Points<T>, p2: Points<T>) {
        let mut delta = [T::default(); MAX_GRADIENT_ENTRY];
        let mut lengths = [T::default(); MAX_GRADIENT_ENTRY];
        let mut len = T::from(0);
        for (n, (x1, x2)) in p1.iter().zip(p2).enumerate() {
            delta[n] = x2 - *x1;
            len += delta[n] * delta[n];
            lengths[n] = len;
        }

        self.p1 = p1;
        self.p2 = p2;
        self.delta = delta;
        self.lengths = lengths;
    }
}

impl<T: Float> Default for Gradient<T> {
    fn default() -> Self {
        let mut gradient = Self {
            p1: [T::default(); MAX_GRADIENT_ENTRY],
            p2: [T::default(); MAX_GRADIENT_ENTRY],
            delta: [T::default(); MAX_GRADIENT_ENTRY],
            lengths: [T::default(); MAX_GRADIENT_ENTRY],
        };

        gradient.reset(
            [T::from(0), T::from(0), T::from(0)],
            [T::from(1), T::from(1), T::from(0)],
        );

        gradient
    }
}

impl<T: Float> Task<T> for Gradient<T> {
    /// Calculates the dot product of the x value scaled to the range [-1, 1].
    fn sample_1d(&mut self, x: T) -> T {
        let dx = x - self.p1[GRADIENT_X];
        let mut dot = dx * self.delta[GRADIENT_X];
        dot /= self.lengths[GRADIENT_X];
        let clampped = clamp(dot, T::from(0), T::from(1));
        lerp(T::from(-1.0), T::from(1), clampped)
    }

    /// Calculates the dot product of the x, y values scaled to the range [-1, 1].
    fn sample_2d(&mut self, x: T, y: T) -> T {
        let dx = x - self.p1[GRADIENT_X];
        let dy = y - self.p1[GRADIENT_Y];
        let mut dot = dx * self.delta[GRADIENT_X] + dy * self.delta[GRADIENT_Y];
        dot /= self.lengths[GRADIENT_Y];
        let clampped = clamp(dot, T::from(0), T::from(1));
        lerp(T::from(-1.0), T::from(1), clampped)
    }

    /// Calculates the dot product of the x, y, and z values scaled to the range [-1, 1].
    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        let dx = x - self.p1[GRADIENT_X];
        let dy = y - self.p1[GRADIENT_Y];
        let dz = z - self.p1[GRADIENT_Z];
        let mut dot =
            dx * self.delta[GRADIENT_X] + dy * self.delta[GRADIENT_Y] + dz * self.delta[GRADIENT_Z];
        dot /= self.lengths[GRADIENT_Z];
        let clampped = clamp(dot, T::from(0), T::from(1));
        lerp(T::from(-1.0), T::from(1), clampped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_type_gradient_tests() {
        let mut result = Gradient::<f64>::default();
        assert_eq!(result.sample_1d(1.0), 1.0);
        assert_eq!(result.sample_1d(-1.0), -1.0);
        assert!((result.sample_1d(0.6) - 0.2).abs() < f64::EPSILON);

        assert_eq!(result.sample_2d(0.75, 0.75), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 1.0);
        assert_eq!(result.sample_2d(-2.0, -2.0), -1.0);

        assert_eq!(result.sample_3d(0.75, 0.75, 0.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 0.0), 1.0);
        assert_eq!(result.sample_3d(-3.0, -3.0, 0.0), -1.0);

        let mut result = Gradient::<f32>::default();
        assert_eq!(result.sample_1d(1.0), 1.0);
        assert_eq!(result.sample_1d(-1.0), -1.0);
        assert!((result.sample_1d(0.6) - 0.2).abs() < f32::EPSILON);

        assert_eq!(result.sample_2d(0.75, 0.75), 0.5);
        assert_eq!(result.sample_2d(2.0, 2.0), 1.0);
        assert_eq!(result.sample_2d(-2.0, -2.0), -1.0);

        assert_eq!(result.sample_3d(0.75, 0.75, 0.0), 0.5);
        assert_eq!(result.sample_3d(2.0, 2.0, 0.0), 1.0);
        assert_eq!(result.sample_3d(-3.0, -3.0, 0.0), -1.0);
    }
}
