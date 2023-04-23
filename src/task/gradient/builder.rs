use crate::{float::Float, source};

use super::Gradient;

pub struct GradientBuilder<T: Float> {
    s1: [T; 3],
    s2: [T; 3],
}

impl<T: Float> Default for GradientBuilder<T> {
    fn default() -> Self {
        Self {
            s1: [T::ZERO; 3],
            s2: [T::ONE, T::ONE, T::ZERO],
        }
    }
}

#[allow(dead_code)]
impl<T: Float> GradientBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> Gradient<T> {
        Gradient {
            noise: source::Gradient::new(self.s1, self.s2),
        }
    }

    pub fn s1(&mut self, point: [T; 3]) -> &mut Self {
        self.s1 = point;
        self
    }

    pub fn s2(&mut self, point: [T; 3]) -> &mut Self {
        self.s2 = point;
        self
    }
}
