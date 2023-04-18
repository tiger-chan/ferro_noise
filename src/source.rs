mod blender;
mod perlin_noise;

pub use blender::Blender;
pub use perlin_noise::Perlin;

use crate::float::Float;

/// Trait for generating noise values.
pub trait Noise<T: Float> {
    /// Evaluates the noise function at the given x-coordinate.
    fn sample_1d(&mut self, x: T) -> T;

    /// Evaluates the noise function at the given (x, y) coordinates.
    fn sample_2d(&mut self, x: T, y: T) -> T;

    /// Evaluates the noise function at the given (x, y, z) coordinates.
    fn sample_3d(&mut self, x: T, y: T, z: T) -> T;
}
