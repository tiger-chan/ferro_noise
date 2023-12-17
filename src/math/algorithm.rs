/// Performs floor division of two integers.
///
/// The `floor_div` function takes two integers, `dividend` and `divisor`,
/// and returns the result of the floor division of `dividend` by `divisor`.
/// Floor division rounds the quotient towards negative infinity, i.e., the result
/// will be the largest integer that is less than or equal to the exact quotient.
///
/// # Panics
///
/// The function will panic if the `divisor` is zero, as dividing by zero is undefined.
///
/// # Arguments
///
/// * `dividend`: An `i32` representing the numerator of the division operation.
/// * `divisor`: An `i32` representing the denominator of the division operation.
///
/// # Returns
///
/// The result of the floor division as an `i32`.
///
/// # Safety
///
/// This function is safe to use with valid inputs, but it is the caller's responsibility
/// to ensure that the inputs do not lead to arithmetic overflow or undefined behavior.
///
/// # Examples
///
/// ```
/// extern crate ferro_noise;
/// use ferro_noise::math::FloorDiv;
/// let result = 10.floor_div(3);
/// assert_eq!(result, 3);
///
/// let result = (-10).floor_div(3);
/// assert_eq!(result, -4);
///
/// let result = 15.floor_div(-4);
/// assert_eq!(result, -4);
/// ```
pub trait FloorDiv: Sized + PartialOrd + Ord + Eq + Copy {
    fn floor_div(&self, divisor: Self) -> Self;
}

macro_rules! impl_floor_integer_div {
    ($T: ty, $test_mod: ident) => {
        impl FloorDiv for $T {
            fn floor_div(&self, divisor: Self) -> Self {
                assert!(divisor != 0, "Division by zero is not allowed.");
                let negative = ((self ^ divisor) < 0) as Self;
                let has_remainder = ((self % divisor) != 0) as Self;
                (*self / divisor) + -(has_remainder * negative)
            }
        }

        #[cfg(test)]
        mod $test_mod {
            use crate::math::FloorDiv;

            #[test]
            fn simple_div() {
                let result = 10.floor_div(3);
                assert_eq!(result, 3);
            }

            #[test]
            fn negative_simple_div() {
                let result = (-10).floor_div(3);
                assert_eq!(result, -4);
            }

            #[test]
            fn negative_divisor() {
                let result = 15.floor_div(-4);
                assert_eq!(result, -4);
            }
        }
    };
}

macro_rules! impl_floor_uint_div {
    ($T: ty, $test_mod: ident) => {
        impl FloorDiv for $T {
            fn floor_div(&self, divisor: Self) -> Self {
                assert!(divisor != 0, "Division by zero is not allowed.");
                (*self / divisor)
            }
        }

        #[cfg(test)]
        mod $test_mod {
            use crate::math::FloorDiv;

            #[test]
            fn simple_div() {
                let result = 10.floor_div(3);
                assert_eq!(result, 3);
            }

            #[test]
            fn negative_simple_div() {
                let result = (-10).floor_div(3);
                assert_eq!(result, -4);
            }

            #[test]
            fn negative_divisor() {
                let result = 15.floor_div(-4);
                assert_eq!(result, -4);
            }
        }
    };
}

impl_floor_integer_div!(i8, i8_tests);
impl_floor_integer_div!(i16, i16_tests);
impl_floor_integer_div!(i32, i32_tests);
impl_floor_integer_div!(i64, i64_tests);
impl_floor_integer_div!(i128, i128_tests);
impl_floor_integer_div!(isize, isize_tests);

impl_floor_uint_div!(u8, u8_tests);
impl_floor_uint_div!(u16, u16_tests);
impl_floor_uint_div!(u32, u32_tests);
impl_floor_uint_div!(u64, u64_tests);
impl_floor_uint_div!(u128, u128_tests);
impl_floor_uint_div!(usize, usize_tests);

macro_rules! clamp {
    ($type: ty) => {
        /// Clamps a value between a minimum and maximum value.
        ///
        /// # Arguments
        ///
        /// * `v`: The value to clamp
        /// * `min`: The minimum value
        /// * `max`: The maximum value
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate ferro_noise;
        /// use ferro_noise::math::f32::{clamp};
        ///
        /// let result = clamp(6.0, 1.0, 5.0);
        /// assert_eq!(result, 5.0);
        /// ```
        pub fn clamp(v: $type, min: $type, max: $type) -> $type {
            v.min(max).max(min)
        }
    };
}

macro_rules! cubic {
    ($type: ty) => {
        /// Evaluates a cubic curve at a given time `t`, where `t` is typically in the range [0, 1].
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate ferro_noise;
        /// use ferro_noise::math::f32::cubic_curve;
        ///
        /// let result = cubic_curve(2.0);
        /// assert_eq!(result, -4.0);
        /// ```
        ///
        /// # Notes
        ///
        /// This cubic curve function uses the formula `3t^2 − 2t^3`, where `t` is typically in the
        /// range [0, 1]. The function returns `0` at `t = 1.5` and `-4` at `t = 2`.
        pub fn cubic_curve(t: $type) -> $type {
            // https://en.wikipedia.org/wiki/Cubic_Hermite_spline
            // 3t^2 − 2t^3
            (t * t) * (3.0 - (2.0 * t))
        }
    };
}

macro_rules! lerp {
    ($type: ty) => {
        /// Linearly interpolate between two values by a given alpha value.
        ///
        /// # Arguments
        ///
        /// * `a`: The starting value to interpolate from
        /// * `b`: The ending value to interpolate to
        /// * `f`: The alpha value, typically in the range [0, 1]
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate ferro_noise;
        /// use ferro_noise::math::f32::lerp;
        ///
        /// let result = lerp(0.0, 1.0, 0.4);
        /// assert_eq!(result, 0.4);
        /// ```
        pub fn lerp(a: $type, b: $type, f: $type) -> $type {
            a * (1.0 - f) + f * b
        }
    };
}

macro_rules! linear {
    ($type: ty) => {
        pub fn linear_curve(t: $type) -> $type {
            t
        }
    };
}

macro_rules! max {
    ($type: ty) => {
        pub fn max(a: $type, b: $type) -> $type {
            a.max(b)
        }
    };
}

macro_rules! min {
    ($type: ty) => {
        pub fn min(a: $type, b: $type) -> $type {
            a.min(b)
        }
    };
}

macro_rules! nearly_eq {
    ($type: ty) => {
        /// Simple check for EPSILON difference to determine equality
        pub fn nearly_eq(a: $type, b: $type) -> bool {
            (a - b).abs() < <$type>::EPSILON
        }
    };
}

macro_rules! quintic {
    ($type: ty) => {
        /// Computes a quintic curve value for the given input value `t`.
        ///
        /// # Arguments
        ///
        /// * `t` - The input value for which to compute the quintic curve value.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate ferro_noise;
        /// use ferro_noise::math::f32::quintic_curve;
        ///
        /// let result = quintic_curve(1.0);
        /// assert_eq!(result, 1.0);
        /// ```
        pub fn quintic_curve(t: $type) -> $type {
            // https://mrl.nyu.edu/~perlin/noise/
            // 6t^5 - 15t^4 + 10t^3
            return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
        }
    };
}

pub mod f32 {
    clamp!(f32);
    cubic!(f32);
    lerp!(f32);
    linear!(f32);
    min!(f32);
    max!(f32);
    nearly_eq!(f32);
    quintic!(f32);
}

pub mod f64 {
    clamp!(f64);
    cubic!(f64);
    lerp!(f64);
    linear!(f64);
    min!(f64);
    max!(f64);
    nearly_eq!(f64);
    quintic!(f64);
}

#[cfg(test)]
mod tests {

    mod f32 {
        use crate::math::f32::*;
        #[test]
        fn lerp_tests() {
            let result = lerp(0.0, 1.0, 0.4);
            assert_eq!(result, 0.4);

            let result = lerp(0.0, 10.0, 0.4);
            assert_eq!(result, 4.0);
        }

        #[test]
        fn clamp_tests() {
            let result = clamp(0.0, 1.0, 5.0);
            assert_eq!(result, 1.0);

            let result = clamp(2.0, 1.0, 5.0);
            assert_eq!(result, 2.0);

            let result = clamp(6.0, 1.0, 5.0);
            assert_eq!(result, 5.0);
        }

        #[test]
        fn cubic_curve_tests() {
            let result = cubic_curve(0.0);
            assert_eq!(result, 0.0);

            let result = cubic_curve(1.0);
            assert_eq!(result, 1.0);

            let result = cubic_curve(1.5);
            assert_eq!(result, 0.0);

            let result = cubic_curve(2.0);
            assert_eq!(result, -4.0);
        }

        #[test]
        fn quintic_curve_tests() {
            let result = quintic_curve(0.0);
            assert_eq!(result, 0.0);

            let result = quintic_curve(1.0);
            assert_eq!(result, 1.0);

            let result = quintic_curve(2.0);
            assert_eq!(result, 32.0);
        }
    }

    mod f64 {
        use crate::math::f64::*;
        #[test]
        fn lerp_tests() {
            let result = lerp(0.0, 1.0, 0.4);
            assert_eq!(result, 0.4);

            let result = lerp(0.0, 10.0, 0.4);
            assert_eq!(result, 4.0);
        }

        #[test]
        fn clamp_tests() {
            let result = clamp(0.0, 1.0, 5.0);
            assert_eq!(result, 1.0);

            let result = clamp(2.0, 1.0, 5.0);
            assert_eq!(result, 2.0);

            let result = clamp(6.0, 1.0, 5.0);
            assert_eq!(result, 5.0);
        }

        #[test]
        fn cubic_curve_tests() {
            let result = cubic_curve(0.0);
            assert_eq!(result, 0.0);

            let result = cubic_curve(1.0);
            assert_eq!(result, 1.0);

            let result = cubic_curve(1.5);
            assert_eq!(result, 0.0);

            let result = cubic_curve(2.0);
            assert_eq!(result, -4.0);
        }

        #[test]
        fn quintic_curve_tests() {
            let result = quintic_curve(0.0);
            assert_eq!(result, 0.0);

            let result = quintic_curve(1.0);
            assert_eq!(result, 1.0);

            let result = quintic_curve(2.0);
            assert_eq!(result, 32.0);
        }
    }
}
