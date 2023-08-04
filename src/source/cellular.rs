const PRIME_X: u64 = 2053;
const PRIME_Y: u64 = 7177;
const PRIME_Z: u64 = 4943;
const XOR_X: u64 = 13844389427;
const XOR_Y: u64 = 24384685848;
const XOR_Z: u64 = 9413284231;

macro_rules! cellular {
    ($T: ty) => {
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct Cellular {
            spacing: [$T; 3],
            seed: u64,
        }

        impl Cellular {
            #[allow(dead_code)]
            pub fn new(spacing: [$T; 3]) -> Self {
                Cellular::new_seeded(spacing, 0)
            }

            pub fn new_seeded(spacing: [$T; 3], seed: u64) -> Self {
                Self {
                    spacing,
                    seed: seed,
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
        }

        impl Default for Cellular {
            fn default() -> Self {
                Cellular::new_seeded([1.0, 1.0, 1.0], 0)
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
                    let dist = (nfx - fx).abs();
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
                        let dist = (dfx * dfx + dfy * dfy).sqrt();
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
                            let dist = (dfx * dfx + dfy * dfy + dfz * dfz).sqrt();
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
    use super::{PRIME_X, PRIME_Y, PRIME_Z, XOR_X, XOR_Y, XOR_Z};
    use crate::rng::PCG;
    use crate::source::f32::Noise;

    cellular!(f32);

    #[cfg(test)]
    mod test {
        use super::{Cellular, Noise};

        #[test]
        fn work() {
            let mut cellular = Cellular::new([10.0, 10.0, 10.0]);
            let x = cellular.sample_2d(5.0, 5.0);
            assert_eq!(x, -4.0);
        }
    }
}

pub mod f64 {
    use super::{PRIME_X, PRIME_Y, PRIME_Z, XOR_X, XOR_Y, XOR_Z};
    use crate::rng::PCG;
    use crate::source::f64::Noise;

    cellular!(f64);
}
