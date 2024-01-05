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

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
}

pub struct Vec2<T>(pub T, pub T);

impl<T: Copy + Clone> Clone for Vec2<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Copy> Copy for Vec2<T> {}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }
}

impl<T> std::ops::Add for Vec2<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Vec2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> std::ops::Sub for Vec2<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Vec2<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T> std::ops::Div<T> for Vec2<T>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Vec2<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl<T> std::ops::Mul<T> for Vec2<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Vec2<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

pub struct Vec3<T>(pub T, pub T, pub T);

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }
}

impl<T> std::ops::Add for Vec3<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Vec3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T> std::ops::Sub for Vec3<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Vec3<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T> std::ops::Div<T> for Vec3<T>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Vec3<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl<T> std::ops::Mul<T> for Vec3<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
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
            dx.abs() + dy.abs()
        }

        fn manhattan_xyz(dx: $T, dy: $T, dz: $T) -> $T {
            dx.abs() + dy.abs() + dz.abs()
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
                    seed,
                    dist,
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

            fn offset_xy(&self, ix: i32, iy: i32) -> Vec2 {
                let seed = self.seed_from_xy(ix, iy);
                let mut rng = PCG::new(seed);
                Vec2::new(rng.next(), rng.next())
            }

            fn offset_xyz(&self, ix: i32, iy: i32, iz: i32) -> Vec3 {
                let seed = self.seed_from_xyz(ix, iy, iz);
                let mut rng = PCG::new(seed);
                Vec3::new(rng.next(), rng.next(), rng.next())
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

                const CELLS: [i32; 2] = [-1, 1];

                let origin = self.offset_x(ix);
                let mut end = origin;
                let mut min_dist: $T = 10.0;
                for dx in CELLS {
                    let nfx = dx as $T + self.offset_x(ix + dx);
                    let dist = self.dist_x(nfx - fx);
                    if dist < min_dist {
                        end = nfx;
                        min_dist = dist;
                    }
                }

                let a = (fx - origin) / (end - origin);
                let a = if a > 0.5 { a - 0.5 } else { a };
                a * 2.0
            }

            /// Calculates the dot product of the x, y values scaled to the range [-1, 1].
            fn sample_2d(&mut self, x: $T, y: $T) -> $T {
                let lx = x / self.spacing[0];
                let ly = y / self.spacing[1];

                let ix = lx.floor() as i32;
                let iy = ly.floor() as i32;

                let fx = lx.fract();
                let fy = ly.fract();

                #[rustfmt::skip]
                        const CELLS: [(i32, i32); 9] = [
                            (-1, -1), (-1, 0), (-1, 1),
                            (0, -1), (0,0), (0, 1),
                            (1, -1), (1, 0), (1, 1),
                        ];

                // Define an array to store distances to the nearest and second-nearest points
                let mut distances: [$T; 2] = [10.0, 10.0];
                for (dx, dy) in CELLS {
                    let n = self.offset_xy(ix + dx, iy + dy);
                    let dfx = (dx as $T + n.0) - fx;
                    let dfy = (dy as $T + n.1) - fy;
                    let dist = self.dist_xy(dfx, dfy);

                    // Update distances array
                    if dist < distances[0] {
                        distances.swap(0, 1);
                        distances[0] = dist;
                    } else if dist < distances[1] {
                        distances[1] = dist;
                    }
                }

                // Calculate normalized difference between distances
                let noise_value = if distances[1] != 0.0 {
                    (distances[0] - distances[1]) / distances[1]
                } else {
                    0.0
                };

                // Return the normalized noise value
                noise_value
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

                let mut min_dist: $T = 10.0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        for dz in -1..=1 {
                            let n = self.offset_xyz(ix + dx, iy + dy, iz + dz);
                            let dfx = (dx as $T + n.0) - fx;
                            let dfy = (dy as $T + n.1) - fy;
                            let dfz = (dz as $T + n.2) - fz;
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

    type Vec2 = super::Vec2<f32>;
    type Vec3 = super::Vec3<f32>;

    cellular!(f32);

    #[cfg(test)]
    mod test {
        use super::{Cellular, Distance, Noise};

        #[test]
        fn work() {
            let mut cellular = Cellular::new([10.0, 10.0, 10.0], Distance::Euclidean);
            let x = cellular.sample_2d(5.0, 5.0);
            assert_eq!(x, -0.017567515);
        }
    }
}

pub mod f64 {
    pub use super::Distance;
    use super::{PRIME_X, PRIME_Y, PRIME_Z, XOR_X, XOR_Y, XOR_Z};
    use crate::rng::PCG;
    use crate::source::f64::Noise;

    type Vec2 = super::Vec2<f64>;
    type Vec3 = super::Vec3<f64>;

    cellular!(f64);
}
