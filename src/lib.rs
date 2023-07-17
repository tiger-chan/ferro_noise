pub mod math;
#[cfg(feature = "serde")]
pub mod ser;
pub mod source;
mod task;

pub mod prelude {
    pub use super::source::f32::{Gradient, Noise, Perlin};
    pub use super::task::f32::*;
}
