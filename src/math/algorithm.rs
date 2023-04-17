use std::ops::{Add, Mul, Sub};

pub fn min<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a < b {
        a
    } else {
        b
    }
}

pub fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

/// Linearly interpolate between two values by a given alpha value.
///
/// # Arguments
///
/// * `v0`: The starting value to interpolate from
/// * `v1`: The ending value to interpolate to
/// * `alpha`: The alpha value, typically in the range [0, 1]
///
/// # Examples
///
/// ```
/// extern crate ferro_noise;
/// use ferro_noise::algorithm::{lerp};
///
/// let result = lerp(0.0, 1.0, 0.4);
/// assert_eq!(result, 0.4);
/// ```
pub fn lerp<T, A>(v0: T, v1: T, alpha: A) -> T
where
    T: Mul<A, Output = T> + Add<Output = T>,
    A: From<f32> + Sub<A, Output = A> + Copy,
{
    let beta: A = 1.0.into();
    let va0 = v0 * (beta - alpha);
    let va1 = v1 * alpha;
    va0 + va1
}

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
/// use ferro_noise::algorithm::{clamp};
///
/// let result = clamp(6.0, 1.0, 5.0);
/// assert_eq!(result, 5.0);
/// ```
pub fn clamp<T>(v: T, min: T, max: T) -> T
where
    T: PartialOrd + PartialEq + Copy,
{
    if v > max {
        max
    } else if v < min {
        min
    } else {
        v
    }
}

/// Evaluates a cubic curve at a given time `t`, where `t` is typically in the range [0, 1].
///
/// # Examples
///
/// ```
/// extern crate ferro_noise;
/// use ferro_noise::algorithm::cubic_curve;
///
/// let result = cubic_curve(2.0);
/// assert_eq!(result, -4.0);
/// ```
///
/// # Notes
///
/// This cubic curve function uses the formula `3t^2 − 2t^3`, where `t` is typically in the
/// range [0, 1]. The function returns `0` at `t = 1.5` and `-4` at `t = 2`.
pub fn cubic_curve<T>(t: T) -> T
where
    T: From<i32> + Mul<T, Output = T> + Sub<T, Output = T> + Copy,
{
    let two: T = 2.into();
    let three: T = 3.into();
    // 3t^2 − 2t^3
    (t * t) * (three - (two * t))
}

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
/// use ferro_noise::algorithm::quintic_curve;
///
/// let result = quintic_curve(1);
/// assert_eq!(result, 1);
/// ```
pub fn quintic_curve<T>(t: T) -> T
where
    T: From<i16> + Mul<T, Output = T> + Sub<T, Output = T> + Add<T, Output = T> + Copy,
{
    let six: T = T::from(6);
    let ten: T = T::from(10);
    let fifteen: T = T::from(15);
    // 6t^5 - 15t^4 + 10t^3
    return t * t * t * (t * (t * six - fifteen) + ten);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_tests() {
        let result = lerp(0.0, 1.0, 0.4);
        assert_eq!(result, 0.4);

        let result = lerp(0f32, 10f32, 0.4f32);
        assert_eq!(result, 4f32);
    }

    #[test]
    fn clamp_tests() {
        let result = clamp(0, 1, 5);
        assert_eq!(result, 1);

        let result = clamp(2, 1, 5);
        assert_eq!(result, 2);

        let result = clamp(6, 1, 5);
        assert_eq!(result, 5);

        let result = clamp(0.0, 1.0, 5.0);
        assert_eq!(result, 1.0);

        let result = clamp(2.0, 1.0, 5.0);
        assert_eq!(result, 2.0);

        let result = clamp(6.0, 1.0, 5.0);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn cubic_curve_tests() {
        let result = cubic_curve(0);
        assert_eq!(result, 0);

        let result = cubic_curve(1);
        assert_eq!(result, 1);

        let result = cubic_curve(2);
        assert_eq!(result, -4);

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
        let result = quintic_curve(0);
        assert_eq!(result, 0);

        let result = quintic_curve(1);
        assert_eq!(result, 1);

        let result = quintic_curve(2);
        assert_eq!(result, 32);

        let result = quintic_curve(0.0_f32);
        assert_eq!(result, 0.0);

        let result = quintic_curve(1.0);
        assert_eq!(result, 1.0);

        let result = quintic_curve(2.0);
        assert_eq!(result, 32.0);
    }
}
