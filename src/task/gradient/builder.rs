macro_rules! gradient_builder_type {
    ($type: ty) => {
        pub struct GradientBuilder {
            s1: [$type; 3],
            s2: [$type; 3],
        }

        impl Default for GradientBuilder {
            fn default() -> Self {
                Self {
                    s1: [0.0; 3],
                    s2: [1.0, 1.0, 0.0],
                }
            }
        }

        #[allow(dead_code)]
        impl GradientBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn build(&self) -> Gradient {
                Gradient {
                    noise: source::Gradient::new(self.s1, self.s2),
                }
            }

            pub fn s1(&mut self, point: [$type; 3]) -> &mut Self {
                self.s1 = point;
                self
            }

            pub fn s2(&mut self, point: [$type; 3]) -> &mut Self {
                self.s2 = point;
                self
            }
        }
    };
}

pub mod f32 {
    use super::super::f32::Gradient;
    use crate::source::f32 as source;
    gradient_builder_type!(f32);
}

pub mod f64 {
    use super::super::f64::Gradient;
    use crate::source::f64 as source;
    gradient_builder_type!(f64);
}
