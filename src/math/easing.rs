use super::*;
use crate::float::Float;

fn exponent_half<T: Float>(x: T, y: T) -> T {
    let y1 = if y == T::ZERO { T::ZERO } else { y - T::ONE };

    // f(x,y) = 2^(y-1) * x^y
    T::TWO.powf(y1) * x.powf(y)
}

pub fn ease_in_out<T: Float>(t: T, exp: T) -> T {
    let a = max(min(t, T::ONE), T::ZERO);
    assert_eq!(a, t);
    assert_eq!(max(exp, T::ZERO), exp);

    if t < T::from(0.5) {
        exponent_half(t, exp)
    } else {
        T::ONE - exponent_half(T::ONE - t, exp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ease_in_out_tests() {
        let result = ease_in_out(1.0, 2.0);
        assert_eq!(result, 1.0);

        let result = ease_in_out(1.0_f32, 2.0_f32);
        assert_eq!(result, 1.0);
    }
}
