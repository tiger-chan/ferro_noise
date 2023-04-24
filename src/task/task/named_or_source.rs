use crate::float::Float;

use super::TaskSource;

pub(crate) enum NameOrSource<T: Float> {
    Named(String),
    Source(TaskSource<T>),
}

macro_rules! source_or_message {
    ($value:expr, $class:ty) => {
        match &$value {
            NameOrSource::Source(x) => x.clone(),
            _ => {
                let class_name = std::any::type_name::<$class>();
                let func = module_path!();
                panic!("{}::link must be called if {} is used", class_name, func);
            }
        }
    };
}

macro_rules! named_to_task {
    ($value:expr, $tree:expr) => {
        match &$value {
            NameOrSource::Named(name) => {
                if let Some(task) = $tree.get(name) {
                    $value = NameOrSource::Source(task.clone());
                }
            }
            _ => {}
        }
    };
}

pub(crate) use named_to_task;
pub(crate) use source_or_message;
