mod builder;

macro_rules! gradient_type {
    ($type: ty) => {
        #[derive(Debug, PartialEq, PartialOrd)]
        pub struct Gradient {
            pub(crate) noise: source::Gradient,
        }

        impl Default for Gradient {
            fn default() -> Self {
                Self {
                    noise: source::Gradient::default(),
                }
            }
        }

        impl Task for Gradient {
            /// Calculates the dot product of the x value scaled to the range [-1, 1].
            fn sample_1d(&mut self, x: $type) -> $type {
                self.noise.sample_1d(x)
            }

            /// Calculates the dot product of the x, y values scaled to the range [-1, 1].
            fn sample_2d(&mut self, x: $type, y: $type) -> $type {
                self.noise.sample_2d(x, y)
            }

            /// Calculates the dot product of the x, y, and z values scaled to the range [-1, 1].
            fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type {
                self.noise.sample_3d(x, y, z)
            }
        }
    };
}

pub mod f32 {
    pub use super::builder::f32::GradientBuilder;
    use crate::{source::f32 as source, task::f32::Task};
	use crate::source::f32::Noise;

    gradient_type!(f32);
}

pub mod f64 {
    pub use super::builder::f64::GradientBuilder;
    use crate::{source::f64 as source, task::f64::Task};
	use crate::source::f64::Noise;

    gradient_type!(f64);
}

#[cfg(test)]
mod tests {
	mod f32 {
		use crate::task::f32::{Task, GradientBuilder};
		#[test]
		fn task_type_gradient_tests() {
			let mut result = GradientBuilder::new().build();
			assert_eq!(result.sample_1d(1.0), 1.0);
	
			let mut result = GradientBuilder::default().s2([1.0, 0.0, 0.0]).build();
			assert_eq!(result.sample_1d(1.0), 1.0);
		}
	}

	mod f64 {
		use crate::task::f64::{Task, GradientBuilder};
		#[test]
		fn task_type_gradient_tests() {
			let mut result = GradientBuilder::new().build();
			assert_eq!(result.sample_1d(1.0), 1.0);
	
			let mut result = GradientBuilder::default().s2([1.0, 0.0, 0.0]).build();
			assert_eq!(result.sample_1d(1.0), 1.0);
		}
	}
}
