mod aggregate_config;
mod bias_config;
mod cellular_config;
mod fractal_config;
mod gradient_config;
mod name_or_const;
mod noise;
mod scale_config;
mod scale_offset_config;
mod selector_config;
mod transform_domain_config;

pub mod f32 {
    pub(crate) use super::aggregate_config::f32::*;
    pub(crate) use super::bias_config::f32::*;
    pub(crate) use super::cellular_config::f32::*;
    pub(crate) use super::fractal_config::f32::*;
    pub(crate) use super::gradient_config::f32::*;
    pub(crate) use super::name_or_const::f32::*;
    pub use super::noise::f32::*;
    pub(crate) use super::scale_config::f32::*;
    pub(crate) use super::scale_offset_config::f32::*;
    pub(crate) use super::selector_config::f32::*;
    pub(crate) use super::transform_domain_config::f32::*;
}

pub mod f64 {
    pub(crate) use super::aggregate_config::f64::*;
    pub(crate) use super::bias_config::f64::*;
    pub(crate) use super::cellular_config::f64::*;
    pub(crate) use super::fractal_config::f64::*;
    pub(crate) use super::gradient_config::f64::*;
    pub(crate) use super::name_or_const::f64::*;
    pub use super::noise::f64::*;
    pub(crate) use super::scale_config::f64::*;
    pub(crate) use super::scale_offset_config::f64::*;
    pub(crate) use super::selector_config::f64::*;
    pub(crate) use super::transform_domain_config::f64::*;
}
