/// Fractal Brownian Motion

macro_rules! eval {
    ($type: ty) => {
        fn eval<F: Fn(&mut dyn Noise, $type, u16) -> $type>(
            config: &NoiseConfig,
            noise: &mut dyn Noise,
            sampler: F,
        ) -> $type {
            let mut result = 0.0;
            let mut amp = config.amplitude;
            let mut freq = config.frequency;

            let mut weight = 0.0;

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
    };
}

macro_rules! sample_1d {
    ($type: ty) => {
        pub fn sample_1d(config: &NoiseConfig, noise: &mut dyn Noise, x: $type) -> $type {
            eval(config, noise, |s, f, o| {
                let o: $type = o as $type;
                (*s).sample_1d(x * f + o)
            })
        }
    };
}

macro_rules! sample_2d {
    ($type: ty) => {
        pub fn sample_2d(config: &NoiseConfig, noise: &mut dyn Noise, x: $type, y: $type) -> $type {
            eval(config, noise, |s, f, o| {
                let o: $type = o as $type;
                (*s).sample_2d(x * f + o, y * f + o)
            })
        }
    };
}

macro_rules! sample_3d {
    ($type: ty) => {
        pub fn sample_3d(
            config: &NoiseConfig,
            noise: &mut dyn Noise,
            x: $type,
            y: $type,
            z: $type,
        ) -> $type {
            eval(config, noise, |s, f, o| {
                let o: $type = o as $type;
                (*s).sample_3d(x * f + o, y * f + o, z * f + o)
            })
        }
    };
}

pub mod f32 {
    use super::super::f32::NoiseConfig;
    use crate::source::f32::Noise;
    eval!(f32);
    sample_1d!(f32);
    sample_2d!(f32);
    sample_3d!(f32);
}

pub mod f64 {
    use super::super::f64::NoiseConfig;
    use crate::source::f64::Noise;
    eval!(f64);
    sample_1d!(f64);
    sample_2d!(f64);
    sample_3d!(f64);
}
