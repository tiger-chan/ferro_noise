use crate::{
    float::Float,
    source::Blender,
    task::{TaskSource, TaskTree},
};

use super::Selector;

enum NameOrSource<T: Float> {
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

pub struct SelectorBuilder<T: Float> {
    blender: Blender<T>,
    condition: NameOrSource<T>,
    lower: NameOrSource<T>,
    upper: NameOrSource<T>,
    falloff: NameOrSource<T>,
    /// threadhold/pivot/boundry to determine when lower or upper is used
    threshold: NameOrSource<T>,
}

#[allow(dead_code)]
impl<T: Float> SelectorBuilder<T> {
    pub fn new() -> Self {
        use crate::math::linear_curve;
        use NameOrSource::Source;
        Self {
            blender: linear_curve,
            condition: Source(T::ZERO.into()),
            lower: Source((-T::ONE).into()),
            upper: Source(T::ONE.into()),
            falloff: Source(T::ZERO.into()),
            threshold: Source(T::from(0.5).into()),
        }
    }

    pub fn blender(&mut self, blender: Blender<T>) -> &mut Self {
        self.blender = blender;
        self
    }

    pub fn build(&mut self) -> Selector<T> {
        Selector {
            blender: self.blender,
            condition: source_or_message!(self.condition, SelectorBuilder<T>),
            lower: source_or_message!(self.lower, SelectorBuilder<T>),
            upper: source_or_message!(self.upper, SelectorBuilder<T>),
            falloff: source_or_message!(self.falloff, SelectorBuilder<T>),
            threshold: source_or_message!(self.threshold, SelectorBuilder<T>),
        }
    }

    pub fn condition<V: Into<TaskSource<T>>>(&mut self, condition: V) -> &mut Self {
        self.condition = NameOrSource::Source(condition.into());
        self
    }

    pub fn falloff<V: Into<TaskSource<T>>>(&mut self, falloff: V) -> &mut Self {
        self.falloff = NameOrSource::Source(falloff.into());
        self
    }

    /// Link named tasks to their task tree values
    pub fn link(&mut self, tree: &TaskTree<T>) -> &mut Self {
        named_to_task!(self.condition, tree);
        named_to_task!(self.falloff, tree);
        named_to_task!(self.lower, tree);
        named_to_task!(self.threshold, tree);
        named_to_task!(self.upper, tree);

        self
    }

    pub fn lower<V: Into<TaskSource<T>>>(&mut self, lower: V) -> &mut Self {
        self.lower = NameOrSource::Source(lower.into());
        self
    }

    pub fn named_condition<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.condition = NameOrSource::Named(name.into());
        self
    }

    pub fn named_falloff<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.falloff = NameOrSource::Named(name.into());
        self
    }

    pub fn named_lower<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.lower = NameOrSource::Named(name.into());
        self
    }

    pub fn named_threadhold<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.threshold = NameOrSource::Named(name.into());
        self
    }

    pub fn named_upper<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.upper = NameOrSource::Named(name.into());
        self
    }

    pub fn threadhold<V: Into<TaskSource<T>>>(&mut self, threshold: V) -> &mut Self {
        self.threshold = NameOrSource::Source(threshold.into());
        self
    }

    pub fn upper<V: Into<TaskSource<T>>>(&mut self, upper: V) -> &mut Self {
        self.upper = NameOrSource::Source(upper.into());
        self
    }
}
