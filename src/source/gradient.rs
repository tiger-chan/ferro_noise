const MAX_GRADIENT_ENTRY: usize = 3;

macro_rules! point {
	($type: ty) => {
		use std::ops::{Div, Sub};

		#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
		struct Point {
			x: $type,
			y: $type,
			z: $type,
		}
		
		impl Default for Point {
			fn default() -> Self {
				Self {
					x: 0.0,
					y: 0.0,
					z: 0.0,
				}
			}
		}
		
		impl Sub for Point {
			type Output = Point;
			fn sub(self, rhs: Self) -> Self::Output {
				Self {
					x: self.x - rhs.x,
					y: self.y - rhs.y,
					z: self.z - rhs.z,
				}
			}
		}
		
		impl Div<$type> for Point {
			type Output = Point;
			fn div(self, rhs: $type) -> Self::Output {
				Self {
					x: self.x / rhs,
					y: self.y / rhs,
					z: self.z / rhs,
				}
			}
		}
		
		impl From<[$type; MAX_GRADIENT_ENTRY]> for Point {
			fn from(value: [$type; MAX_GRADIENT_ENTRY]) -> Self {
				Self {
					x: value[0],
					y: value[1],
					z: value[2],
				}
			}
		}
		
		impl Point {
			pub fn dot(self, rhs: Self) -> $type {
				self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
			}
		}
	};
}

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
			s1: Point,
			dir: Point,
			mag: $type,
		}
		
		impl Default for Gradient {
			fn default() -> Self {
				let s1 = [0.0; MAX_GRADIENT_ENTRY].into();
				let s2 = [1.0, 1.0, 0.0].into();
		
				let (dir, mag) = Gradient::pre_calc(&s1, &s2);
				Self {
					s1: s1,
					dir: dir,
					mag: mag,
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
					s1: s1,
					dir: dir,
					mag: mag,
				}
			}
		
			fn pre_calc(s1: &Point, s2: &Point) -> (Point, $type) {
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
		
			fn eval(&mut self, p1: Point) -> $type {
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
				self.eval(Point {
					x,
					y: self.s1.y + delta,
					z: self.s1.z + delta,
				})
			}
		
			/// Calculates the dot product of the x, y values scaled to the range [-1, 1].
			fn sample_2d(&mut self, x: $type, y: $type) -> $type {
				let p1 = Point { x, y, z: self.s1.z };
				let diff = p1 - self.s1;
				let mag = diff.dot(diff);
				let mag = match mag {
					_ if mag <= 0.0 => 0.0,
					x => x.sqrt(),
				};
				self.eval(Point {
					x,
					y,
					z: self.s1.z + self.dir.z * mag,
				})
			}
		
			/// Calculates the dot product of the x, y, and z values scaled to the range [-1, 1].
			fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type {
				self.eval(Point { x, y, z })
			}
		}
	};
}

pub mod f32 {
	use super::MAX_GRADIENT_ENTRY;
	use crate::source::f32::Noise;
	use crate::math::f32 as math;
	point!(f32);
	gradient!(f32);
}

pub mod f64 {
	use super::MAX_GRADIENT_ENTRY;
	use crate::source::f64::Noise;
	use crate::math::f64 as math;
	point!(f64);
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
