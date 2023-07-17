mod billowing;
mod builder;
mod fbm;
mod ridged_multi;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum FractalType {
    /// Billowing
    Billowing,
    #[cfg_attr(feature = "serde", serde(alias = "fbm"))]
    #[default]
    /// Fractal Brownian Motion
    Brownian,
    /// Ridged Multi Fractal
    RidgedMulti,
}

macro_rules! scale_bias_type {
    ($type: ty) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
        pub struct ScaleBias {
            pub scale: $type,
            pub bias: $type,
        }
    };
}

macro_rules! noise_config_type {
    ($type: ty) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct NoiseConfig {
            pub octaves: u16,
            pub lacunarity: $type,
            pub gain: $type,
            pub frequency: $type,
            pub amplitude: $type,
        }

        impl Default for NoiseConfig {
            fn default() -> Self {
                NoiseConfig {
                    octaves: 6,
                    lacunarity: 2.0,
                    gain: 0.5,
                    frequency: 1.0,
                    amplitude: 1.0,
                }
            }
        }
    };
}

macro_rules! fractal_type {
    ($type: ty) => {
        #[derive(Debug)]
        pub struct Fractal {
            pub(crate) config: NoiseConfig,
            pub(crate) noise: Box<dyn Noise>,
            pub(crate) fractal: FractalType,
            /// Only used in Ridged Multi
            pub(crate) pre_calc: PreCalc,
        }

        impl Task for Fractal {
            fn sample_1d(&mut self, x: $type) -> $type {
                match self.fractal {
                    FractalType::Brownian => fbm::sample_1d(&self.config, self.noise.as_mut(), x),
                    FractalType::Billowing => {
                        billowing::sample_1d(&self.config, self.noise.as_mut(), x)
                    }
                    FractalType::RidgedMulti => ridged_multi::sample_1d(
                        &self.config,
                        &self.pre_calc,
                        self.noise.as_mut(),
                        x,
                    ),
                }
            }

            fn sample_2d(&mut self, x: $type, y: $type) -> $type {
                match self.fractal {
                    FractalType::Brownian => {
                        fbm::sample_2d(&self.config, self.noise.as_mut(), x, y)
                    }
                    FractalType::Billowing => {
                        billowing::sample_2d(&self.config, self.noise.as_mut(), x, y)
                    }
                    FractalType::RidgedMulti => ridged_multi::sample_2d(
                        &self.config,
                        &self.pre_calc,
                        self.noise.as_mut(),
                        x,
                        y,
                    ),
                }
            }

            fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type {
                match self.fractal {
                    FractalType::Brownian => {
                        fbm::sample_3d(&self.config, self.noise.as_mut(), x, y, z)
                    }
                    FractalType::Billowing => {
                        billowing::sample_3d(&self.config, self.noise.as_mut(), x, y, z)
                    }
                    FractalType::RidgedMulti => ridged_multi::sample_3d(
                        &self.config,
                        &self.pre_calc,
                        self.noise.as_mut(),
                        x,
                        y,
                        z,
                    ),
                }
            }
        }
    };
}

pub mod f32 {
    use super::billowing::f32 as billowing;
    pub use super::builder::f32::FractalBuilder;
    use super::fbm::f32 as fbm;
    use super::ridged_multi::f32 as ridged_multi;
    pub use super::FractalType;
    use crate::source::f32::Noise;
    use crate::task::f32::Task;
    use ridged_multi::PreCalc;
    scale_bias_type!(f32);
    noise_config_type!(f32);
    fractal_type!(f32);
}

pub mod f64 {
    use super::billowing::f64 as billowing;
    pub use super::builder::f64::FractalBuilder;
    use super::fbm::f64 as fbm;
    use super::ridged_multi::f64 as ridged_multi;
    pub use super::FractalType;
    use crate::source::f64::Noise;
    use crate::task::f64::Task;
    use ridged_multi::PreCalc;
    scale_bias_type!(f64);
    noise_config_type!(f64);
    fractal_type!(f64);
}

#[cfg(test)]
mod tests {
    mod f32 {
		use crate::task::f32::{FractalBuilder, FractalType, Task};
		use crate::source::f32::Perlin;
		use crate::math::f32::cubic_curve;
        
		#[test]
        fn fractal_fbm_is_smooth() {
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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

	mod f64 {
		use crate::task::f64::{FractalBuilder, FractalType, Task};
		use crate::source::f64::Perlin;
		use crate::math::f64::cubic_curve;
        
		#[test]
        fn fractal_fbm_is_smooth() {
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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
                let sum: f64 = diffs.clone().sum();
                let count = (diffs.count() + 1) as f64;
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
                let sum: f64 = diffs.clone().sum();
                let count = (diffs.count() + 1) as f64;
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
                let sum: f64 = diffs.clone().sum();
                let count = (diffs.count() + 1) as f64;
                let avg_diff = (sum / count).abs();

                assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
            }
        }

        #[test]
        fn fbm_is_continuous() {
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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
                let sum: f64 = diffs.clone().sum();
                let count = (diffs.count() + 1) as f64;
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
                let sum: f64 = diffs.clone().sum();
                let count = (diffs.count() + 1) as f64;
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
                let sum: f64 = diffs.clone().sum();
                let count = (diffs.count() + 1) as f64;
                let avg_diff = (sum / count).abs();

                assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
            }
        }

        #[test]
        fn billowing_is_continuous() {
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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
                let sum: f64 = diffs.clone().sum();
                let count = (diffs.count() + 1) as f64;
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
                let sum: f64 = diffs.clone().sum();
                let count = (diffs.count() + 1) as f64;
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
                let sum: f64 = diffs.clone().sum();
                let count = (diffs.count() + 1) as f64;
                let avg_diff = (sum / count).abs();

                assert!(avg_diff < 0.1, "a = {}, b = 0.1", avg_diff);
            }
        }

        #[test]
        fn ridged_multi_is_continuous() {
            let source = Box::new(Perlin::new(cubic_curve));
            let mut result = FractalBuilder::new()
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
}
