macro_rules! fractal_builder {
    ($type: ty) => {
        pub struct FractalBuilder {
            source: Box<dyn BoxNoise>,
            fractal: FractalType,
            blender: Blender,
            octaves: u16,
            lacunarity: $type,
            gain: $type,
            frequency: $type,
            amplitude: $type,

            /// Used in Ridged Multi
            offset: $type,
            /// Used in Ridged Multi
            exponent: $type,
        }

        #[allow(dead_code)]
        impl FractalBuilder {
            pub fn amplitude(&mut self, amplitude: $type) -> &mut Self {
                self.amplitude = amplitude;
                self
            }

            pub fn build(&self) -> Fractal {
                Fractal {
                    config: NoiseConfig {
                        octaves: self.octaves,
                        lacunarity: self.lacunarity,
                        gain: self.gain,
                        frequency: self.frequency,
                        amplitude: self.amplitude,
                    },
                    noise: self.source.box_clone(),
                    fractal: self.fractal,
                    pre_calc: match self.fractal {
                        FractalType::RidgedMulti => {
                            PreCalc::new(self.lacunarity, self.exponent, self.offset)
                        }
                        _ => PreCalc::default(),
                    },
                }
            }

            pub fn fractal(&mut self, fractal: FractalType) -> &mut Self {
                self.fractal = fractal;
                self
            }

            pub fn frequency(&mut self, frequency: $type) -> &mut Self {
                self.frequency = frequency;
                self
            }

            pub fn gain(&mut self, gain: $type) -> &mut Self {
                self.gain = gain;
                self
            }

            pub fn interp(&mut self, blender: Blender) -> &mut Self {
                self.blender = blender;
                self
            }

            pub fn lacunarity(&mut self, lacunarity: $type) -> &mut Self {
                self.lacunarity = lacunarity;
                self
            }

            pub fn new() -> Self {
                Self::default()
            }

            pub fn octaves(&mut self, octaves: u16) -> &mut Self {
                self.octaves = octaves;
                self
            }

            pub fn source(&mut self, src: Box<dyn BoxNoise + 'static>) -> &mut Self {
                self.source = src;
                self
            }
        }

        impl Default for FractalBuilder {
            fn default() -> Self {
                Self {
                    source: Box::new(Perlin::new(cubic_curve)),
                    fractal: FractalType::Brownian,
                    blender: cubic_curve,
                    octaves: 6,
                    lacunarity: 2.0,
                    gain: 0.5,
                    frequency: 1.0,
                    amplitude: 1.0,
                    offset: 1.0,
                    exponent: 0.9,
                }
            }
        }
    };
}

use super::{f32 as sf32, f64 as sf64, FractalType};

pub mod f32 {
    use super::{
        super::ridged_multi::f32::PreCalc,
        sf32::{Fractal, NoiseConfig},
        FractalType,
    };
    use crate::{
        math::f32::cubic_curve,
        source::f32::{Blender, BoxNoise, Perlin},
    };
    fractal_builder!(f32);
}

pub mod f64 {
    use super::{
        super::ridged_multi::f64::PreCalc,
        sf64::{Fractal, NoiseConfig},
        FractalType,
    };
    use crate::{
        math::f64::cubic_curve,
        source::f64::{Blender, BoxNoise, Perlin},
    };
    fractal_builder!(f64);
}
