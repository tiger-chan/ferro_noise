// https://en.wikipedia.org/wiki/Permuted_congruential_generator

use super::rand::Random;

const MULTIPLYER: u64 = 6364136223846793005;
const INCREMENT: u64 = 1442659040888963407;

/// Permuted congruential generator (PCG).
///
/// The PCG is a random number generator that offers good statistical properties
/// and is efficient for most general-purpose randomization needs. This
/// implementation uses the PCG-XSH-RR (xorshift high (bits), random rotation)
/// variant.
///
/// # Examples
///
/// Creating a new PCG instance with a seed value:
///
/// ```
/// extern crate ferro_noise;
/// use ferro_noise::rng::PCG;
/// let mut rng = PCG::new(42);
/// let random_number: u64 = rng.next();
/// println!("Random number: {random_number}");
/// ```
///
/// Creating a new PCG instance with a custom increment value:
///
/// ```
/// extern crate ferro_noise;
/// use ferro_noise::rng::PCG;
/// let mut rng = PCG::new_with_increment(42, 54654654654654);
/// let random_number: u64 = rng.next();
/// println!("Random number: {random_number}");
/// ```
///
/// Creating a new PCG instance with custom multiplier and increment values:
///
/// ```
/// extern crate ferro_noise;
/// use ferro_noise::rng::PCG;
/// let mut rng = PCG::new_with_multiplier(42, 987654321, 123456789);
/// let random_number: u64 = rng.next();
/// println!("Random number: {random_number}");
/// ```
pub struct PCG {
    seed: u64,
    multiplier: u64,
    increment: u64,
}

impl PCG {
    /// Creates a new instance of the PCG with the provided seed.
    pub fn new(seed: u64) -> Self {
        PCG::new_with_multiplier(seed, MULTIPLYER, INCREMENT)
    }

    /// Creates a new instance of the PCG with the provided seed and increment value.
    #[allow(dead_code)]
    pub fn new_with_increment(seed: u64, increment: u64) -> Self {
        PCG::new_with_multiplier(seed, MULTIPLYER, increment)
    }

    /// Creates a new instance of the PCG with the provided seed, multiplier, and increment values.
    #[allow(dead_code)]
    pub fn new_with_multiplier(seed: u64, multiplier: u64, increment: u64) -> Self {
        Self {
            seed: seed.wrapping_add(increment),
            multiplier: multiplier,
            increment: increment,
        }
    }

    /// Generates the next 64-bit unsigned integer from the PCG sequence.
    fn next_u64(&mut self) -> u64 {
        // 5 rot count bits
        // 32 - 5 shift bits
        // (64 - (32 - 5)) / 2 xor shift bits

        let x = self.seed;
        self.seed = x.wrapping_mul(self.multiplier).wrapping_add(self.increment);

        let rot_count = (x >> 5) as u32;
        let x = x ^ x >> 18;
        x.rotate_left(rot_count)
    }

    /// Generates the next value of type `T` from the PCG sequence.
    ///
    /// This method relies on the `Random` trait implementation for type `T`.
    /// Ensure that the type `T` implements the `Random` trait for successful generation.
    ///
    /// # Example
    /// ```
    /// extern crate ferro_noise;
    /// use ferro_noise::rng::PCG;
    ///
    /// let mut rng = PCG::new(42);
    /// let random_u32: u32 = rng.next();
    /// println!("Random u32: {random_u32}");
    /// ```
    #[allow(dead_code)]
    pub fn next<T>(&mut self) -> T
    where
        PCG: Random<T>,
    {
        Random::<T>::next(self)
    }
}

impl Default for PCG {
    fn default() -> Self {
        Self {
            seed: 8967452310258,
            multiplier: MULTIPLYER,
            increment: INCREMENT,
        }
    }
}

impl Random<i8> for PCG {
    fn next(&mut self) -> i8 {
        self.next_u64() as i8
    }
}

impl Random<i16> for PCG {
    fn next(&mut self) -> i16 {
        self.next_u64() as i16
    }
}

impl Random<i32> for PCG {
    fn next(&mut self) -> i32 {
        self.next_u64() as i32
    }
}

impl Random<i64> for PCG {
    fn next(&mut self) -> i64 {
        self.next_u64() as i64
    }
}

impl Random<u8> for PCG {
    fn next(&mut self) -> u8 {
        self.next_u64() as u8
    }
}

impl Random<u16> for PCG {
    fn next(&mut self) -> u16 {
        self.next_u64() as u16
    }
}

impl Random<u32> for PCG {
    fn next(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

impl Random<u64> for PCG {
    fn next(&mut self) -> u64 {
        self.next_u64()
    }
}

impl Random<f32> for PCG {
    fn next(&mut self) -> f32 {
        self.next_u64() as f32 / u64::MAX as f32
    }
}

impl Random<f64> for PCG {
    fn next(&mut self) -> f64 {
        self.next_u64() as f64 / u64::MAX as f64
    }
}

impl Random<bool> for PCG {
    fn next(&mut self) -> bool {
        self.next::<f64>() < 0.5
    }
}

#[cfg(test)]
mod tests {
    pub use super::PCG;
    pub use crate::rng::rand::Random;

    #[test]
    fn i8_generate_random() {
        let mut rand = PCG::default();
        let x = rand.next::<i8>();
        assert_eq!(x, 4);
    }

    #[test]
    fn i16_generate_random() {
        let mut rand = PCG::default();
        let x = rand.next::<i16>();
        assert_eq!(x, 4);
    }

    #[test]
    fn i32_generate_random() {
        let mut rand = PCG::default();
        let x = rand.next::<i32>();
        assert_eq!(x, -1065353212);
    }

    #[test]
    fn i64_generate_random() {
        let mut rand = PCG::default();
        let x = rand.next::<i64>();
        assert_eq!(x, 1437742715042267140);
    }

    #[test]
    fn u8_generate_random() {
        let mut rand = PCG::default();
        let x = rand.next::<u8>();
        assert_eq!(x, 4);
    }

    #[test]
    fn u16_generate_random() {
        let mut rand = PCG::default();
        let x = rand.next::<u16>();
        assert_eq!(x, 4);
    }

    #[test]
    fn u32_generate_random() {
        let mut rand = PCG::default();
        let x = rand.next::<u32>();
        assert_eq!(x, 3229614084);
    }

    #[test]
    fn u64_generate_random() {
        let mut rand = PCG::default();
        let x = rand.next::<u64>();
        assert_eq!(x, 1437742715042267140);
    }

    #[test]
    fn f32_generate_random() {
        let mut rand = PCG::default();
        let x = rand.next::<f32>();
        assert_eq!(x, 0.07794019);
    }

    #[test]
    fn f64_generate_random() {
        let mut rand = PCG::default();
        let x = rand.next::<f64>();
        assert_eq!(x, 0.07794018875620168);
    }

    #[test]
    fn bool_generate_random() {
        let mut rand = PCG::default();
        let x = rand.next::<bool>();
        assert_eq!(x, true);
    }
}
