mod aggregator;
mod bias;
mod cache;
mod cellular;
mod fractal;
mod gradient;
mod scale;
mod scale_offset;
mod selector;
mod task_source;
mod task_tree;
mod transform_domain;

pub(crate) use task_source::{named_to_task, source_or_message};

macro_rules! task_type {
    ($type: ty) => {
        /// Trait for generating noise values.
        pub trait Task {
            /// Evaluates the noise function at the given x-coordinate.
            fn sample_1d(&mut self, x: $type) -> $type;

            /// Evaluates the noise function at the given (x, y) coordinates.
            fn sample_2d(&mut self, x: $type, y: $type) -> $type;

            /// Evaluates the noise function at the given (x, y, z) coordinates.
            fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type;
        }
    };
}

pub mod f32 {
    pub use super::aggregator::f32::*;
    pub use super::bias::f32::*;
    pub use super::cache::f32::*;
    pub use super::cellular::f32::*;
    pub use super::fractal::f32::*;
    pub use super::gradient::f32::*;
    pub use super::scale::f32::*;
    pub use super::scale_offset::f32::*;
    pub use super::selector::f32::*;
    pub use super::task_source::f32::*;
    pub use super::task_tree::f32::*;
    pub use super::transform_domain::f32::*;
    task_type!(f32);
}

pub mod f64 {
    pub use super::aggregator::f64::*;
    pub use super::bias::f64::*;
    pub use super::cache::f64::*;
    pub use super::cellular::f64::*;
    pub use super::fractal::f64::*;
    pub use super::gradient::f64::*;
    pub use super::scale::f64::*;
    pub use super::scale_offset::f64::*;
    pub use super::selector::f64::*;
    pub use super::task_source::f64::*;
    pub use super::task_tree::f64::*;
    pub use super::transform_domain::f64::*;
    task_type!(f64);
}
