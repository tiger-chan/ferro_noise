mod builder;

macro_rules! scale {
    ($type: ty) => {
        #[derive(Clone, Debug)]
        pub struct Scale {
            pub(crate) scale: TaskSource,
            pub(crate) source: TaskSource,
        }

        impl Scale {
            fn eval<F: Fn(&mut dyn Task) -> $type>(&mut self, sampler: F) -> $type {
                let v = sampler(&mut self.source);
                let s = sampler(&mut self.scale);
                v * s
            }
        }

        impl Task for Scale {
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
    pub use super::builder::f32::ScaleBuilder;
    use crate::task::f32::{Task, TaskSource};
    scale!(f32);
}

pub mod f64 {
    pub use super::builder::f64::ScaleBuilder;
    use crate::task::f64::{Task, TaskSource};
    scale!(f64);
}

#[cfg(test)]
mod tests {
    mod f32 {
        use crate::task::f32::{ScaleBuilder, Task};
        #[test]
        fn scale_half() {
            let mut result = ScaleBuilder::new().scale(0.5).source(1.0).build();

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

        #[test]
        fn scale_double() {
            let mut result = ScaleBuilder::new().scale(2.0).source(1.0).build();

            assert_eq!(result.sample_1d(1.0), 2.0);
            assert_eq!(result.sample_1d(2.0), 2.0);
            assert_eq!(result.sample_1d(3.0), 2.0);

            assert_eq!(result.sample_2d(1.0, 1.0), 2.0);
            assert_eq!(result.sample_2d(2.0, 2.0), 2.0);
            assert_eq!(result.sample_2d(3.0, 3.0), 2.0);

            assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 2.0);
            assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 2.0);
            assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 2.0);
        }
    }

    mod f64 {
        use crate::task::f64::{ScaleBuilder, Task};
        #[test]
        fn scale_half() {
            let mut result = ScaleBuilder::new().scale(0.5).source(1.0).build();

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

        #[test]
        fn scale_double() {
            let mut result = ScaleBuilder::new().scale(2.0).source(1.0).build();

            assert_eq!(result.sample_1d(1.0), 2.0);
            assert_eq!(result.sample_1d(2.0), 2.0);
            assert_eq!(result.sample_1d(3.0), 2.0);

            assert_eq!(result.sample_2d(1.0, 1.0), 2.0);
            assert_eq!(result.sample_2d(2.0, 2.0), 2.0);
            assert_eq!(result.sample_2d(3.0, 3.0), 2.0);

            assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 2.0);
            assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 2.0);
            assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 2.0);
        }
    }
}
