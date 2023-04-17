use crate::float::Float;

use super::{task::TaskType, Task};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct CacheValue<T> {
    x: T,
    y: T,
    z: T,
    value: T,
}

const MAX_CACHE_ENTRY: usize = 3;
const CACHE_1D: usize = 0;
const CACHE_2D: usize = 1;
const CACHE_3D: usize = 2;

pub struct Cache<T: Float> {
    store: [Option<CacheValue<T>>; MAX_CACHE_ENTRY],
    source: TaskType<T>,
}

impl<T: Float> Cache<T> {
    #[allow(dead_code)]
    pub fn new(source: TaskType<T>) -> Self {
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
