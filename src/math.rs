mod algorithm;
mod easing;

pub use easing::*;

pub mod f32 {
    pub use super::algorithm::f32::*;
    pub use super::easing::f32::*;
}

pub mod f64 {
    pub use super::algorithm::f64::*;
	pub use super::easing::f64::*;
}
