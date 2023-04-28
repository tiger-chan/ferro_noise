use serde::{Deserialize, Serialize};

use crate::task::FractalType;

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
