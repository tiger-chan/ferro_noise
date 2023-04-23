/// Ridged Multi Fractal
use super::{NoiseConfig, ScaleBias};
use crate::{float::Float, source::Noise};

pub const MAX_OCTAVES: usize = 20;

pub struct PreCalc<T: Float> {
    pub offset: T,
    /// Each successive gain is raised to the power of -h
    pub exponent: T,
    gain: [T; MAX_OCTAVES],
    scale_bias: [ScaleBias<T>; MAX_OCTAVES],
}

impl<T: Float> Default for PreCalc<T> {
    fn default() -> Self {
        Self {
            offset: T::ONE,
            exponent: T::from(0.9),
            gain: [T::default(); MAX_OCTAVES],
            scale_bias: [ScaleBias::default(); MAX_OCTAVES],
        }
    }
}

impl<T: Float> PreCalc<T> {
    /// Calculate scale/bias by guessing at minimum and maximum values and remapping to [-1,1]
    pub fn new(lacunarity: T, exp: T, offset: T) -> Self {
        let mut gain = [T::default(); MAX_OCTAVES];
        let mut scale_bias = [ScaleBias::default(); MAX_OCTAVES];

        let mut max = T::ZERO;
        let mut min = T::ZERO;
        for i in 0..MAX_OCTAVES {
            gain[i] = lacunarity.powf(T::from(-(i as f32)) * exp);

            let offset_one = offset - T::ONE;
            min += offset_one * offset_one * gain[i];
            max += offset * offset * gain[i];

            let a = -T::ONE;
            let scale = T::TWO / (max - min);
            let bias = a - min * scale;
            scale_bias[i] = ScaleBias { scale, bias };
        }

        Self {
            exponent: exp,
            offset,
            gain,
            scale_bias,
        }
    }
}

fn eval<T: Float, F: Fn(&mut dyn Noise<T>, T, u16) -> T>(
    config: &NoiseConfig<T>,
    pre_calc: &PreCalc<T>,
    noise: &mut dyn Noise<T>,
    sampler: F,
) -> T {
    let mut result = T::ZERO;
    let mut freq = config.frequency;

    for octave in 0..config.octaves {
        let tmp = sampler(noise, freq, octave);
        let tmp = pre_calc.offset - tmp.abs();
        let tmp = tmp * tmp;
        result += tmp * pre_calc.gain[octave as usize];

        freq *= config.lacunarity;
    }
    let sb = pre_calc.scale_bias[(config.octaves - 1) as usize];
    sb.bias + result * sb.scale
}

pub fn sample_1d<T: Float>(
    config: &NoiseConfig<T>,
    pre_calc: &PreCalc<T>,
    noise: &mut dyn Noise<T>,
    x: T,
) -> T {
    eval(config, pre_calc, noise, |s, f, o| {
        let o: T = T::from(o);
        (*s).sample_1d(x * f + o)
    })
}

pub fn sample_2d<T: Float>(
    config: &NoiseConfig<T>,
    pre_calc: &PreCalc<T>,
    noise: &mut dyn Noise<T>,
    x: T,
    y: T,
) -> T {
    eval(config, pre_calc, noise, |s, f, o| {
        let o: T = T::from(o);
        (*s).sample_2d(x * f + o, y * f + o)
    })
}

pub fn sample_3d<T: Float>(
    config: &NoiseConfig<T>,
    pre_calc: &PreCalc<T>,
    noise: &mut dyn Noise<T>,
    x: T,
    y: T,
    z: T,
) -> T {
    eval(config, pre_calc, noise, |s, f, o| {
        let o: T = T::from(o);
        (*s).sample_3d(x * f + o, y * f + o, z * f + o)
    })
}
