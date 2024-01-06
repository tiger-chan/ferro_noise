pub mod math;
pub mod rng;
#[cfg(feature = "serde")]
pub mod ser;
pub mod source;
pub mod task;

pub mod prelude {
    pub use super::source::f32::{Gradient, Noise, Perlin};
    pub use super::task::f32::*;
}
