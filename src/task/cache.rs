mod builder;

use builder::MAX_CACHE_ENTRY;

const CACHE_1D: usize = 0;
const CACHE_2D: usize = 1;
const CACHE_3D: usize = 2;

macro_rules! cache_value {
    ($type: ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
        pub(crate) struct CacheValue {
            pub x: $type,
            pub y: $type,
            pub z: $type,
            pub value: $type,
        }
    };
}

macro_rules! cache {
    ($type: ty) => {
        #[derive(Clone, Debug)]
        pub struct Cache {
            pub(crate) store: [Option<CacheValue>; MAX_CACHE_ENTRY],
            pub(crate) source: TaskSource,
        }

        impl Cache {
            fn eval<F: Fn(&mut dyn Task) -> $type>(
                &mut self,
                key: usize,
                coords: ($type, $type, $type),
                sampler: F,
            ) -> $type {
                if let Some(v) = self.store[key] {
                    if nearly_eq(v.x, coords.0) && nearly_eq(v.y, coords.1) && nearly_eq(v.z, coords.2)
                    {
                        return v.value;
                    }
                }

                self.store[key] = Some(CacheValue {
                    x: coords.0,
                    y: coords.1,
                    z: coords.2,
                    value: sampler(&mut self.source),
                });

                self.store[key].unwrap().value
            }
        }

        impl Task for Cache {
            fn sample_1d(&mut self, x: $type) -> $type {
                self.eval(CACHE_1D, (x, 0.0, 0.0), |t| t.sample_1d(x))
            }

            fn sample_2d(&mut self, x: $type, y: $type) -> $type {
                self.eval(CACHE_2D, (x, y, 0.0), |t| t.sample_2d(x, y))
            }

            fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type {
                self.eval(CACHE_3D, (x, y, z), |t| t.sample_3d(x, y, z))
            }
        }
    };
}

pub mod f32 {
    pub use super::builder::f32::CacheBuilder;
    use super::{CACHE_1D, CACHE_2D, CACHE_3D, MAX_CACHE_ENTRY};
    use crate::{
        math::f32::nearly_eq,
        task::f32::{Task, TaskSource},
    };
    cache_value!(f32);
    cache!(f32);
}

pub mod f64 {
    pub use super::builder::f64::CacheBuilder;
    use super::{CACHE_1D, CACHE_2D, CACHE_3D, MAX_CACHE_ENTRY};
    use crate::{
        math::f64::nearly_eq,
        task::f64::{Task, TaskSource},
    };
    cache_value!(f64);
    cache!(f64);
}

#[cfg(test)]
mod test {
    use super::{CACHE_1D, CACHE_2D, CACHE_3D};

    mod f32 {
        use super::{CACHE_1D, CACHE_2D, CACHE_3D};
        use crate::task::f32::{CacheBuilder, CacheValue, Task};

        #[test]
        fn value_cached() {
            let mut result = CacheBuilder::new().source(1.0).build();

            assert_eq!(result.sample_1d(1.0), 1.0);
            assert_eq!(result.sample_2d(1.0, 1.0), 1.0);
            assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 1.0);
            result.store[CACHE_1D] = Some(CacheValue {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                value: 2.0,
            });

            result.store[CACHE_2D] = Some(CacheValue {
                x: 1.0,
                y: 1.0,
                z: 0.0,
                value: 12345.0,
            });

            result.store[CACHE_3D] = Some(CacheValue {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                value: 54321.0,
            });

            assert_eq!(result.sample_1d(1.0), 2.0);
            assert_eq!(result.sample_2d(1.0, 1.0), 12345.0);
            assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 54321.0);

            result.store[CACHE_1D] = None;
            result.store[CACHE_2D] = None;
            result.store[CACHE_3D] = None;

            assert_eq!(result.sample_1d(1.0), 1.0);
            assert_eq!(result.sample_2d(1.0, 1.0), 1.0);
            assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 1.0);
        }
    }

    mod f64 {
        use super::{CACHE_1D, CACHE_2D, CACHE_3D};
        use crate::task::f64::{CacheBuilder, CacheValue, Task};

        #[test]
        fn value_cached() {
            let mut result = CacheBuilder::new().source(1.0).build();

            assert_eq!(result.sample_1d(1.0), 1.0);
            assert_eq!(result.sample_2d(1.0, 1.0), 1.0);
            assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 1.0);
            result.store[CACHE_1D] = Some(CacheValue {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                value: 2.0,
            });

            result.store[CACHE_2D] = Some(CacheValue {
                x: 1.0,
                y: 1.0,
                z: 0.0,
                value: 12345.0,
            });

            result.store[CACHE_3D] = Some(CacheValue {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                value: 54321.0,
            });

            assert_eq!(result.sample_1d(1.0), 2.0);
            assert_eq!(result.sample_2d(1.0, 1.0), 12345.0);
            assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 54321.0);

            result.store[CACHE_1D] = None;
            result.store[CACHE_2D] = None;
            result.store[CACHE_3D] = None;

            assert_eq!(result.sample_1d(1.0), 1.0);
            assert_eq!(result.sample_2d(1.0, 1.0), 1.0);
            assert_eq!(result.sample_3d(1.0, 1.0, 1.0), 1.0);
        }
    }
}
