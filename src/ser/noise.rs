use std::collections::HashMap;

use crate::{
    float::Float,
    task::{CacheBuilder, TaskSource, TaskTree},
};

use super::{
    aggregate_config::*, bias_config::*, fractal_config::*, gradient_config::*, selector_config::*,
    transform_domain_config::*,
};

use serde::{Deserialize, Serialize};

pub(crate) trait TaskDependencies {
    fn dependencies(&self) -> Vec<String>;
}

pub(crate) trait IntoTaskSource<U: Float> {
    #[must_use]
    fn config_into(&self, tree: &TaskTree<U>) -> TaskSource<U>;
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TaskConfig {
    Aggregate(AggregateConfig),
    Bias(BiasConfig),
    Cache(String),
    Constant(f64),
    Fractal(FractalConfig),
    Gradient(GradientConfig),
    Selector(SelectorConfig),
    TransformDomain(TransformDomainConfig),
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self::Constant(0.0)
    }
}

impl TaskDependencies for TaskConfig {
    fn dependencies(&self) -> Vec<String> {
        match &self {
            Self::Aggregate(x) => x.dependencies(),
            Self::Bias(x) => x.dependencies(),
            Self::Cache(x) => vec![x.clone()],
            Self::Constant(_) => vec![],
            Self::Fractal(x) => x.dependencies(),
            Self::Gradient(x) => x.dependencies(),
            Self::Selector(x) => x.dependencies(),
            Self::TransformDomain(x) => x.dependencies(),
        }
    }
}

impl<U: Float> IntoTaskSource<U> for TaskConfig {
    fn config_into(&self, tree: &TaskTree<U>) -> TaskSource<U> {
        match &self {
            TaskConfig::Aggregate(x) => x.config_into(tree),
            TaskConfig::Bias(x) => x.config_into(tree),
            TaskConfig::Cache(x) => CacheBuilder::new()
                .named_source(x)
                .link(tree)
                .build()
                .into(),
            TaskConfig::Constant(x) => TaskSource::<U>::from(U::as_float(*x)),
            TaskConfig::Fractal(x) => x.config_into(tree),
            TaskConfig::Gradient(x) => x.config_into(tree),
            TaskConfig::Selector(x) => x.config_into(tree),
            TaskConfig::TransformDomain(x) => x.config_into(tree),
        }
    }
}

/// https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
///
/// L ← Empty list that will contain the sorted elements
/// S ← Set of all nodes with no incoming edge
///
/// while S is not empty do
/// remove a node n from S
///     add n to L
/// 	for each node m with an edge e from n to m do
/// 	remove edge e from the graph
/// 	if m has no other incoming edges then
/// 		insert m into S
///
/// if graph has edges then
/// 	return error   (graph has at least one cycle)
/// else
/// 	return L   (a topologically sorted order)
fn sort_tasks(tasks: &HashMap<String, TaskConfig>) -> Result<Vec<String>, String> {
    let graph_tasks: Vec<&String> = tasks.keys().collect();
    let index_of = |o: &String| -> Option<usize> {
        for (i, t) in graph_tasks.iter().enumerate() {
            if *t == o {
                return Some(i);
            } else {
                continue;
            }
        }
        None
    };
    // Create list of how many dependents a task has
    let mut graph = vec![Vec::<usize>::new(); graph_tasks.len()];

    for (i, t) in graph_tasks.iter().enumerate() {
        for t in tasks[*t].dependencies().iter() {
            if let Some(t) = index_of(t) {
                graph[t].push(i);
            } else {
                return Err(format!("Task '{}' doesn't exist", t));
            }
        }
    }

    let mut in_edges = vec![0; tasks.len()];
    for node in graph.iter() {
        for i in node {
            in_edges[*i] += 1;
        }
    }

    // Sorted Output list
    let mut l = Vec::with_capacity(tasks.len());
    // List of vertex with no dependencies
    let mut s: Vec<usize> = in_edges
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == 0)
        .map(|(i, _)| i)
        .collect();

    while let Some(n) = s.pop() {
        l.push(graph_tasks[n].to_owned());
        for m in graph[n].iter() {
            in_edges[*m] -= 1;
            if in_edges[*m] == 0 {
                s.push(*m);
            }
        }
    }

    if in_edges.iter().any(|x| *x != 0) {
        Err("Cycle Detected".to_owned())
    } else {
        Ok(l)
    }
}

#[cfg(feature = "toml")]
pub mod toml {
    use super::*;
    pub fn from_str(data: impl Into<String>) -> Result<Box<TaskTree<f64>>, String> {
        let result: Result<HashMap<String, TaskConfig>, ::toml::de::Error> =
            ::toml::from_str(&data.into());
        match result {
            Ok(mut result) => {
                let sorted_tasks = sort_tasks(&result)?;
                let mut tree = Box::new(TaskTree::new());

                for task_name in sorted_tasks {
                    let config: &TaskConfig = result.entry(task_name.clone()).or_default();
                    let task: TaskSource<f64> = config.config_into(tree.as_ref());
                    tree.add_task(&task_name, task);
                }

                Ok(tree)
            }
            Err(x) => Err(x.to_string()),
        }
    }

    #[cfg(test)]
    mod test {
        use std::collections::HashMap;

        use super::*;

        #[test]
        fn generic_parse() {
            let data = ::toml::to_string(&::toml::toml! {
                [const_a]
                constant = 1.0

                [cache_b]
                cache = "fractal_a"

                [fractal_a]
                fractal = { octaves = 1, frequency = 0.5, source = "perlin" }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = ::toml::from_str(data.as_str()).unwrap();

            assert_eq!(config.len(), 3);
            assert_eq!(config["const_a"], TaskConfig::Constant(1.0));
            assert_eq!(config["cache_b"], TaskConfig::Cache("fractal_a".to_owned()));
            assert_eq!(
                config["fractal_a"],
                TaskConfig::Fractal(FractalConfig {
                    octaves: 1,
                    frequency: 0.5,
                    source: FractalSource::Perlin,
                    ..Default::default()
                })
            );
        }

        #[test]
        fn generic_sort() {
            let data = ::toml::to_string(&::toml::toml! {
                [const_a]
                constant = 1.0

                [cache_b]
                cache = "fractal_a"

                [fractal_a]
                fractal = { octaves = 1, frequency = 0.5, source = "perlin" }
            })
            .unwrap();
            let config: HashMap<String, TaskConfig> = ::toml::from_str(data.as_str()).unwrap();

            let all_possible = vec![
                vec![
                    "const_a".to_owned(),
                    "fractal_a".to_owned(),
                    "cache_b".to_owned(),
                ],
                vec![
                    "fractal_a".to_owned(),
                    "cache_b".to_owned(),
                    "const_a".to_owned(),
                ],
            ];

            let sorted = sort_tasks(&config);
            assert!(sorted.is_ok());
            let sorted = sorted.unwrap();
            assert_eq!(sorted.len(), 3);
            assert!(all_possible.contains(&sorted));
        }

        #[test]
        fn helper_from_str() {
            let data = r#"
            [const_a]
            constant = 1.0

            [cache_b]
            cache = "fractal_a"

            [fractal_a]
            fractal = { octaves = 1, frequency = 0.5, source = "perlin" }
		"#;
            match from_str(data) {
                Ok(mut x) => {
                    assert!(x.get("const_a").is_some());
                    assert!(x.get("cache_b").is_some());
                    assert!(x.get("fractal_a").is_some());

                    assert_eq!(x.sample_1d("const_a", 1.0), 1.0);
                }
                Err(x) => panic!("{}", x),
            }
        }
    }
}
