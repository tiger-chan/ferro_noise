mod builder;

macro_rules! selector_type {
    ($type: ty) => {
        #[derive(Clone, Debug)]
        pub struct Selector {
            pub(crate) blender: Blender,
            pub(crate) condition: TaskSource,
            pub(crate) lower: TaskSource,
            pub(crate) upper: TaskSource,
            pub(crate) falloff: TaskSource,
            /// threshold/pivot/boundry to determine when lower or upper is used
            pub(crate) threshold: TaskSource,
        }

        impl Selector {
            fn eval<F>(&mut self, sampler: F) -> $type
            where
                F: Fn(&mut TaskSource) -> $type,
            {
                let c = sampler(&mut self.condition);
                let f = sampler(&mut self.falloff);
                let t = sampler(&mut self.threshold);
                if f > 0.0 {
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

        impl Task for Selector {
            fn sample_1d(&mut self, x: $type) -> $type {
                self.eval(|s| (*s).sample_1d(x))
            }

            fn sample_2d(&mut self, x: $type, y: $type) -> $type {
                self.eval(|s| (*s).sample_2d(x, y))
            }

            fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type {
                self.eval(|s| (*s).sample_3d(x, y, z))
            }
        }
    };
}

pub mod f32 {
    use super::super::f32::*;
    pub use super::builder::f32::SelectorBuilder;
    use crate::math::f32::lerp;
    use crate::source::f32::Blender;
    selector_type!(f32);
}
pub mod f64 {
    use super::super::f64::*;
    pub use super::builder::f64::SelectorBuilder;
    use crate::math::f64::lerp;
    use crate::source::f64::Blender;
    selector_type!(f64);
}

#[cfg(test)]
mod tests {
	mod f32 {
		use crate::task::f32::{Task, SelectorBuilder};
		#[test]
		fn task_type_selector_tests() {
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.upper(1.0)
				.condition(1.0)
				.build();
	
			assert_eq!(result.sample_1d(0.0), 1.0);
			assert_eq!(result.sample_2d(0.0, 0.0), 1.0);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 1.0);
	
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.upper(1.0)
				.condition(0.0)
				.build();
			assert_eq!(result.sample_1d(0.0), 0.0);
			assert_eq!(result.sample_2d(0.0, 0.0), 0.0);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.0);
	
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.condition(0.5)
				.falloff(0.25)
				.build();
	
			assert_eq!(result.sample_1d(0.0), 0.5);
			assert_eq!(result.sample_2d(0.0, 0.0), 0.5);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.5);
	
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.upper(1.0)
				.threshold(0.5)
				.condition(1.0)
				.build();
	
			assert_eq!(result.sample_1d(0.0), 1.0);
			assert_eq!(result.sample_2d(0.0, 0.0), 1.0);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 1.0);
	
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.upper(1.0)
				.threshold(0.5)
				.condition(0.0)
				.build();
	
			assert_eq!(result.sample_1d(0.0), 0.0);
			assert_eq!(result.sample_2d(0.0, 0.0), 0.0);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.0);
	
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.upper(1.0)
				.threshold(0.5)
				.falloff(0.25)
				.condition(0.5)
				.build();
	
			assert_eq!(result.sample_1d(0.0), 0.5);
			assert_eq!(result.sample_2d(0.0, 0.0), 0.5);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.5);
		}
	}
	
	mod f64 {
		use crate::task::f64::{Task, SelectorBuilder};
		#[test]
		fn task_type_selector_tests() {
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.upper(1.0)
				.condition(1.0)
				.build();
	
			assert_eq!(result.sample_1d(0.0), 1.0);
			assert_eq!(result.sample_2d(0.0, 0.0), 1.0);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 1.0);
	
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.upper(1.0)
				.condition(0.0)
				.build();
			assert_eq!(result.sample_1d(0.0), 0.0);
			assert_eq!(result.sample_2d(0.0, 0.0), 0.0);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.0);
	
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.condition(0.5)
				.falloff(0.25)
				.build();
	
			assert_eq!(result.sample_1d(0.0), 0.5);
			assert_eq!(result.sample_2d(0.0, 0.0), 0.5);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.5);
	
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.upper(1.0)
				.threshold(0.5)
				.condition(1.0)
				.build();
	
			assert_eq!(result.sample_1d(0.0), 1.0);
			assert_eq!(result.sample_2d(0.0, 0.0), 1.0);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 1.0);
	
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.upper(1.0)
				.threshold(0.5)
				.condition(0.0)
				.build();
	
			assert_eq!(result.sample_1d(0.0), 0.0);
			assert_eq!(result.sample_2d(0.0, 0.0), 0.0);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.0);
	
			let mut result = SelectorBuilder::new()
				.lower(0.0)
				.upper(1.0)
				.threshold(0.5)
				.falloff(0.25)
				.condition(0.5)
				.build();
	
			assert_eq!(result.sample_1d(0.0), 0.5);
			assert_eq!(result.sample_2d(0.0, 0.0), 0.5);
			assert_eq!(result.sample_3d(0.0, 0.0, 0.0), 0.5);
		}
	}

}
