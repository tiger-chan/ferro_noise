mod builder;

use crate::float::Float;
pub use builder::CacheBuilder;
use builder::MAX_CACHE_ENTRY;

use super::{task::TaskSource, Task};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct CacheValue<T> {
    x: T,
    y: T,
    z: T,
    value: T,
}

const CACHE_1D: usize = 0;
const CACHE_2D: usize = 1;
const CACHE_3D: usize = 2;

pub struct Cache<T: Float> {
    store: [Option<CacheValue<T>>; MAX_CACHE_ENTRY],
    source: TaskSource<T>,
}

impl<T: Float> Cache<T> {
    fn eval<F: Fn(&mut dyn Task<T>) -> T>(
        &mut self,
        key: usize,
        coords: (T, T, T),
        sampler: F,
    ) -> T {
        if let Some(v) = self.store[key] {
            if v.x.nearly_eq(coords.0) && v.y.nearly_eq(coords.1) && v.z.nearly_eq(coords.2) {
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

impl<T: Float> Task<T> for Cache<T> {
    fn sample_1d(&mut self, x: T) -> T {
        self.eval(CACHE_1D, (x, T::ZERO, T::ZERO), |t| t.sample_1d(x))
    }

    fn sample_2d(&mut self, x: T, y: T) -> T {
        self.eval(CACHE_2D, (x, y, T::ZERO), |t| t.sample_2d(x, y))
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        self.eval(CACHE_3D, (x, y, z), |t| t.sample_3d(x, y, z))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn value_cached() {
        let mut result = CacheBuilder::<f32>::new().source(1.0).build();

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
