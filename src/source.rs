mod blender;
mod gradient;
mod perlin_noise;

use std::fmt::Debug;

pub use blender::Blender;
pub use gradient::Gradient;
pub use perlin_noise::Perlin;

use crate::float::Float;

/// Trait for generating noise values.
pub trait Noise<T: Float> : Debug {
    /// Evaluates the noise function at the given x-coordinate.
    fn sample_1d(&mut self, x: T) -> T;

    /// Evaluates the noise function at the given (x, y) coordinates.
    fn sample_2d(&mut self, x: T, y: T) -> T;

    /// Evaluates the noise function at the given (x, y, z) coordinates.
    fn sample_3d(&mut self, x: T, y: T, z: T) -> T;
}

pub trait BoxNoise<T: Float>: Noise<T> {
    fn box_clone(&self) -> Box<dyn Noise<T> + 'static>;
}
