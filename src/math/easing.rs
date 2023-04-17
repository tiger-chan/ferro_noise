use super::*;
use crate::float::Float;

fn exponent_half<T: Float>(x: T, y: T) -> T {
    let y1 = if y == T::from(0) {
        T::from(0)
    } else {
        y - T::from(1.0)
    };

    // f(x,y) = 2^(y-1) * x^y
    T::from(2).powf(y1) * x.powf(y)
}

pub fn ease_in_out<T: Float>(t: T, exp: T) -> T {
    let a = max(min(t, T::from(1)), T::from(0));
    assert_eq!(a, t);
    assert_eq!(max(exp, T::from(0)), exp);

    if t < T::from(0.5) {
        exponent_half(t, exp)
    } else {
        T::from(1) - exponent_half(T::from(1.0) - t, exp)
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
