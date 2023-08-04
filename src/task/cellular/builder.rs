macro_rules! cellular_builder {
    ($T: ty) => {
        pub struct CellularBuilder {
            spacing: [$T; 3],
            seed: u64,
            dist: source::Distance,
        }

        impl Default for CellularBuilder {
            fn default() -> Self {
                Self {
                    spacing: [1.0, 1.0, 1.0],
                    seed: 0,
                    dist: source::Distance::Euclidean,
                }
            }
        }

        #[allow(dead_code)]
        impl CellularBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn build(&self) -> Cellular {
                Cellular {
                    noise: source::Cellular::new_seeded(self.spacing, self.dist, self.seed),
                }
            }

            pub fn distance(&mut self, d: source::Distance) -> &mut Self {
                self.dist = d;
                self
            }

            pub fn spacing(&mut self, s: [$T; 3]) -> &mut Self {
                self.spacing = s;
                self
            }

            pub fn seed(&mut self, s: u64) -> &mut Self {
                self.seed = s;
                self
            }
        }
    };
}

pub mod f32 {
    use crate::source::f32 as source;
    use crate::task::f32::Cellular;
    cellular_builder!(f32);
}

pub mod f64 {
    use crate::source::f64 as source;
    use crate::task::f64::Cellular;

    cellular_builder!(f64);
}
