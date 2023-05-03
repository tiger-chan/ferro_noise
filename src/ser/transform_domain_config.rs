use serde::{Deserialize, Serialize};

use crate::{
    float::Float,
    task::{DomainOperation, TransformDomainBuilder},
};

use super::{name_or_const::*, IntoTaskSource, TaskDependencies};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(default)]
pub struct TransformDomainConfig {
    pub operation: DomainOperation,
    pub dx: NameOrConst,
    pub dy: NameOrConst,
    pub dz: NameOrConst,
    pub source: NameOrConst,
}

impl Default for TransformDomainConfig {
    fn default() -> Self {
        Self {
            operation: DomainOperation::default(),
            dx: 0.0.into(),
            dy: 0.0.into(),
            dz: 0.0.into(),
            source: 1.0.into(),
        }
    }
}

impl TaskDependencies for TransformDomainConfig {
    fn dependencies(&self) -> Vec<String> {
        let mut r = vec![];
        push_named_to_vec!(r, self.dx);
        push_named_to_vec!(r, self.dy);
        push_named_to_vec!(r, self.dz);
        push_named_to_vec!(r, self.source);
        r
    }
}

impl<T: Float> IntoTaskSource<T> for TransformDomainConfig {
    fn config_into(&self, tree: &crate::task::TaskTree<T>) -> crate::task::TaskSource<T> {
        let mut builder = TransformDomainBuilder::<T>::new();

        builder.operation(self.operation);
        add_task_to_builder!(self.dx, builder, value_x, named_value_x, tree);
        add_task_to_builder!(self.dy, builder, value_y, named_value_y, tree);
        add_task_to_builder!(self.dz, builder, value_z, named_value_z, tree);
        add_task_to_builder!(self.source, builder, source, named_source, tree);

        builder.link(tree).build().into()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::ser::TaskConfig;

    use super::*;

    #[test]
    fn deserialize() {
        let data = toml::to_string(&toml::toml! {
            [transform_domain_a]
            transform_domain.dx = "dx"
            transform_domain.dy = -1
            transform_domain.source = 2.0

            [transform_domain_b]
            transform_domain = { dx = "dx" }
        })
        .unwrap();
        let config: HashMap<String, TaskConfig> = toml::from_str(data.as_str()).unwrap();

        assert_eq!(config.len(), 2);
        assert_eq!(
            config["transform_domain_a"],
            TaskConfig::TransformDomain(TransformDomainConfig {
                dx: "dx".to_owned().into(),
                dy: (-1.0).into(),
                source: 2.0.into(),
                ..Default::default()
            })
        );

        assert_eq!(
            config["transform_domain_b"],
            TaskConfig::TransformDomain(TransformDomainConfig {
                dx: "dx".to_owned().into(),
                ..Default::default()
            })
        );
    }
}
