macro_rules! exponent_half {
	($type: ty) => {
		fn exponent_half(x: $type, y: $type) -> $type {
			let y1: $type = if y == 0.0 { 0.0 } else { y - 1.0 };
			const TWO: $type = 2.0;
			// f(x,y) = 2^(y-1) * x^y
			TWO.powf(y1) * x.powf(y)
		}
	};
}

macro_rules! ease_in_out {
	($type: ty) => {
		pub fn ease_in_out(t: $type, exp: $type) -> $type {
			let a = max(min(t, 1.0), 0.0);
			assert_eq!(a, t);
			assert_eq!(max(exp, 0.0), exp);
		
			if t < 0.5 {
				exponent_half(t, exp)
			} else {
				1.0 - exponent_half(1.0 - t, exp)
			}
		}
	};
}

pub mod f32 {
	use crate::math::algorithm::f32::*;
	exponent_half!(f32);
	ease_in_out!(f32);
}

pub mod f64 {
	use crate::math::algorithm::f64::*;
	exponent_half!(f64);
	ease_in_out!(f64);
}


#[cfg(test)]
mod tests {
	mod f32 {
		use crate::math::easing::f32::*;

		#[test]
		fn ease_in_out_tests() {
			let result = ease_in_out(1.0, 2.0);
			assert_eq!(result, 1.0);
	
			let result = ease_in_out(1.0, 2.0);
			assert_eq!(result, 1.0);
		}
	}
	
	mod f64 {
		use crate::math::easing::f64::*;

		#[test]
		fn ease_in_out_tests() {
			let result = ease_in_out(1.0, 2.0);
			assert_eq!(result, 1.0);
	
			let result = ease_in_out(1.0, 2.0);
			assert_eq!(result, 1.0);
		}
	}
}
