/// Fractal Brownian Motion
use super::NoiseConfig;
use crate::{float::Float, source::Noise};

fn eval<T: Float, F: Fn(&mut dyn Noise<T>, T, u16) -> T>(
    config: &NoiseConfig<T>,
    noise: &mut dyn Noise<T>,
    sampler: F,
) -> T {
    let mut result = T::ZERO;
    let mut amp = config.amplitude;
    let mut freq = config.frequency;

    let mut weight = T::ZERO;

    for octave in 0..config.octaves {
        let tmp = sampler(noise, freq, octave);
        result += tmp * amp;

        // used to normalize values generated.
        weight += amp;

        freq *= config.lacunarity;
        amp *= config.gain;
    }
    result /= weight;

    result
}

pub fn sample_1d<T: Float>(config: &NoiseConfig<T>, noise: &mut dyn Noise<T>, x: T) -> T {
    eval(config, noise, |s, f, o| {
        let o: T = T::from(o);
        (*s).sample_1d(x * f + o)
    })
}

pub fn sample_2d<T: Float>(config: &NoiseConfig<T>, noise: &mut dyn Noise<T>, x: T, y: T) -> T {
    eval(config, noise, |s, f, o| {
        let o: T = T::from(o);
        (*s).sample_2d(x * f + o, y * f + o)
    })
}

pub fn sample_3d<T: Float>(
    config: &NoiseConfig<T>,
    noise: &mut dyn Noise<T>,
    x: T,
    y: T,
    z: T,
) -> T {
    eval(config, noise, |s, f, o| {
        let o: T = T::from(o);
        (*s).sample_3d(x * f + o, y * f + o, z * f + o)
    })
}
