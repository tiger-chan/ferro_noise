use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

pub trait Floor {
    fn floor(self) -> Self;
}

impl Floor for f32 {
    fn floor(self) -> Self {
        self.floor()
    }
}

impl Floor for f64 {
    fn floor(self) -> Self {
        self.floor()
    }
}

pub trait PowF {
    fn powf(self, n: Self) -> Self;
}

impl PowF for f32 {
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }
}

impl PowF for f64 {
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }
}

pub trait AsIndex {
    fn as_index(self) -> usize;
}

impl AsIndex for f32 {
    fn as_index(self) -> usize {
        self as usize
    }
}

impl AsIndex for f64 {
    fn as_index(self) -> usize {
        self as usize
    }
}

pub trait NearlyEq {
    fn nearly_eq(self, r: Self) -> bool;
}

impl NearlyEq for f32 {
    fn nearly_eq(self, r: Self) -> bool {
        (r - self).abs() < std::f32::EPSILON
    }
}

impl NearlyEq for f64 {
    fn nearly_eq(self, r: Self) -> bool {
        (r - self).abs() < std::f64::EPSILON
    }
}

pub trait Float<T = Self>:
    Add<T, Output = T>
    + AddAssign<T>
    + Debug
    + Div<T, Output = T>
    + DivAssign<T>
    + Mul<T, Output = T>
    + MulAssign<T>
    + Sub<T, Output = T>
    + Neg<Output = T>
    + PartialEq
    + PartialOrd
    + Copy
    + Floor
    + AsIndex
    + PowF
    + NearlyEq
    + From<f32>
    + From<u16>
    + Default
{
}

impl Float for f32 {}

impl Float for f64 {}
