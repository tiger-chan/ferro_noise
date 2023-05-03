use serde::{Deserialize, Serialize};

use crate::{
    float::Float,
    math::{cubic_curve, linear_curve, quintic_curve},
    prelude::Perlin,
    source::Blender,
    task::{FractalBuilder, FractalType, TaskSource, TaskTree},
};

use super::{IntoTaskSource, TaskDependencies};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd, Default)]
#[serde(rename_all = "snake_case")]
pub enum FractalSource {
    #[default]
    Perlin,
    // todo!() Eventually this will include simplex noise
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd, Default)]
#[serde(rename_all = "snake_case")]
pub enum FractalBlender {
    Linear,
    #[serde(alias = "hermite")]
    Cubic,
    #[default]
    Quintic,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(default)]
pub struct FractalConfig {
    #[serde(alias = "amp")]
    pub amplitude: f64,
    #[serde(alias = "exp")]
    pub exponent: Option<f64>,
    pub fractal: FractalType,
    #[serde(alias = "freq")]
    pub frequency: f64,
    pub gain: f64,
    pub interp: FractalBlender,
    pub lacunarity: f64,
    pub octaves: u16,
    pub offset: Option<f64>,
    #[serde(alias = "src")]
    pub source: FractalSource,
}

impl Default for FractalConfig {
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            exponent: None,
            fractal: FractalType::default(),
            frequency: 1.0,
            gain: 0.5,
            interp: FractalBlender::default(),
            lacunarity: 2.0,
            octaves: 6,
            offset: None,
            source: FractalSource::default(),
        }
    }
}

impl TaskDependencies for FractalConfig {
    fn dependencies(&self) -> Vec<String> {
        vec![]
    }
}

impl<T: Float> IntoTaskSource<T> for FractalConfig {
    fn config_into(&self, _: &TaskTree<T>) -> TaskSource<T> {
        let mut builder = FractalBuilder::<T>::new();

        let blender: Blender<T> = match self.interp {
            FractalBlender::Cubic => cubic_curve::<T>,
            FractalBlender::Linear => linear_curve::<T>,
            FractalBlender::Quintic => quintic_curve::<T>,
        };

        builder
            .amplitude(T::as_float(self.amplitude))
            .fractal(self.fractal)
            .frequency(T::as_float(self.frequency))
            .gain(T::as_float(self.gain))
            .interp(blender)
            .lacunarity(T::as_float(self.lacunarity))
            .octaves(self.octaves)
            .source(match self.source {
                FractalSource::Perlin => Box::new(Perlin::new(blender)),
            });

        builder.build().into()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::ser::TaskConfig;

    use super::*;

    #[test]
    fn deserialize() {
        let data = toml::to_string(&toml::toml! {
            [fractal_a]
            fractal = { octaves = 1, frequency = 0.5, fractal = "fbm" }

            [fractal_b]
            fractal = { octaves = 2, freq = 0.9 }
        })
        .unwrap();
        let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

        assert_eq!(config.len(), 2);
        assert_eq!(
            config["fractal_a"],
            TaskConfig::Fractal(FractalConfig {
                octaves: 1,
                fractal: FractalType::Brownian,
                frequency: 0.5,
                ..Default::default()
            })
        );

        assert_eq!(
            config["fractal_b"],
            TaskConfig::Fractal(FractalConfig {
                octaves: 2,
                frequency: 0.9,
                ..Default::default()
            })
        );
    }
}
