mod billowing;
mod builder;
mod fbm;
mod ridged_multi;

pub use builder::FractalBuilder;

use crate::{float::Float, source::Noise};

use self::ridged_multi::PreCalc;

use super::Task;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FractalType {
    /// Billowing
    Billowing,
    /// Fractal Brownian Motion
    Brownian,
    /// Ridged Multi Fractal
    RidgedMulti,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
struct ScaleBias<T> {
    scale: T,
    bias: T,
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

#[derive(Debug)]
pub struct Fractal<T: Float> {
    config: NoiseConfig<T>,
    noise: Box<dyn Noise<T>>,
    fractal: FractalType,
    /// Only used in Ridged Multi
    pre_calc: PreCalc<T>,
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
    use crate::task::fractal::builder::FractalBuilder;

    use super::*;

    #[test]
    fn fractal_fbm_is_smooth() {
        use crate::{math::cubic_curve, source};
        let source = Box::new(source::Perlin::new(cubic_curve));
        let mut result = FractalBuilder::<f32>::new()
            .fractal(FractalType::Brownian)
            .source(source)
            .octaves(1)
            .build();

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
        let mut result = FractalBuilder::<f32>::new()
            .fractal(FractalType::Brownian)
            .source(source)
            .octaves(1)
            .build();

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
        let mut result = FractalBuilder::<f32>::new()
            .fractal(FractalType::Billowing)
            .source(source)
            .octaves(1)
            .build();

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
        let mut result = FractalBuilder::<f32>::new()
            .fractal(FractalType::Billowing)
            .source(source)
            .octaves(1)
            .build();

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
        let mut result = FractalBuilder::<f32>::new()
            .fractal(FractalType::RidgedMulti)
            .source(source)
            .octaves(1)
            .build();

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
        let mut result = FractalBuilder::<f32>::new()
            .fractal(FractalType::RidgedMulti)
            .source(source)
            .octaves(1)
            .build();

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
