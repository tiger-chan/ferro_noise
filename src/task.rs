mod bias;
mod cache;
mod gradient;
mod perlin;
mod selector;
mod task;

pub use bias::*;
pub use cache::*;
pub use gradient::*;
pub use perlin::*;
pub use selector::*;

use crate::float::Float;

/// Trait for generating noise values.
pub trait Task<T: Float> {
    /// Evaluates the noise function at the given x-coordinate.
    fn sample_1d(&mut self, x: T) -> T;

    /// Evaluates the noise function at the given (x, y) coordinates.
    fn sample_2d(&mut self, x: T, y: T) -> T;

    /// Evaluates the noise function at the given (x, y, z) coordinates.
    fn sample_3d(&mut self, x: T, y: T, z: T) -> T;
}
