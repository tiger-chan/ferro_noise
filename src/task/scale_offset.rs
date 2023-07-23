mod builder;

macro_rules! scale_offset {
    ($type: ty) => {
        #[derive(Clone, Debug)]
        pub struct ScaleOffset {
            pub(crate) offset: TaskSource,
            pub(crate) scale: TaskSource,
            pub(crate) source: TaskSource,
        }

        impl ScaleOffset {
            fn eval<F: Fn(&mut dyn Task) -> $type>(&mut self, sampler: F) -> $type {
                let v = sampler(&mut self.source);
                let s = sampler(&mut self.scale);
                let o = sampler(&mut self.offset);
                v * s + o
            }
        }

        impl Task for ScaleOffset {
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
    pub use super::builder::f32::ScaleOffsetBuilder;
    use crate::task::f32::{Task, TaskSource};
    scale_offset!(f32);
}

pub mod f64 {
    pub use super::builder::f64::ScaleOffsetBuilder;
    use crate::task::f64::{Task, TaskSource};
    scale_offset!(f64);
}

#[cfg(test)]
mod tests {
    mod f32 {
        use crate::task::f32::{ScaleOffsetBuilder, Task};
        #[test]
        fn scale_half() {
            let mut result = ScaleOffsetBuilder::new().scale(0.5).source(1.0).build();

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
        fn scale_double_negative_offset() {
            let mut result = ScaleOffsetBuilder::new()
                .scale(2.0)
                .source(1.0)
                .offset(-1.0)
                .build();

            assert_eq!(result.sample_1d(1.0), 1.0);
            assert_eq!(result.sample_1d(2.0), 1.0);
            assert_eq!(result.sample_1d(3.0), 1.0);

            assert_eq!(result.sample_2d(1.0, 1.0), 1.0);
            assert_eq!(result.sample_2d(2.0, 2.0), 1.0);
            assert_eq!(result.sample_2d(3.0, 3.0), 1.0);

            assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 1.0);
            assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 1.0);
            assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 1.0);
        }
    }

    mod f64 {
        use crate::task::f64::{ScaleOffsetBuilder, Task};
        #[test]
        fn scale_half() {
            let mut result = ScaleOffsetBuilder::new().scale(0.5).source(1.0).build();

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
        fn scale_double_negative_offset() {
            let mut result = ScaleOffsetBuilder::new()
                .scale(2.0)
                .source(1.0)
                .offset(-1.0)
                .build();

            assert_eq!(result.sample_1d(1.0), 1.0);
            assert_eq!(result.sample_1d(2.0), 1.0);
            assert_eq!(result.sample_1d(3.0), 1.0);

            assert_eq!(result.sample_2d(1.0, 1.0), 1.0);
            assert_eq!(result.sample_2d(2.0, 2.0), 1.0);
            assert_eq!(result.sample_2d(3.0, 3.0), 1.0);

            assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 1.0);
            assert_eq!(result.sample_3d(2.0, 2.0, 2.0), 1.0);
            assert_eq!(result.sample_3d(3.0, 3.0, 3.0), 1.0);
        }
    }
}
