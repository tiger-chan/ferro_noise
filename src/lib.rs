pub mod algorithm;
mod perlin_noise;
mod float;

mod noise;

pub use float::{Float, Floor, AsIndex};
pub use noise::{Noise1D, Noise2D, Noise3D};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
