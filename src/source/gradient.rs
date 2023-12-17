use super::vec3;
const MAX_GRADIENT_ENTRY: usize = vec3::MAX_COMPONENTS;

macro_rules! gradient {
    ($type: ty) => {
        /// A struct representing the gradient (i.e., the rate of change) between two points on a line segment.
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
        /// use ferro_noise::source::f64::{Noise, Gradient};
        ///
        /// // Define two points on a line segment.
        /// let s1 = [0.0, 0.0, 0.0];
        /// let s2 = [1.0, 1.0, 0.0];
        ///
        /// // Create a Gradient struct to calculate the gradient between the points.
        /// let mut grad = Gradient::new(s1, s2);
        ///
        /// // Calculate the gradient for a point on the line segment.
        /// let gradient = grad.sample_2d(0.5, 0.5);
        /// assert_eq!(gradient, 0.0);
        /// ```
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct Gradient {
            s1: Vec3,
            dir: Vec3,
            mag: $type,
        }

        impl Default for Gradient {
            fn default() -> Self {
                let s1 = [0.0; MAX_GRADIENT_ENTRY].into();
                let s2 = [1.0, 1.0, 0.0].into();

                let (dir, mag) = Gradient::pre_calc(&s1, &s2);
                Self {
                    s1,
                    dir,
                    mag,
                }
            }
        }

        impl Gradient {
            /// Creates a new Gradient struct with the given line segment endpoints.
            ///
            /// # Arguments
            ///
            /// * `s1`: The first point on the line segment.
            /// * `s2`: The second point on the line segment.
            pub fn new(s1: [$type; MAX_GRADIENT_ENTRY], s2: [$type; MAX_GRADIENT_ENTRY]) -> Self {
                let s1 = s1.into();
                let s2 = s2.into();
                let (dir, mag) = Gradient::pre_calc(&s1, &s2);
                Self {
                    s1,
                    dir,
                    mag,
                }
            }

            fn pre_calc(s1: &Vec3, s2: &Vec3) -> (Vec3, $type) {
                let direction = *s2 - *s1;
                let len = direction.dot(direction);

                if len <= 0.0 {
                    panic!("Gradient segment must have a greater length than 0.0");
                }
                // sqrt + a little margin to acount for floating point error
                let len = len.sqrt();

                let direction = direction / len;

                (direction, len)
            }

            fn eval(&mut self, p1: Vec3) -> $type {
                let dp = p1 - self.s1;
                let dot = dp.dot(self.dir);
                let proj_p = (dot / self.mag) * (1.0 + <$type>::EPSILON);
                let clampped = math::clamp(proj_p, 0.0, 1.0);
                math::lerp(-1.0, 1.0, clampped)
            }
        }

        impl Noise for Gradient {
            /// Calculates the dot product of the x value scaled to the range [-1, 1].
            fn sample_1d(&mut self, x: $type) -> $type {
                let delta = x - self.s1.x;
                self.eval(Vec3 {
                    x,
                    y: self.s1.y + delta,
                    z: self.s1.z + delta,
                })
            }

            /// Calculates the dot product of the x, y values scaled to the range [-1, 1].
            fn sample_2d(&mut self, x: $type, y: $type) -> $type {
                let p1 = Vec3 { x, y, z: self.s1.z };
                let diff = p1 - self.s1;
                let mag = diff.dot(diff);
                let mag = match mag {
                    _ if mag <= 0.0 => 0.0,
                    x => x.sqrt(),
                };
                self.eval(Vec3 {
                    x,
                    y,
                    z: self.s1.z + self.dir.z * mag,
                })
            }

            /// Calculates the dot product of the x, y, and z values scaled to the range [-1, 1].
            fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type {
                self.eval(Vec3 { x, y, z })
            }
        }
    };
}

pub mod f32 {
    use super::super::vec3::f32::Vec3;
    use super::MAX_GRADIENT_ENTRY;
    use crate::math::f32 as math;
    use crate::source::f32::Noise;

    gradient!(f32);
}

pub mod f64 {
    use super::super::vec3::f64::Vec3;
    use super::MAX_GRADIENT_ENTRY;
    use crate::math::f64 as math;
    use crate::source::f64::Noise;
    gradient!(f64);
}

#[cfg(test)]
mod tests {
    macro_rules! assert_nearly_eq {
        ($v1:expr, $v2:expr, $epsilon:expr) => {{
            let v1 = $v1;
            let v2 = $v2;
            let epsilon = $epsilon;
            assert!((v1 - v2).abs() < epsilon, "{} - {} < {}", v1, v2, epsilon);
        }};
    }

    mod f32 {
        use crate::source::f32::{Gradient, Noise};
        #[test]
        fn task_type_gradient_tests() {
            let mut result = Gradient::default();
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

    mod f64 {
        use crate::source::f64::{Gradient, Noise};
        #[test]
        fn task_type_gradient_tests() {
            let mut result = Gradient::default();
            assert_nearly_eq!(result.sample_1d(1.0), 1.0, f64::EPSILON);
            assert_nearly_eq!(result.sample_1d(-1.0), -1.0, f64::EPSILON);
            assert_nearly_eq!(result.sample_1d(0.6), 0.2, f64::EPSILON);

            assert_nearly_eq!(result.sample_2d(0.75, 0.75), 0.5, f64::EPSILON);
            assert_nearly_eq!(result.sample_2d(2.0, 2.0), 1.0, f64::EPSILON);
            assert_nearly_eq!(result.sample_2d(-2.0, -2.0), -1.0, f64::EPSILON);

            assert_nearly_eq!(result.sample_3d(0.75, 0.75, 0.0), 0.5, f64::EPSILON);
            assert_nearly_eq!(result.sample_3d(2.0, 2.0, 0.0), 1.0, f64::EPSILON);
            assert_nearly_eq!(result.sample_3d(-3.0, -3.0, 0.0), -1.0, f64::EPSILON);
        }
    }
}
