use serde::{Deserialize, Serialize};

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

macro_rules! fractal_config {
    ($type: ty) => {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
        #[serde(default)]
        pub struct FractalConfig {
            #[serde(alias = "amp")]
            pub amplitude: $type,
            #[serde(alias = "exp")]
            pub exponent: Option<$type>,
            pub fractal: FractalType,
            #[serde(alias = "freq")]
            pub frequency: $type,
            pub gain: $type,
            pub interp: FractalBlender,
            pub lacunarity: $type,
            pub octaves: u16,
            pub offset: Option<$type>,
            #[serde(alias = "src")]
            pub source: FractalSource,
            pub cached: bool,
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
                    cached: false,
                }
            }
        }

        impl TaskDependencies for FractalConfig {
            fn dependencies(&self) -> Vec<String> {
                vec![]
            }
        }

        impl IntoTaskSource for FractalConfig {
            fn config_into(&self, _: &TaskTree) -> TaskSource {
                let mut builder = FractalBuilder::new();

                let blender: Blender = match self.interp {
                    FractalBlender::Cubic => math::cubic_curve,
                    FractalBlender::Linear => math::linear_curve,
                    FractalBlender::Quintic => math::quintic_curve,
                };

                builder
                    .amplitude(self.amplitude)
                    .fractal(self.fractal)
                    .frequency(self.frequency)
                    .gain(self.gain)
                    .interp(blender)
                    .lacunarity(self.lacunarity)
                    .octaves(self.octaves)
                    .source(match self.source {
                        FractalSource::Perlin => Box::new(Perlin::new(blender)),
                    });

                builder.build().into()
            }
        }
    };
}

pub mod f32 {
    pub use super::{FractalBlender, FractalSource};
    use crate::math::f32 as math;
    use crate::ser::f32::{IntoTaskSource, TaskDependencies};
    use crate::source::f32::{Blender, Perlin};
    use crate::task::f32::{FractalBuilder, FractalType, TaskSource, TaskTree};
    fractal_config!(f32);
}

pub mod f64 {
    pub use super::{FractalBlender, FractalSource};
    use crate::math::f64 as math;
    use crate::ser::f64::{IntoTaskSource, TaskDependencies};
    use crate::source::f64::{Blender, Perlin};
    use crate::task::f64::{FractalBuilder, FractalType, TaskSource, TaskTree};
    fractal_config!(f64);
}

#[cfg(test)]
mod test {
    mod f32 {
        use std::collections::HashMap;

        use crate::ser::f32::{FractalConfig, TaskConfig};
        use crate::task::f32::FractalType;

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [fractal_a]
                fractal = { octaves = 1, frequency = 0.5, fractal = "fbm", cached = true }

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
                    cached: true,
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

    mod f64 {
        use std::collections::HashMap;

        use crate::ser::f64::{FractalConfig, TaskConfig};
        use crate::task::f64::FractalType;

        #[test]
        fn deserialize() {
            let data = toml::to_string(&toml::toml! {
                [fractal_a]
                fractal = { octaves = 1, frequency = 0.5, fractal = "fbm", cached = true }

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
                    cached: true,
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
}
