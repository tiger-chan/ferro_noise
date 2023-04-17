use crate::{float::Float, source::Noise};

use super::Task;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PerlinConfig<T> {
    pub octaves: u16,
    pub lacunarity: T,
    pub persistance: T,
    pub frequency: T,
    pub amplitude: T,
}

impl<T: Float> Default for PerlinConfig<T> {
    fn default() -> Self {
        PerlinConfig {
            octaves: 6,
            lacunarity: T::from(2.0),
            persistance: T::from(0.5),
            frequency: T::from(1.0),
            amplitude: T::from(1.0),
        }
    }
}

pub struct Perlin<T: Float> {
    config: PerlinConfig<T>,
    noise: Box<dyn Noise<T>>,
}

impl<T: Float> Perlin<T> {
    #[allow(dead_code)]
    pub fn new(noise: Box<dyn Noise<T>>) -> Self {
        Self {
            config: PerlinConfig::default(),
            noise: noise,
        }
    }

    #[allow(dead_code)]
    pub fn new_with_config(noise: Box<dyn Noise<T>>, config: PerlinConfig<T>) -> Self {
        Self {
            config: config,
            noise: noise,
        }
    }
}

impl<T: Float> Task<T> for Perlin<T> {
    fn sample_1d(&mut self, x: T) -> T {
        let mut result = T::from(0.0);
        let mut amp = self.config.amplitude;
        let mut freq = self.config.frequency;

        let mut weight = T::from(0.0);

        for octave in 0..self.config.octaves {
            let o: T = T::from(octave);
            let x = x * freq + o;
            let tmp = self.noise.sample_1d(x) * amp;
            result += tmp;

            // used to normalize values generated.
            weight += amp;

            freq *= self.config.lacunarity;
            amp *= self.config.persistance;
        }
        result /= weight;

        result
    }

    fn sample_2d(&mut self, x: T, y: T) -> T {
        let mut result = T::from(0.0);
        let mut amp = self.config.amplitude;
        let mut freq = self.config.frequency;

        let mut weight = T::from(0.0);

        for octave in 0..self.config.octaves {
            let o: T = T::from(octave);
            let x = x * freq + o;
            let y = y * freq + o;
            let tmp = self.noise.sample_2d(x, y) * amp;
            result += tmp;

            // used to normalize values generated.
            weight += amp;

            freq *= self.config.lacunarity;
            amp *= self.config.persistance;
        }
        result /= weight;

        result
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        let mut result = T::from(0.0);
        let mut amp = self.config.amplitude;
        let mut freq = self.config.frequency;

        let mut weight = T::from(0.0);

        for octave in 0..self.config.octaves {
            let o: T = T::from(octave);
            let x = x * freq + o;
            let y = y * freq + o;
            let z = z * freq + o;
            let tmp = self.noise.sample_3d(x, y, z) * amp;
            result += tmp;

            // used to normalize values generated.
            weight += amp;

            freq *= self.config.lacunarity;
            amp *= self.config.persistance;
        }
        result /= weight;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_type_perlin_is_smooth() {
        use crate::{math::cubic_curve, source};
        let source = Box::new(source::Perlin::new(cubic_curve));
        let mut result = Box::new(Perlin::new_with_config(
            source,
            PerlinConfig {
                octaves: 1,
                ..Default::default()
            },
        ));

        // 1d testing
        {
            let samples_1d = [0.1, 0.2, 0.23, 0.14];
            let mut value_total = 0.0;
            for x in samples_1d.iter() {
                value_total = result.sample_1d(*x);
            }
            value_total /= samples_1d.len() as f64;

            assert!(value_total < 0.1);
        }

        // 2d testing
        {
            let samples_x = [0.4, 0.6, 0.5, 0.5];
            let samples_y = [0.5, 0.5, 0.4, 0.6];
            let mut value_total = 0.0;
            for (x, y) in samples_x.iter().zip(samples_y) {
                value_total = result.sample_2d(*x, y);
            }
            value_total /= samples_x.len() as f64;

            assert!(value_total < 0.1);
        }

        // 3d testing
        {
            let samples_x = [0.4, 0.6, 0.5, 0.5];
            let samples_y = [0.5, 0.5, 0.4, 0.6];
            let samples_z = [0.4, 0.5, 0.5, 0.6];
            let mut value_total = 0.0;
            for ((x, y), z) in samples_x.iter().zip(samples_y).zip(samples_z) {
                value_total = result.sample_3d(*x, y, z);
            }
            value_total /= samples_x.len() as f64;

            assert!(value_total < 0.1);
        }
    }

    #[test]
    fn task_type_perlin_is_continuous() {
        use crate::{math::cubic_curve, source};
        let source = Box::new(source::Perlin::new(cubic_curve));
        let mut result = Box::new(Perlin::new_with_config(
            source,
            PerlinConfig {
                octaves: 1,
                ..Default::default()
            },
        ));

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
