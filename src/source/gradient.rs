use std::ops::{Div, Sub};

use crate::{
    float::Float,
    math::{clamp, lerp},
};

use super::Noise;

const MAX_GRADIENT_ENTRY: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Point<T: Float> {
    x: T,
    y: T,
    z: T,
}

impl<T: Float> Default for Point<T> {
    fn default() -> Self {
        Self {
            x: T::ZERO,
            y: T::ZERO,
            z: T::ZERO,
        }
    }
}

impl<T: Float> Sub for Point<T> {
    type Output = Point<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Float> Div<T> for Point<T> {
    type Output = Point<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Float> From<[T; MAX_GRADIENT_ENTRY]> for Point<T> {
    fn from(value: [T; MAX_GRADIENT_ENTRY]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl<T: Float> Point<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

/// A struct representing the gradient (i.e., the rate of change) between two points on a line segment.
///
/// # Type Parameters
///
/// * `T`: A floating-point type that implements the `Float` (f32 or f64) trait.
///
/// # Fields
///
/// * `s1`: The first point on the line segment.
/// * `s2`: The second point on the line segment.
///
/// # Examples
///
/// ```
/// extern crate ferro_noise;
/// use ferro_noise::source::{Noise, Gradient};
///
/// // Define two points on a line segment.
/// let s1 = [0.0, 0.0, 0.0];
/// let s2 = [1.0, 1.0, 0.0];
///
/// // Create a Gradient struct to calculate the gradient between the points.
/// let mut grad = Gradient::<f64>::new(s1, s2);
///
/// // Calculate the gradient for a point on the line segment.
/// let gradient = grad.sample_2d(0.5, 0.5);
/// assert_eq!(gradient, 0.0);
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Gradient<T: Float> {
    s1: Point<T>,
    dir: Point<T>,
    mag: T,
}

impl<T: Float> Default for Gradient<T> {
    fn default() -> Self {
        let s1 = [T::ZERO; MAX_GRADIENT_ENTRY].into();
        let s2 = [T::ONE, T::ONE, T::ZERO].into();

        let (dir, mag) = Gradient::pre_calc(&s1, &s2);
        Self {
            s1: s1,
            dir: dir,
            mag: mag,
        }
    }
}

impl<T: Float> Gradient<T> {
    /// Creates a new Gradient struct with the given line segment endpoints.
    ///
    /// # Arguments
    ///
    /// * `s1`: The first point on the line segment.
    /// * `s2`: The second point on the line segment.
    pub fn new(s1: [T; MAX_GRADIENT_ENTRY], s2: [T; MAX_GRADIENT_ENTRY]) -> Self {
        let s1 = s1.into();
        let s2 = s2.into();
        let (dir, mag) = Gradient::pre_calc(&s1, &s2);
        Self {
            s1: s1,
            dir: dir,
            mag: mag,
        }
    }

    fn pre_calc(s1: &Point<T>, s2: &Point<T>) -> (Point<T>, T) {
        let direction = *s2 - *s1;
        let len = direction.dot(direction);

        if len <= T::ZERO {
            panic!("Gradient segment must have a greater length than 0.0");
        }
        // sqrt + a little margin to acount for floating point error
        let len = len.sqrt();

        let direction = direction / len;

        (direction, len)
    }

    fn eval(&mut self, p1: Point<T>) -> T {
        let dp = p1 - self.s1;
        let dot = dp.dot(self.dir);
        let proj_p = (dot / self.mag) * (T::ONE + T::EPSILON);
        let clampped = clamp(proj_p, T::ZERO, T::ONE);
        lerp(-T::ONE, T::ONE, clampped)
    }
}

impl<T: Float> Noise<T> for Gradient<T> {
    /// Calculates the dot product of the x value scaled to the range [-1, 1].
    fn sample_1d(&mut self, x: T) -> T {
        let delta = x - self.s1.x;
        self.eval(Point {
            x,
            y: self.s1.y + delta,
            z: self.s1.z + delta,
        })
    }

    /// Calculates the dot product of the x, y values scaled to the range [-1, 1].
    fn sample_2d(&mut self, x: T, y: T) -> T {
        let p1 = Point { x, y, z: self.s1.z };
        let diff = p1 - self.s1;
        let mag = diff.dot(diff);
        let mag = match mag {
            _ if mag <= T::ZERO => T::ZERO,
            x => x.sqrt(),
        };
        self.eval(Point {
            x,
            y,
            z: self.s1.z + self.dir.z * mag,
        })
    }

    /// Calculates the dot product of the x, y, and z values scaled to the range [-1, 1].
    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        self.eval(Point { x, y, z })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_nearly_eq {
        ($v1:expr, $v2:expr, $epsilon:expr) => {{
            let v1 = $v1;
            let v2 = $v2;
            let epsilon = $epsilon;
            assert!((v1 - v2).abs() < epsilon, "{} - {} < {}", v1, v2, epsilon);
        }};
    }

    #[test]
    fn task_type_gradient_tests() {
        let mut result = Gradient::<f64>::default();
        assert_nearly_eq!(result.sample_1d(1.0), 1.0, f64::EPSILON);
        assert_nearly_eq!(result.sample_1d(-1.0), -1.0, f64::EPSILON);
        assert_nearly_eq!(result.sample_1d(0.6), 0.2, f64::EPSILON);

        assert_nearly_eq!(result.sample_2d(0.75, 0.75), 0.5, f64::EPSILON);
        assert_nearly_eq!(result.sample_2d(2.0, 2.0), 1.0, f64::EPSILON);
        assert_nearly_eq!(result.sample_2d(-2.0, -2.0), -1.0, f64::EPSILON);

        assert_nearly_eq!(result.sample_3d(0.75, 0.75, 0.0), 0.5, f64::EPSILON);
        assert_nearly_eq!(result.sample_3d(2.0, 2.0, 0.0), 1.0, f64::EPSILON);
        assert_nearly_eq!(result.sample_3d(-3.0, -3.0, 0.0), -1.0, f64::EPSILON);

        let mut result = Gradient::<f32>::default();
        assert_nearly_eq!(result.sample_1d(1.0), 1.0, 0.001);
        assert_nearly_eq!(result.sample_1d(-1.0), -1.0, 0.001);
        assert_nearly_eq!(result.sample_1d(0.6), 0.2, 0.001);

        assert_nearly_eq!(result.sample_2d(0.75, 0.75), 0.5, 0.001);
        assert_nearly_eq!(result.sample_2d(2.0, 2.0), 1.0, 0.001);
        assert_nearly_eq!(result.sample_2d(-2.0, -2.0), -1.0, 0.001);

        assert_nearly_eq!(result.sample_3d(0.75, 0.75, 0.0), 0.5, 0.001);
        assert_nearly_eq!(result.sample_3d(2.0, 2.0, 0.0), 1.0, 0.001);
        assert_nearly_eq!(result.sample_3d(-3.0, -3.0, 0.0), -1.0, 0.001);
    }
}
