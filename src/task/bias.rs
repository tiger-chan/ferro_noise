mod builder;

macro_rules! bias {
	($type: ty) => {
		#[derive(Clone, Debug)]
		pub struct Bias {
			pub(crate) bias: TaskSource,
			pub(crate) source: TaskSource,
			// The `min` field represents the minimum value of an exponent used
			pub(crate) min: $type,
			// The `max` field represents the maximum value of an exponent used
			pub(crate) max: $type,
		}
		
		impl Bias {
			fn eval<F: Fn(&mut dyn Task) -> $type>(&mut self, sampler: F) -> $type {
				let v = sampler(&mut self.source);
				let b = sampler(&mut self.bias);
				// ease in out with higher exponents will push the values further towards the extremes
				let p = (b * self.max) + self.min;
				math::ease_in_out(v, p)
			}
		}
		
		impl Task for Bias {
			fn sample_1d(&mut self, x: $type) -> $type {
				self.eval(|t| t.sample_1d(x))
			}
		
			fn sample_2d(&mut self, x: $type, y: $type) -> $type {
				self.eval(|t| t.sample_2d(x, y))
			}
		
			fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type {
				self.eval(|t| t.sample_3d(x, y, z))
			}
		}		
	};
}

pub mod f32 {
	use crate::math::f32 as math;
	use crate::task::f32::{TaskSource, Task};
	pub use super::builder::f32::BiasBuilder;
	bias!(f32);
}

pub mod f64 {
	use crate::math::f64 as math;
	use crate::task::f64::{TaskSource, Task};
	pub use super::builder::f64::BiasBuilder;
	bias!(f64);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_type_bias_tests() {
		{
			use crate::task::f64::Task;
			let mut result = f64::BiasBuilder::new().bias(1.0).source(0.5).build();

			assert_eq!(result.sample_1d(1.0), 0.5);
			assert_eq!(result.sample_1d(2.0), 0.5);
			assert_eq!(result.sample_1d(3.0), 0.5);

			assert_eq!(result.sample_2d(1.0, 1.0), 0.5);
			assert_eq!(result.sample_2d(2.0, 2.0), 0.5);
			assert_eq!(result.sample_2d(3.0, 3.0), 0.5);

			assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 0.5);
			assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 0.5);
			assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 0.5);
		}
		
		{
			use crate::task::f32::Task;
			let mut result = f32::BiasBuilder::new().bias(0.5).source(0.25).build();
	
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
}
