use crate::{
    float::Float,
    math::cubic_curve,
    source::{Blender, BoxNoise, Perlin},
};

use super::{ridged_multi::PreCalc, Fractal, FractalType, NoiseConfig};

pub struct FractalBuilder<T: Float> {
    source: Box<dyn BoxNoise<T>>,
    fractal: FractalType,
    blender: Blender<T>,
    octaves: u16,
    lacunarity: T,
    gain: T,
    frequency: T,
    amplitude: T,

    /// Used in Ridged Multi
    offset: T,
    /// Used in Ridged Multi
    exponent: T,
}

#[allow(dead_code)]
impl<T: Float> FractalBuilder<T> {
    pub fn amplitude(&mut self, amplitude: T) -> &mut Self {
        self.amplitude = amplitude;
        self
    }

    pub fn build(&self) -> Fractal<T> {
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

    pub fn frequency(&mut self, frequency: T) -> &mut Self {
        self.frequency = frequency;
        self
    }

    pub fn gain(&mut self, gain: T) -> &mut Self {
        self.gain = gain;
        self
    }

    pub fn interp(&mut self, blender: Blender<T>) -> &mut Self {
        self.blender = blender;
        self
    }

    pub fn lacunarity(&mut self, lacunarity: T) -> &mut Self {
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

    pub fn source(&mut self, src: Box<dyn BoxNoise<T> + 'static>) -> &mut Self {
        self.source = src;
        self
    }
}

impl<T: Float> Default for FractalBuilder<T> {
    fn default() -> Self {
        Self {
            source: Box::new(Perlin::<T>::new(cubic_curve)),
            fractal: FractalType::Brownian,
            blender: cubic_curve,
            octaves: 6,
            lacunarity: T::TWO,
            gain: T::from(0.5),
            frequency: T::ONE,
            amplitude: T::ONE,
            offset: T::ONE,
            exponent: T::from(0.9),
        }
    }
}
