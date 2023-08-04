mod builder;

macro_rules! cellular {
    ($T: ty) => {
        #[derive(Debug)]
        pub struct Cellular {
            pub(crate) noise: CellularNoise,
        }

        impl Task for Cellular {
            fn sample_1d(&mut self, x: $T) -> $T {
                self.noise.sample_1d(x)
            }

            fn sample_2d(&mut self, x: $T, y: $T) -> $T {
                self.noise.sample_2d(x, y)
            }

            fn sample_3d(&mut self, x: $T, y: $T, z: $T) -> $T {
                self.noise.sample_3d(x, y, z)
            }
        }
    };
}

pub mod f32 {
    pub use super::builder::f32::CellularBuilder;
    use crate::source::f32::{Cellular as CellularNoise, Noise};
    use crate::task::f32::Task;
    cellular!(f32);
}

pub mod f64 {
    pub use super::builder::f64::CellularBuilder;
    use crate::source::f64::{Cellular as CellularNoise, Noise};
    use crate::task::f64::Task;
    cellular!(f64);
}

#[cfg(test)]
mod tests {}
