/// Ridged Multi Fractal
pub const MAX_OCTAVES: usize = 20;

macro_rules! pre_calc {
    ($type: ty) => {
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct PreCalc {
            pub offset: $type,
            /// Each successive gain is raised to the power of -h
            pub exponent: $type,
            gain: [$type; MAX_OCTAVES],
            scale_bias: [ScaleBias; MAX_OCTAVES],
        }

        impl Default for PreCalc {
            fn default() -> Self {
                Self {
                    offset: 1.0,
                    exponent: 0.9,
                    gain: [0.0; MAX_OCTAVES],
                    scale_bias: [ScaleBias::default(); MAX_OCTAVES],
                }
            }
        }

        impl PreCalc {
            /// Calculate scale/bias by guessing at minimum and maximum values and remapping to [-1,1]
            pub fn new(lacunarity: $type, exp: $type, offset: $type) -> Self {
                let mut gain = [0.0; MAX_OCTAVES];
                let mut scale_bias = [ScaleBias::default(); MAX_OCTAVES];

                let mut max = 0.0;
                let mut min = 0.0;
                for i in 0..MAX_OCTAVES {
                    gain[i] = lacunarity.powf(-(i as $type) * exp);

                    let offset_one = offset - 1.0;
                    min += offset_one * offset_one * gain[i];
                    max += offset * offset * gain[i];

                    let a = -1.0;
                    let scale = 2.0 / (max - min);
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
    };
}

macro_rules! eval {
    ($type: ty) => {
        fn eval<F: Fn(&mut dyn Noise, $type, u16) -> $type>(
            config: &NoiseConfig,
            pre_calc: &PreCalc,
            noise: &mut dyn Noise,
            sampler: F,
        ) -> $type {
            let mut result = 0.0;
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
    };
}

macro_rules! sample_1d {
    ($type: ty) => {
        pub fn sample_1d(
            config: &NoiseConfig,
            pre_calc: &PreCalc,
            noise: &mut dyn Noise,
            x: $type,
        ) -> $type {
            eval(config, pre_calc, noise, |s, f, o| {
                let o: $type = o as $type;
                (*s).sample_1d(x * f + o)
            })
        }
    };
}

macro_rules! sample_2d {
    ($type: ty) => {
        pub fn sample_2d(
            config: &NoiseConfig,
            pre_calc: &PreCalc,
            noise: &mut dyn Noise,
            x: $type,
            y: $type,
        ) -> $type {
            eval(config, pre_calc, noise, |s, f, o| {
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
            pre_calc: &PreCalc,
            noise: &mut dyn Noise,
            x: $type,
            y: $type,
            z: $type,
        ) -> $type {
            eval(config, pre_calc, noise, |s, f, o| {
                let o: $type = o as $type;
                (*s).sample_3d(x * f + o, y * f + o, z * f + o)
            })
        }
    };
}

pub mod f32 {
    use super::{
        super::f32::{NoiseConfig, ScaleBias},
        MAX_OCTAVES,
    };
    use crate::source::f32::Noise;
    pre_calc!(f32);
    eval!(f32);
    sample_1d!(f32);
    sample_2d!(f32);
    sample_3d!(f32);
}

pub mod f64 {
    use super::{
        super::f64::{NoiseConfig, ScaleBias},
        MAX_OCTAVES,
    };
    use crate::source::f64::Noise;
    pre_calc!(f64);
    eval!(f64);
    sample_1d!(f64);
    sample_2d!(f64);
    sample_3d!(f64);
}
