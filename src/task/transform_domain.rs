mod builder;

pub use builder::TransformDomainBuilder;

use crate::float::Float;

use super::{Task, TaskSource};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum DomainOperation {
    #[default]
    Translate,
    Scale,
}

#[derive(Clone, Debug)]
pub struct TransformDomain<T: Float> {
    source: TaskSource<T>,
    operation: DomainOperation,
    value: [TaskSource<T>; 3],
}

impl<T: Float> Default for TransformDomain<T> {
    fn default() -> Self {
        Self {
            operation: DomainOperation::Translate,
            source: T::ZERO.into(),
            value: [T::ZERO.into(), T::ZERO.into(), T::ZERO.into()],
        }
    }
}

impl<T: Float> Task<T> for TransformDomain<T> {
    fn sample_1d(&mut self, x: T) -> T {
        let result = self.eval([x, T::ZERO, T::ZERO], |t| t.sample_1d(x));
        self.source.sample_1d(result[0])
    }

    fn sample_2d(&mut self, x: T, y: T) -> T {
        let result = self.eval([x, y, T::ZERO], |t| t.sample_2d(x, y));
        self.source.sample_2d(result[0], result[1])
    }

    fn sample_3d(&mut self, x: T, y: T, z: T) -> T {
        let result = self.eval([x, y, z], |t| t.sample_3d(x, y, z));
        self.source.sample_3d(result[0], result[1], result[2])
    }
}

impl<T: Float> TransformDomain<T> {
    fn eval<F: Fn(&mut dyn Task<T>) -> T>(&mut self, args: [T; 3], sampler: F) -> [T; 3] {
        use DomainOperation::*;
        let v: Vec<T> = match &self.operation {
            Translate => args
                .iter()
                .zip(self.value.iter_mut())
                .map(|(x, y)| *x + sampler(y))
                .collect(),
            Scale => args
                .iter()
                .zip(self.value.iter_mut())
                .map(|(x, y)| *x * sampler(y))
                .collect(),
        };

        [v[0], v[1], v[2]]
    }
}
