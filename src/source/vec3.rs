pub const MAX_COMPONENTS: usize = 3;

macro_rules! vec3 {
	($type: ty) => {
		use std::ops::{Div, Sub};

		#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
		pub struct Vec3 {
			pub x: $type,
			pub y: $type,
			pub z: $type,
		}
		
		impl Default for Vec3 {
			fn default() -> Self {
				Self {
					x: 0.0,
					y: 0.0,
					z: 0.0,
				}
			}
		}
		
		impl Sub for Vec3 {
			type Output = Vec3;
			fn sub(self, rhs: Self) -> Self::Output {
				Self {
					x: self.x - rhs.x,
					y: self.y - rhs.y,
					z: self.z - rhs.z,
				}
			}
		}
		
		impl Div<$type> for Vec3 {
			type Output = Vec3;
			fn div(self, rhs: $type) -> Self::Output {
				Self {
					x: self.x / rhs,
					y: self.y / rhs,
					z: self.z / rhs,
				}
			}
		}
		
		impl From<[$type; MAX_COMPONENTS]> for Vec3 {
			fn from(value: [$type; MAX_COMPONENTS]) -> Self {
				Self {
					x: value[0],
					y: value[1],
					z: value[2],
				}
			}
		}
		
		impl Vec3 {
			pub fn dot(self, rhs: Self) -> $type {
				self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
			}
		}
	};
}

pub mod f32 {
	pub use super::MAX_COMPONENTS;
	vec3!(f32);
}

pub mod f64 {
	pub use super::MAX_COMPONENTS;
	vec3!(f64);
}