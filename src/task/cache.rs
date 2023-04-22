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
    #[allow(dead_code)]
    pub fn new(source: TaskSource<T>) -> Self {
        Self {
            store: [None; MAX_CACHE_ENTRY],
            source: source,
        }
    }
}

impl<T: Float> Task<T> for Cache<T> {
    fn sample_1d(&mut self, x: T) -> T {
        if let Some(v) = self.store[CACHE_1D] {
            if v.x.nearly_eq(x) {
                return v.value;
            }
        }

        self.store[CACHE_1D] = Some(CacheValue {
            x,
            y: T::from(0.0),
            z: T::from(0.0),
            value: self.source.sample_1d(x),
        });

        self.store[CACHE_1D].unwrap().value
    }

    fn sample_2d(&mut self, x: T, y: T) -> T {
        if let Some(v) = self.store[CACHE_2D] {
            if v.x.nearly_eq(x) && v.y.nearly_eq(y) {
                return v.value;
            }
        }

        self.store[CACHE_2D] = Some(CacheValue {
            x,
            y,
            z: T::from(0.0),
            value: self.source.sample_2d(x, y),
        });

        self.store[CACHE_2D].unwrap().value
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        if let Some(v) = self.store[CACHE_3D] {
            if v.x.nearly_eq(x) && v.y.nearly_eq(y) && v.z.nearly_eq(z) {
                return v.value;
            }
        }
        {
            self.store[CACHE_3D] = Some(CacheValue {
                x,
                y,
                z,
                value: self.source.sample_3d(x, y, z),
            });

            self.store[CACHE_3D].unwrap().value
        }
    }
}

#[cfg(test)]
mod test {
    use super::{TaskSource::Constant, *};

    #[test]
    fn value_cached() {
        let mut result = CacheBuilder::<f32>::new().source(Constant(1.0)).build();

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
