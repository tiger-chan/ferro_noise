macro_rules! perlin_details {
	($type: ty) => {
		mod details {
			use rand::Rng;
		
			pub type NoisePermutions = [usize; 512];
		
			// fn perlin_permutations() -> [i32; 512] {
			//     const SIZE: usize = 256;
			// 	const OUT_SIZE: usize = 512;
		
			// 	const PERMUTATIONS: [i32; SIZE] = [
			// 		151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30,
			// 		69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94,
			// 		252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171,
			// 		168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60,
			// 		211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1,
			// 		216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
			// 		164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
			// 		126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
			// 		213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39,
			// 		253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34,
			// 		242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49,
			// 		192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
			// 		138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
			// 	];
		
			// 	let mut p = [0_i32; OUT_SIZE];
			// 	for i in 0..SIZE {
			// 		p[i] = PERMUTATIONS[i];
			// 		p[i + SIZE] = PERMUTATIONS[i];
			// 	}
			// 	p
			// }
		
			pub fn perlin_permutation_seeded(seed: u64) -> NoisePermutions {
				use rand::{distributions::Uniform, rngs::StdRng, SeedableRng};
		
				const SIZE: usize = 256;
				const OUT_SIZE: usize = 512;
				let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
				let side = Uniform::new_inclusive(0, 255);
		
				let mut p: NoisePermutions = [0_usize; OUT_SIZE];
				for i in 0..SIZE {
					let v = rng.sample(side);
					p[i] = v;
					p[i + SIZE] = v;
				}
				p
			}
		
			pub const PERLIN_PERMUTATIONS: NoisePermutions = [
				151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30,
				69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94,
				252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171,
				168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60,
				211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1,
				216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
				164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
				126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
				213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39,
				253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34,
				242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49,
				192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
				138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
				151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30,
				69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94,
				252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171,
				168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60,
				211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1,
				216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
				164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
				126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
				213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39,
				253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34,
				242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49,
				192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
				138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
			];
		
			pub const GRADIENT_1D: [$type; 16] = [
				-1.000, -0.875, -0.750, -0.625, -0.500, -0.375, -0.250, -0.125, 0.125, 0.250, 0.375, 0.500,
				0.625, 0.75, 0.875, 1.000,
			];
		
			pub fn gradient_1d(hash: usize, _x: $type) -> $type
			{
				let h: usize = hash & 15; // Convert lo 4 bits of hash code
				GRADIENT_1D[h].into() // * x;
			}
		
			pub fn gradient_2d(hash: usize, x: $type, y: $type) -> $type
			{
				let h = hash & 7; // Convert lo 3 bits of hash code
				match h {
					0 => x,
					1 => x + y,
					2 => y,
					3 => -x + y,
					4 => -x,
					5 => -x - y,
					6 => -y,
					7 => x - y,
					_ => 0.0,
				}
			}
		
			fn gradient_3d_a(hash: usize, x: $type, y: $type, z: $type) -> $type
			{
				match hash & 15 {
					// 12 cube midpoints
					0 => x + z,
					1 => x + y,
					2 => y + z,
					3 => -x + y,
					4 => -x + z,
					5 => -x - y,
					6 => -y + z,
					7 => x - y,
					8 => x - z,
					9 => y - z,
					10 => -x - z,
					11 => -y - z,
					// 4 vertices of regular tetrahedron
					12 => x + y,
					13 => -x + y,
					14 => -y + z,
					15 => -y - z,
					// This can't happen
					_ => 0.0,
				}
			}
		
			#[allow(dead_code)]
			pub fn gradient_3d_b(hash: usize, x: $type, y: $type, z: $type) -> $type
			{
				// this seems like it would be slower to compute compared to version A (above).
				// https://mrl.nyu.edu/~perlin/noise/
				// Convert lo 4 bits of hash code into 12 gradient directions.
				let h = hash & 15;
				let u = if h < 8 { x } else { y };
				let v = if h < 4 {
					y
				} else if h == 12 || h == 14 {
					x
				} else {
					z
				};
		
				let f = if (h & 1) == 0 { u } else { -u };
				let g = if (h & 2) == 0 { v } else { -v };
				f + g
			}
		
			pub fn gradient_3d(hash: usize, x: $type, y: $type, z: $type) -> $type {
				gradient_3d_a(hash, x, y, z)
			}
		}		
	};
}

macro_rules! perlin {
	($type: ty) => {
		#[derive(Debug, Clone, Copy)]
		pub struct Perlin {
			perm: details::NoisePermutions,
			blender: Blender,
		}

		#[allow(dead_code)]
		impl Perlin {
			pub fn new(blender: Blender) -> Self {
				Perlin {
					perm: details::PERLIN_PERMUTATIONS.clone(),
					blender: blender,
				}
			}

			pub fn new_from_seed(blender: Blender, seed: u64) -> Self {
				Perlin {
					perm: details::perlin_permutation_seeded(seed),
					blender: blender,
				}
			}

			fn fade(&self, v: $type) -> $type {
				(self.blender)(v)
			}
		}

		impl Noise for Perlin {
			fn sample_1d(&mut self, x: $type) -> $type {
				const INDEX_MASK: usize = 255;
				let x0 = x.floor();
				let x1 = x0 + 1.0;

				let dx = x - x0;
				let u = self.fade(dx);

				let a = self.perm[x0 as usize & INDEX_MASK];
				let b = self.perm[x1 as usize & INDEX_MASK];

				let gx0 = details::gradient_1d(a, x0);
				let gx1 = details::gradient_1d(b, x1);

				let p0 = gx0 * (x - x0);
				let p1 = gx1 * (x - x1);
				lerp(p0, p1, u)
			}

			fn sample_2d(&mut self, x: $type, y: $type) -> $type {
				const INDEX_MASK: usize = 255;
				let x0 = x.floor();
				let y0 = y.floor();
				let xi = x0 as usize & INDEX_MASK;
				let yi = y0 as usize & INDEX_MASK;
				let x0 = x - x0;
				let y0 = y - y0;
				let x1 = x0 - 1.0;
				let y1 = y0 - 1.0;

				let aa = self.perm[xi] + yi;
				let ab = aa + 1;
				let ba = self.perm[xi + 1] + yi;
				let bb = ba + 1;

				let u = self.fade(x0);
				let v = self.fade(y0);

				let l1 = lerp(
					details::gradient_2d(self.perm[aa], x0, y0),
					details::gradient_2d(self.perm[ba], x1, y0),
					u,
				);
				let l2 = lerp(
					details::gradient_2d(self.perm[ab], x0, y1),
					details::gradient_2d(self.perm[bb], x1, y1),
					u,
				);

				let alpha = clamp((lerp(l1, l2, v) + 1.0) / 2.0, 0.0, 1.0);
				lerp(-1.0, 1.0, alpha)
			}

			fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type {
				const INDEX_MASK: usize = 255;
				// https://mrl.nyu.edu/~perlin/noise/
				// Find unit cube that contains point.
				let x0 = x.floor();
				let y0 = y.floor();
				let z0 = z.floor();
				let xi = x0 as usize & INDEX_MASK;
				let yi = y0 as usize & INDEX_MASK;
				let zi = z0 as usize & INDEX_MASK;

				//  Find relative x,y,z of point in cube.
				let x0 = x - x0;
				let y0 = y - y0;
				let z0 = z - z0;
				let x1 = x0 - 1.0;
				let y1 = y0 - 1.0;
				let z1 = z0 - 1.0;

				// Hash coordinates of the 8 cube corners
				let a = self.perm[xi] + yi;
				let aa = self.perm[a] + zi;
				let ab = self.perm[a + 1] + zi;
				let b = self.perm[xi + 1] + yi;
				let ba = self.perm[b] + zi;
				let bb = self.perm[b + 1] + zi;

				// Compute fade curves for each of x,y,z.
				let u = self.fade(x0);
				let v = self.fade(y0);
				let w = self.fade(z0);

				// And add blended results from 8 corners of cube
				let lu1 = lerp(
					details::gradient_3d(self.perm[aa], x0, y0, z0),
					details::gradient_3d(self.perm[ba], x1, y0, z0),
					u,
				);

				let lu2 = lerp(
					details::gradient_3d(self.perm[ab], x0, y1, z0),
					details::gradient_3d(self.perm[bb], x1, y1, z0),
					u,
				);

				let lu3 = lerp(
					details::gradient_3d(self.perm[aa + 1], x0, y0, z1),
					details::gradient_3d(self.perm[ba + 1], x1, y0, z1),
					u,
				);

				let lu4 = lerp(
					details::gradient_3d(self.perm[ab + 1], x0, y1, z1),
					details::gradient_3d(self.perm[bb + 1], x1, y1, z1),
					u,
				);

				let lv1 = lerp(lu1, lu2, v);
				let lv2 = lerp(lu3, lu4, v);

				let alpha = clamp((lerp(lv1, lv2, w) + 1.0) / 2.0, 0.0, 1.0);
				lerp(-1.0, 1.0, alpha)
			}
		}

		impl BoxNoise for Perlin {
			fn box_clone(&self) -> Box<dyn Noise + 'static> {
				Box::new(Self {
					perm: self.perm.clone(),
					blender: self.blender,
				})
			}
		}		
	};
}

pub(self) use super::{f32 as sf32, f64 as sf64};

pub mod f32 {
	use super::sf32::{ Blender, BoxNoise, Noise};
	use crate::math::f32::*;
	perlin_details!(f32);
	perlin!(f32);
}

pub mod f64 {
	use crate::math::f64::*;
	use super::sf64::{ Blender, BoxNoise, Noise};
	perlin_details!(f64);
	perlin!(f64);
}

#[cfg(test)]
mod tests {

	mod f32 {
		use crate::source::f32::{Noise ,Perlin};
		use crate::math::f32::*;

		#[test]
		fn perlin_tests() {
			let mut perlin = Perlin::new(quintic_curve);
			let result = perlin.sample_1d(0.0);
			assert_eq!(result, 0.0);
	
			let result = perlin.sample_1d(0.1);
			assert_eq!(result, -0.004689);
		}
	
		#[test]
		fn perlin_seeded_tests() {
			let mut perlin = Perlin::new_from_seed(quintic_curve, 12345);
			let result = perlin.sample_1d(0.0);
			assert_eq!(result, 0.0);
	
			let result = perlin.sample_1d(0.1);
			assert_eq!(result, 0.092529);
		}
	}
	
	mod f64 {
		use crate::source::f64::{Noise ,Perlin};
		use crate::math::f64::*;

		#[test]
		fn perlin_tests() {
			let mut perlin = Perlin::new(quintic_curve);
			let result = perlin.sample_1d(0.0);
			assert_eq!(result, 0.0);
	
			let result = perlin.sample_1d(0.1);
			assert_eq!(result, -0.004688999999999998);
		}
	
		#[test]
		fn perlin_seeded_tests() {
			let mut perlin = Perlin::new_from_seed(quintic_curve, 12345);
			let result = perlin.sample_1d(0.0);
			assert_eq!(result, 0.0);
	
			let result = perlin.sample_1d(0.1);
			assert_eq!(result, 0.09252900000000001);
		}
	}

}
