// https://en.wikipedia.org/wiki/Xorshift

use super::rand::Random;

/// Utility function for splitting the input seed to produce a new pseudo-random seed.
///
/// The `split_max` function takes a mutable reference to a 64-bit unsigned integer `seed`.
/// It performs a series of bitwise and arithmetic operations on the `seed` to split it into
/// a new pseudo-random value. The function uses bitwise shifts, bitwise XOR operations, and
/// multiplications with constants to ensure good randomness properties.
///
/// The returned value is a new pseudo-random 64-bit unsigned integer that can be used as the
/// next seed value for a random number generator.
///
/// # Arguments
///
/// * `seed`: A mutable reference to a 64-bit unsigned integer that represents the current seed value.
fn split_max(seed: u64) -> u64 {
    let result = seed.wrapping_add(0x9E3779B97F4A7C15);
    let result = (result ^ (result >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    let result = (result ^ (result >> 27)).wrapping_mul(0x94D049BB133111EB);
    result ^ (result >> 31)
}

/// Xorshift random number generator.
///
/// The `Xorshift` struct represents the Xorshift random number generator,
/// which is a simple and efficient pseudo-random number generator.
/// It implements the Xorshift* variant, using a 64-bit seed.
///
/// # Examples
///
/// Creating a new `Xorshift` instance with a seed value:
///
/// ```
/// use ferro_noise::rng::Xorshift;
///
/// let mut rng = Xorshift::new(42);
/// let random_u32: u32 = rng.next();
/// println!("Random u32: {}", random_u32);
/// ```
///
/// Generating random values of different types:
///
/// ```
/// extern crate ferro_noise;
/// use ferro_noise::rng::Xorshift;
///
/// let mut rng = Xorshift::new(42);
/// let random_u32: u32 = rng.next();
/// let random_f64: f64 = rng.next();
/// let random_bool: bool = rng.next();
/// println!("Random u32: {}", random_u32);
/// println!("Random f64: {}", random_f64);
/// println!("Random bool: {}", random_bool);
/// ```
pub struct Xorshift {
    seed: u64,
}

impl Xorshift {
    /// Creates a new instance of the Xorshift random number generator with the specified seed value.
    ///
    /// The `new` function takes a 64-bit unsigned integer `seed` as input and returns a new instance
    /// of the `Xorshift` struct with the provided seed as the initial state.
    ///
    /// # Arguments
    ///
    /// * `seed`: A 64-bit unsigned integer that serves as the seed value for the random number generator.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate ferro_noise;
    /// use ferro_noise::rng::Xorshift;
    ///
    /// let mut rng = Xorshift::new(42);
    /// ```
    pub fn new(seed: u64) -> Self {
        Self {
            seed: split_max(seed),
        }
    }

    /// Generates the next 64-bit unsigned integer from the Xorshift sequence.
    ///
    /// The `next_u64` method generates the next pseudo-random 64-bit unsigned integer using the Xorshift*
    /// algorithm. The internal seed value is updated to produce subsequent random numbers.
    ///
    /// # Returns
    ///
    /// A 64-bit unsigned integer representing the next pseudo-random value in the Xorshift sequence.
    fn next_u64(&mut self) -> u64 {
        // https://en.wikipedia.org/wiki/Xorshift#xorshift*
        let mut x = self.seed;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.seed = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    /// Generates the next value of type `T` from the Xorshift sequence.
    ///
    /// This method relies on the `Random` trait implementation for type `T`.
    /// Ensure that the type `T` implements the `Random` trait for successful generation.
    ///
    /// # Example
    /// ```
    /// extern crate ferro_noise;
    /// use ferro_noise::rng::Xorshift;
    ///
    /// let mut rng = Xorshift::new(42);
    /// let random_u32: u32 = rng.next();
    /// println!("Random u32: {random_u32}");
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn next<T>(&mut self) -> T
    where
        Xorshift: Random<T>,
    {
        Random::<T>::next(self)
    }
}

impl Default for Xorshift {
    fn default() -> Self {
        Self {
            seed: split_max(382548641),
        }
    }
}

impl Random<i8> for Xorshift {
    fn next(&mut self) -> i8 {
        self.next_u64() as i8
    }
}

impl Random<i16> for Xorshift {
    fn next(&mut self) -> i16 {
        self.next_u64() as i16
    }
}

impl Random<i32> for Xorshift {
    fn next(&mut self) -> i32 {
        self.next_u64() as i32
    }
}

impl Random<i64> for Xorshift {
    fn next(&mut self) -> i64 {
        self.next_u64() as i64
    }
}

impl Random<u8> for Xorshift {
    fn next(&mut self) -> u8 {
        self.next_u64() as u8
    }
}

impl Random<u16> for Xorshift {
    fn next(&mut self) -> u16 {
        self.next_u64() as u16
    }
}

impl Random<u32> for Xorshift {
    fn next(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

impl Random<u64> for Xorshift {
    fn next(&mut self) -> u64 {
        self.next_u64()
    }
}

impl Random<f32> for Xorshift {
    fn next(&mut self) -> f32 {
        self.next_u64() as f32 / u64::MAX as f32
    }
}

impl Random<f64> for Xorshift {
    fn next(&mut self) -> f64 {
        self.next_u64() as f64 / u64::MAX as f64
    }
}

impl Random<bool> for Xorshift {
    fn next(&mut self) -> bool {
        self.next::<f64>() < 0.5
    }
}

#[cfg(test)]
mod tests {
    pub use super::Xorshift;
    pub use crate::rng::rand::Random;

    #[test]
    fn i8_generate_random() {
        let mut rand = Xorshift::default();
        let x = rand.next::<i8>();
        assert_eq!(x, -91);
    }

    #[test]
    fn i16_generate_random() {
        let mut rand = Xorshift::default();
        let x = rand.next::<i16>();
        assert_eq!(x, -15195);
    }

    #[test]
    fn i32_generate_random() {
        let mut rand = Xorshift::default();
        let x = rand.next::<i32>();
        assert_eq!(x, 1008649381);
    }

    #[test]
    fn i64_generate_random() {
        let mut rand = Xorshift::default();
        let x = rand.next::<i64>();
        assert_eq!(x, 1130587310604076197);
    }

    #[test]
    fn u8_generate_random() {
        let mut rand = Xorshift::default();
        let x = rand.next::<u8>();
        assert_eq!(x, 165);
    }

    #[test]
    fn u16_generate_random() {
        let mut rand = Xorshift::default();
        let x = rand.next::<u16>();
        assert_eq!(x, 50341);
    }

    #[test]
    fn u32_generate_random() {
        let mut rand = Xorshift::default();
        let x = rand.next::<u32>();
        assert_eq!(x, 1008649381);
    }

    #[test]
    fn u64_generate_random() {
        let mut rand = Xorshift::default();
        let x = rand.next::<u64>();
        assert_eq!(x, 1130587310604076197);
    }

    #[test]
    fn f32_generate_random() {
        let mut rand = Xorshift::default();
        let x = rand.next::<f32>();
        assert_eq!(x, 0.061289262);
    }

    #[test]
    fn f64_generate_random() {
        let mut rand = Xorshift::default();
        let x = rand.next::<f64>();
        assert_eq!(x, 0.06128926091707416);
    }

    #[test]
    fn bool_generate_random() {
        let mut rand = Xorshift::default();
        let x = rand.next::<bool>();
        assert!(x);
    }
}
