pub mod float;
pub mod math;
pub mod source;
mod task;

pub mod prelude {
    pub use super::float::Float;
    pub use super::math;
    pub use super::source::{Gradient, Noise, Perlin};
    pub use super::task::*;
}
