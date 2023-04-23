use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

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
    + From<f32>
    + From<u16>
    + Default
    + 'static
{
    fn abs(self) -> Self;
    fn as_index(self) -> usize;
    fn floor(self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn nearly_eq(self, r: Self) -> bool;
    fn sqrt(self) -> Self;

    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MAX: Self;
    const MIN: Self;
    const EPSILON: Self;
}

impl Float for f32 {
    fn abs(self) -> Self {
        self.abs()
    }

    fn as_index(self) -> usize {
        self as usize
    }

    fn floor(self) -> Self {
        self.floor()
    }

    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }

    fn nearly_eq(self, r: Self) -> bool {
        (r - self).abs() < Self::EPSILON
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;

    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    const EPSILON: Self = Self::EPSILON;
}

impl Float for f64 {
    fn abs(self) -> Self {
        self.abs()
    }

    fn as_index(self) -> usize {
        self as usize
    }

    fn floor(self) -> Self {
        self.floor()
    }

    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }

    fn nearly_eq(self, r: Self) -> bool {
        (r - self).abs() < Self::EPSILON
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;

    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    const EPSILON: Self = Self::EPSILON;
}
