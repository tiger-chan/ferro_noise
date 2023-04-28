use serde::{Deserialize, Serialize};

use crate::task::DomainOperation;

use super::name_or_const::NameOrConst;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(default)]
pub struct TransformDomainConfig {
    operation: DomainOperation,
    dx: NameOrConst,
    dy: NameOrConst,
    dz: NameOrConst,
    source: NameOrConst,
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
