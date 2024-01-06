mod blender;
mod cellular;
mod gradient;
mod perlin_noise;
mod vec3;

macro_rules! noise_trait {
    ($type: ty) => {
        /// Trait for generating noise values.
        pub trait Noise: Debug {
            /// Evaluates the noise function at the given x-coordinate.
            fn sample_1d(&mut self, x: $type) -> $type;

            /// Evaluates the noise function at the given (x, y) coordinates.
            fn sample_2d(&mut self, x: $type, y: $type) -> $type;

            /// Evaluates the noise function at the given (x, y, z) coordinates.
            fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type;
        }
    };
}

macro_rules! boxed_noise_trait {
    () => {
        pub trait BoxNoise: Noise {
            fn box_clone(&self) -> Box<dyn Noise + 'static>;
        }
    };
}

pub mod f32 {
    use std::fmt::Debug;
    noise_trait!(f32);
    boxed_noise_trait!();

    pub use super::blender::f32::Blender;
    pub use super::cellular::f32::{Cellular, Distance};
    pub use super::gradient::f32::Gradient;
    pub use super::perlin_noise::f32::Perlin;
}

pub mod f64 {
    use std::fmt::Debug;
    noise_trait!(f64);
    boxed_noise_trait!();

    pub use super::blender::f64::Blender;
    pub use super::cellular::f64::{Cellular, Distance};
    pub use super::gradient::f64::Gradient;
    pub use super::perlin_noise::f64::Perlin;
}
