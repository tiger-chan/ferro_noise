use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

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
    + Mul<T, Output = T>
    + MulAssign<T>
    + Div<T, Output = T>
    + DivAssign<T>
    + Sub<T, Output = T>
    + Neg<Output = T>
    + PartialEq
    + PartialOrd
    + Copy
    + Floor
    + AsIndex
    + NearlyEq
    + From<f32>
    + From<u16>
    + Default
{
}

impl Float for f32 {}

impl Float for f64 {}
