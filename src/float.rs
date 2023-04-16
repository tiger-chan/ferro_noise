use std::ops::{Add, Mul, Sub};

pub trait Floor {
    type Output;
    fn floor(self) -> Self::Output;
}

impl Floor for f32 {
    type Output = Self;
    fn floor(self) -> Self::Output {
        self.floor()
    }
}

impl Floor for f64 {
    type Output = Self;
    fn floor(self) -> Self::Output {
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

pub trait Float<T = Self>:
    Add<T, Output = T>
    + Mul<T, Output = T>
    + Sub<T, Output = T>
    + PartialEq
    + PartialOrd
    + Copy
    + Floor<Output = T>
    + AsIndex
    + From<f32>
{
}

impl Float for f32 {}

impl Float for f64 {}
