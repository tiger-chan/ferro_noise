use crate::{float::Float, source::Noise};

use super::Task;

const MAX_OCTAVES: usize = 20;

#[allow(dead_code)]
pub enum FractalType {
    /// Billowing
    Billowing,
    /// Fractal Brownian Motion
    Brownian,
    /// Ridged Multi Fractal
    RidgedMulti,
}

/// Fractal Brownian Motion
mod fbm {
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
}

/// Billowing
mod billowing {
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
            let tmp = T::TWO * tmp.abs() - T::ONE;
            result += tmp * amp;

            // used to normalize values generated.
            weight += amp;

            freq *= config.lacunarity;
            amp *= config.gain;
        }
        result /= weight;
        result += T::from(0.5);

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
}

/// Ridged Multi Fractal
mod ridged_multi {
    use super::{NoiseConfig, PreCalc};
    use crate::{float::Float, source::Noise};

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
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct ScaleBias<T> {
    scale: T,
    bias: T,
}

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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct NoiseConfig<T> {
    pub octaves: u16,
    pub lacunarity: T,
    pub gain: T,
    pub frequency: T,
    pub amplitude: T,
}

impl<T: Float> Default for NoiseConfig<T> {
    fn default() -> Self {
        NoiseConfig {
            octaves: 6,
            lacunarity: T::TWO,
            gain: T::from(0.5),
            frequency: T::ONE,
            amplitude: T::ONE,
        }
    }
}

pub struct Fractal<T: Float> {
    config: NoiseConfig<T>,
    noise: Box<dyn Noise<T>>,
    fractal: FractalType,
    /// Only used in Ridged Multi
    pre_calc: PreCalc<T>,
}

impl<T: Float> Fractal<T> {
    #[allow(dead_code)]
    pub fn new_fbm(noise: Box<dyn Noise<T>>) -> Self {
        Self {
            config: NoiseConfig::default(),
            noise: noise,
            fractal: FractalType::Brownian,
            pre_calc: PreCalc::default(),
        }
    }

    #[allow(dead_code)]
    pub fn new_fbm_with_config(noise: Box<dyn Noise<T>>, config: NoiseConfig<T>) -> Self {
        Self {
            config: config,
            noise: noise,
            fractal: FractalType::Brownian,
            pre_calc: PreCalc::default(),
        }
    }

    #[allow(dead_code)]
    pub fn new_billowing(noise: Box<dyn Noise<T>>) -> Self {
        Self {
            config: NoiseConfig::default(),
            noise: noise,
            fractal: FractalType::Billowing,
            pre_calc: PreCalc::default(),
        }
    }

    #[allow(dead_code)]
    pub fn new_billowing_with_config(noise: Box<dyn Noise<T>>, config: NoiseConfig<T>) -> Self {
        Self {
            config: config,
            noise: noise,
            fractal: FractalType::Billowing,
            pre_calc: PreCalc::default(),
        }
    }

    #[allow(dead_code)]
    pub fn new_ridged_multi(noise: Box<dyn Noise<T>>) -> Self {
        let config = NoiseConfig::default();
        let pre_calc = PreCalc::default();
        Self {
            config: config,
            noise: noise,
            fractal: FractalType::RidgedMulti,
            pre_calc: PreCalc::new(config.lacunarity, pre_calc.exponent, pre_calc.offset),
        }
    }

    #[allow(dead_code)]
    pub fn new_ridged_multi_with_config(
        noise: Box<dyn Noise<T>>,
        config: NoiseConfig<T>,
        exp: T,
        offset: T,
    ) -> Self {
        Self {
            config: config,
            noise: noise,
            fractal: FractalType::RidgedMulti,
            pre_calc: PreCalc::new(config.lacunarity, exp, offset),
        }
    }
}

impl<T: Float> Task<T> for Fractal<T> {
    fn sample_1d(&mut self, x: T) -> T {
        match self.fractal {
            FractalType::Brownian => fbm::sample_1d(&self.config, self.noise.as_mut(), x),
            FractalType::Billowing => billowing::sample_1d(&self.config, self.noise.as_mut(), x),
            FractalType::RidgedMulti => {
                ridged_multi::sample_1d(&self.config, &self.pre_calc, self.noise.as_mut(), x)
            }
        }
    }

    fn sample_2d(&mut self, x: T, y: T) -> T {
        match self.fractal {
            FractalType::Brownian => fbm::sample_2d(&self.config, self.noise.as_mut(), x, y),
            FractalType::Billowing => billowing::sample_2d(&self.config, self.noise.as_mut(), x, y),
            FractalType::RidgedMulti => {
                ridged_multi::sample_2d(&self.config, &self.pre_calc, self.noise.as_mut(), x, y)
            }
        }
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        match self.fractal {
            FractalType::Brownian => fbm::sample_3d(&self.config, self.noise.as_mut(), x, y, z),
            FractalType::Billowing => {
                billowing::sample_3d(&self.config, self.noise.as_mut(), x, y, z)
            }
            FractalType::RidgedMulti => {
                ridged_multi::sample_3d(&self.config, &self.pre_calc, self.noise.as_mut(), x, y, z)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fractal_fbm_is_smooth() {
        use crate::{math::cubic_curve, source};
        let source = Box::new(source::Perlin::new(cubic_curve));
        let mut result = Fractal::new_fbm_with_config(
            source,
            NoiseConfig {
                octaves: 1,
                ..Default::default()
            },
        );

        // 1d testing
        {
            let samples_1d = [0.1, 0.14, 0.2, 0.23];
            let mut value_total = [0.0; 4];
            for (i, x) in samples_1d.iter().enumerate() {
                value_total[i] = result.sample_1d(*x);
            }
            let diffs = value_total.windows(2).map(|w| w[1] - w[0]);
            let sum: f32 = diffs.clone().sum();
            let count = (diffs.count() + 1) as f32;
            let avg_diff = (sum / count).abs();

            assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
        }

        // 2d testing
        {
            let samples_x = [0.4, 0.43, 0.43, 0.45];
            let samples_y = [0.5, 0.5, 0.55, 0.55];
            let mut value_total = [0.0; 4];
            for (i, (x, y)) in samples_x.iter().zip(samples_y).enumerate() {
                value_total[i] += result.sample_2d(*x, y);
            }
            let diffs = value_total.windows(2).map(|w| w[1] - w[0]);
            let sum: f32 = diffs.clone().sum();
            let count = (diffs.count() + 1) as f32;
            let avg_diff = (sum / count).abs();

            assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
        }

        // 3d testing
        {
            let samples_x = [0.4, 0.6, 0.5, 0.5];
            let samples_y = [0.5, 0.5, 0.4, 0.6];
            let samples_z = [0.4, 0.5, 0.5, 0.6];
            let mut value_total = [0.0; 4];
            for (i, ((x, y), z)) in samples_x.iter().zip(samples_y).zip(samples_z).enumerate() {
                value_total[i] += result.sample_3d(*x, y, z);
            }
            let diffs = value_total.windows(2).map(|w| w[1] - w[0]);
            let sum: f32 = diffs.clone().sum();
            let count = (diffs.count() + 1) as f32;
            let avg_diff = (sum / count).abs();

            assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
        }
    }

    #[test]
    fn fbm_is_continuous() {
        use crate::{math::cubic_curve, source};
        let source = Box::new(source::Perlin::<f32>::new(cubic_curve));
        let mut result = Fractal::new_fbm_with_config(
            source,
            NoiseConfig {
                octaves: 1,
                ..Default::default()
            },
        );

        // 1d testing
        {
            let sample1 = result.sample_1d(0.1);
            let sample2 = result.sample_1d(0.11);

            assert!((sample2 - sample1).abs() < 0.1);
        }

        // 2d testing
        {
            let sample1 = result.sample_2d(0.1, 0.1);
            let sample2 = result.sample_2d(0.11, 0.11);

            assert!((sample2 - sample1).abs() < 0.1);
        }

        // 3d testing
        {
            let sample1 = result.sample_3d(0.1, 0.1, 0.1);
            let sample2 = result.sample_3d(0.11, 0.11, 0.11);

            assert!((sample2 - sample1).abs() < 0.1);
        }
    }

    #[test]
    fn billowing_is_smooth() {
        use crate::{math::cubic_curve, source};
        let source = Box::new(source::Perlin::new(cubic_curve));
        let mut result = Fractal::new_billowing_with_config(
            source,
            NoiseConfig {
                octaves: 1,
                ..Default::default()
            },
        );

        // 1d testing
        {
            let samples_1d = [0.1, 0.14, 0.2, 0.23];
            let mut value_total = [0.0; 4];
            for (i, x) in samples_1d.iter().enumerate() {
                value_total[i] = result.sample_1d(*x);
            }
            let diffs = value_total.windows(2).map(|w| w[1] - w[0]);
            let sum: f32 = diffs.clone().sum();
            let count = (diffs.count() + 1) as f32;
            let avg_diff = (sum / count).abs();

            assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
        }

        // 2d testing
        {
            let samples_x = [0.4, 0.43, 0.43, 0.45];
            let samples_y = [0.5, 0.5, 0.55, 0.55];
            let mut value_total = [0.0; 4];
            for (i, (x, y)) in samples_x.iter().zip(samples_y).enumerate() {
                value_total[i] += result.sample_2d(*x, y);
            }
            let diffs = value_total.windows(2).map(|w| w[1] - w[0]);
            let sum: f32 = diffs.clone().sum();
            let count = (diffs.count() + 1) as f32;
            let avg_diff = (sum / count).abs();

            assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
        }

        // 3d testing
        {
            let samples_x = [0.4, 0.6, 0.5, 0.5];
            let samples_y = [0.5, 0.5, 0.4, 0.6];
            let samples_z = [0.4, 0.5, 0.5, 0.6];
            let mut value_total = [0.0; 4];
            for (i, ((x, y), z)) in samples_x.iter().zip(samples_y).zip(samples_z).enumerate() {
                value_total[i] += result.sample_3d(*x, y, z);
            }
            let diffs = value_total.windows(2).map(|w| w[1] - w[0]);
            let sum: f32 = diffs.clone().sum();
            let count = (diffs.count() + 1) as f32;
            let avg_diff = (sum / count).abs();

            assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
        }
    }

    #[test]
    fn billowing_is_continuous() {
        use crate::{math::cubic_curve, source};
        let source = Box::new(source::Perlin::<f32>::new(cubic_curve));
        let mut result = Fractal::new_billowing_with_config(
            source,
            NoiseConfig {
                octaves: 1,
                ..Default::default()
            },
        );

        // 1d testing
        {
            let sample1 = result.sample_1d(0.1);
            let sample2 = result.sample_1d(0.11);

            assert!((sample2 - sample1).abs() < 0.1);
        }

        // 2d testing
        {
            let sample1 = result.sample_2d(0.1, 0.1);
            let sample2 = result.sample_2d(0.11, 0.11);

            assert!((sample2 - sample1).abs() < 0.1);
        }

        // 3d testing
        {
            let sample1 = result.sample_3d(0.1, 0.1, 0.1);
            let sample2 = result.sample_3d(0.11, 0.11, 0.11);

            assert!((sample2 - sample1).abs() < 0.1);
        }
    }

    #[test]
    fn ridged_multi_is_smooth() {
        use crate::{math::cubic_curve, source};
        let source = Box::new(source::Perlin::new(cubic_curve));
        let mut result = Fractal::<f32>::new_ridged_multi_with_config(
            source,
            NoiseConfig {
                octaves: 1,
                ..Default::default()
            },
            0.9,
            1.0,
        );

        // 1d testing
        {
            let samples_1d = [0.1, 0.14, 0.2, 0.23];
            let mut value_total = [0.0; 4];
            for (i, x) in samples_1d.iter().enumerate() {
                value_total[i] = result.sample_1d(*x);
            }
            let diffs = value_total.windows(2).map(|w| w[1] - w[0]);
            let sum: f32 = diffs.clone().sum();
            let count = (diffs.count() + 1) as f32;
            let avg_diff = (sum / count).abs();

            assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
        }

        // 2d testing
        {
            let samples_x = [0.4, 0.43, 0.43, 0.45];
            let samples_y = [0.5, 0.5, 0.55, 0.55];
            let mut value_total = [0.0; 4];
            for (i, (x, y)) in samples_x.iter().zip(samples_y).enumerate() {
                value_total[i] += result.sample_2d(*x, y);
            }
            let diffs = value_total.windows(2).map(|w| w[1] - w[0]);
            let sum: f32 = diffs.clone().sum();
            let count = (diffs.count() + 1) as f32;
            let avg_diff = (sum / count).abs();

            assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
        }

        // 3d testing
        {
            let samples_x = [0.4, 0.6, 0.5, 0.5];
            let samples_y = [0.5, 0.5, 0.4, 0.6];
            let samples_z = [0.4, 0.5, 0.5, 0.6];
            let mut value_total = [0.0; 4];
            for (i, ((x, y), z)) in samples_x.iter().zip(samples_y).zip(samples_z).enumerate() {
                value_total[i] += result.sample_3d(*x, y, z);
            }
            let diffs = value_total.windows(2).map(|w| w[1] - w[0]);
            let sum: f32 = diffs.clone().sum();
            let count = (diffs.count() + 1) as f32;
            let avg_diff = (sum / count).abs();

            assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
        }
    }

    #[test]
    fn ridged_multi_is_continuous() {
        use crate::{math::cubic_curve, source};
        let source = Box::new(source::Perlin::<f32>::new(cubic_curve));
        let mut result = Fractal::new_ridged_multi(source);

        // 1d testing
        {
            let sample1 = result.sample_1d(0.1);
            let sample2 = result.sample_1d(0.11);

            assert!((sample2 - sample1).abs() < 0.1);
        }

        // 2d testing
        {
            let sample1 = result.sample_2d(0.1, 0.1);
            let sample2 = result.sample_2d(0.11, 0.11);

            assert!((sample2 - sample1).abs() < 0.1);
        }

        // 3d testing
        {
            let sample1 = result.sample_3d(0.1, 0.1, 0.1);
            let sample2 = result.sample_3d(0.11, 0.11, 0.11);

            assert!((sample2 - sample1).abs() < 0.1);
        }
    }
}
