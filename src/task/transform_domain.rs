mod builder;

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

macro_rules! transform_domain {
    ($type: ty) => {
        #[derive(Clone, Debug)]
        pub struct TransformDomain {
            pub(crate) source: TaskSource,
            pub(crate) operation: DomainOperation,
            pub(crate) value: [TaskSource; 3],
        }

        impl Default for TransformDomain {
            fn default() -> Self {
                Self {
                    operation: DomainOperation::Translate,
                    source: 0.0.into(),
                    value: [0.0.into(), 0.0.into(), 0.0.into()],
                }
            }
        }

        impl Task for TransformDomain {
            fn sample_1d(&mut self, x: $type) -> $type {
                let result = self.eval([x, 0.0, 0.0], |t| t.sample_1d(x));
                self.source.sample_1d(result[0])
            }

            fn sample_2d(&mut self, x: $type, y: $type) -> $type {
                let result = self.eval([x, y, 0.0], |t| t.sample_2d(x, y));
                self.source.sample_2d(result[0], result[1])
            }

            fn sample_3d(&mut self, x: $type, y: $type, z: $type) -> $type {
                let result = self.eval([x, y, z], |t| t.sample_3d(x, y, z));
                self.source.sample_3d(result[0], result[1], result[2])
            }
        }

        impl TransformDomain {
            fn eval<F: Fn(&mut dyn Task) -> $type>(
                &mut self,
                args: [$type; 3],
                sampler: F,
            ) -> [$type; 3] {
                use DomainOperation::*;
                let v: Vec<$type> = match &self.operation {
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
    };
}

pub mod f32 {
    pub use super::builder::f32::TransformDomainBuilder;
    pub use super::DomainOperation;
    use crate::task::f32::{Task, TaskSource};

    transform_domain!(f32);
}

pub mod f64 {
    pub use super::builder::f64::TransformDomainBuilder;
    pub use super::DomainOperation;
    use crate::task::f64::{Task, TaskSource};

    transform_domain!(f64);
}
