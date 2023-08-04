#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Distance {
    #[default]
    #[cfg_attr(feature = "serde", serde(alias = "ed"))]
    Euclidean,
    #[cfg_attr(feature = "serde", serde(alias = "md"))]
    Manhattan,
}

const PRIME_X: u64 = 2053;
const PRIME_Y: u64 = 7177;
const PRIME_Z: u64 = 4943;
const XOR_X: u64 = 13844389427;
const XOR_Y: u64 = 24384685848;
const XOR_Z: u64 = 9413284231;

macro_rules! cellular {
    ($T: ty) => {
        fn manhattan_x(dx: $T) -> $T {
            dx.abs()
        }

        fn manhattan_xy(dx: $T, dy: $T) -> $T {
            (dx.abs() + dy.abs())
        }

        fn manhattan_xyz(dx: $T, dy: $T, dz: $T) -> $T {
            (dx.abs() + dy.abs() + dz.abs())
        }

        fn euclidean_x(dx: $T) -> $T {
            dx.abs()
        }

        fn euclidean_xy(dx: $T, dy: $T) -> $T {
            (dx * dx + dy * dy).sqrt()
        }

        fn euclidean_xyz(dx: $T, dy: $T, dz: $T) -> $T {
            (dx * dx + dy * dy + dz * dz).sqrt()
        }

        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct Cellular {
            spacing: [$T; 3],
            seed: u64,
            dist: Distance,
        }

        impl Cellular {
            #[allow(dead_code)]
            pub fn new(spacing: [$T; 3], dist: Distance) -> Self {
                Cellular::new_seeded(spacing, dist, 0)
            }

            pub fn new_seeded(spacing: [$T; 3], dist: Distance, seed: u64) -> Self {
                Self {
                    spacing,
                    seed: seed,
                    dist: dist,
                }
            }

            fn seed_from_x(&self, ix: i32) -> u64 {
                (ix as u64).wrapping_mul(PRIME_X).wrapping_add(self.seed) ^ XOR_X
            }

            fn seed_from_xy(&self, ix: i32, iy: i32) -> u64 {
                let seed = (ix as u64).wrapping_mul(PRIME_X).wrapping_add(self.seed) ^ XOR_X;
                seed.wrapping_add((iy as u64).wrapping_mul(PRIME_Y) ^ XOR_Y)
            }

            fn seed_from_xyz(&self, ix: i32, iy: i32, iz: i32) -> u64 {
                let seed = (ix as u64).wrapping_mul(PRIME_X).wrapping_add(self.seed) ^ XOR_X;
                let seed = seed.wrapping_add((iy as u64).wrapping_mul(PRIME_Y) ^ XOR_Y);
                seed.wrapping_add((iz as u64).wrapping_mul(PRIME_Z) ^ XOR_Z)
            }

            fn offset_x(&self, ix: i32) -> $T {
                let seed = self.seed_from_x(ix);
                let mut rng = PCG::new(seed);
                rng.next()
            }

            fn offset_xy(&self, ix: i32, iy: i32) -> ($T, $T) {
                let seed = self.seed_from_xy(ix, iy);
                let mut rng = PCG::new(seed);
                (rng.next(), rng.next())
            }

            fn offset_xyz(&self, ix: i32, iy: i32, iz: i32) -> ($T, $T, $T) {
                let seed = self.seed_from_xyz(ix, iy, iz);
                let mut rng = PCG::new(seed);
                (rng.next(), rng.next(), rng.next())
            }

            fn dist_x(&self, x: $T) -> $T {
                match self.dist {
                    Distance::Euclidean => euclidean_x(x),
                    Distance::Manhattan => manhattan_x(x),
                }
            }

            fn dist_xy(&self, x: $T, y: $T) -> $T {
                match self.dist {
                    Distance::Euclidean => euclidean_xy(x, y),
                    Distance::Manhattan => manhattan_xy(x, y),
                }
            }

            fn dist_xyz(&self, x: $T, y: $T, z: $T) -> $T {
                match self.dist {
                    Distance::Euclidean => euclidean_xyz(x, y, z),
                    Distance::Manhattan => manhattan_xyz(x, y, z),
                }
            }
        }

        impl Default for Cellular {
            fn default() -> Self {
                Cellular::new_seeded([1.0, 1.0, 1.0], Distance::Euclidean, 0)
            }
        }

        impl Noise for Cellular {
            /// Calculates the dot product of the x value scaled to the range [-1, 1].
            fn sample_1d(&mut self, x: $T) -> $T {
                let lx = x / self.spacing[0];
                let ix = lx.floor() as i32;
                let fx = lx.fract();

                let mut min_dist: $T = 1.0;
                for dx in -1..=1 {
                    let nfx = dx as $T + self.offset_x(ix + dx);
                    let dist = self.dist_x(nfx - fx);
                    min_dist = min_dist.min(dist);
                }

                min_dist * 2.0 - 1.0
            }

            /// Calculates the dot product of the x, y values scaled to the range [-1, 1].
            fn sample_2d(&mut self, x: $T, y: $T) -> $T {
                let lx = x / self.spacing[0];
                let ly = y / self.spacing[1];

                let ix = lx.floor() as i32;
                let iy = ly.floor() as i32;

                let fx = lx.fract();
                let fy = ly.fract();

                let mut min_dist: $T = 1.0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let (nx, ny) = self.offset_xy(ix + dx, iy + dy);
                        let dfx = (dx as $T + nx) - fx;
                        let dfy = (dy as $T + ny) - fy;
                        let dist = self.dist_xy(dfx, dfy);
                        min_dist = min_dist.min(dist);
                    }
                }

                min_dist * 2.0 - 1.0
            }

            /// Calculates the dot product of the x, y, and z values scaled to the range [-1, 1].
            fn sample_3d(&mut self, x: $T, y: $T, z: $T) -> $T {
                let lx = x / self.spacing[0];
                let ly = y / self.spacing[1];
                let lz = z / self.spacing[2];

                let ix = lx.floor() as i32;
                let iy = ly.floor() as i32;
                let iz = lz.floor() as i32;

                let fx = lx.fract();
                let fy = ly.fract();
                let fz = lz.fract();

                let mut min_dist: $T = 1.0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        for dz in -1..=1 {
                            let (nx, ny, nz) = self.offset_xyz(ix + dx, iy + dy, iz + dz);
                            let dfx = (dx as $T + nx) - fx;
                            let dfy = (dy as $T + ny) - fy;
                            let dfz = (dz as $T + nz) - fz;
                            let dist = self.dist_xyz(dfx, dfy, dfz);
                            min_dist = min_dist.min(dist);
                        }
                    }
                }

                min_dist * 2.0 - 1.0
            }
        }
    };
}

pub mod f32 {
    pub use super::Distance;
    use super::{PRIME_X, PRIME_Y, PRIME_Z, XOR_X, XOR_Y, XOR_Z};
    use crate::rng::PCG;
    use crate::source::f32::Noise;

    cellular!(f32);

    #[cfg(test)]
    mod test {
        use super::{Cellular, Distance, Noise};

        #[test]
        fn work() {
            let mut cellular = Cellular::new([10.0, 10.0, 10.0], Distance::Euclidean);
            let x = cellular.sample_2d(5.0, 5.0);
            assert_eq!(x, -4.0);
        }
    }
}

pub mod f64 {
    pub use super::Distance;
    use super::{PRIME_X, PRIME_Y, PRIME_Z, XOR_X, XOR_Y, XOR_Z};
    use crate::rng::PCG;
    use crate::source::f64::Noise;

    cellular!(f64);
}
