use crate::{float::Float, source::Blender, task::TaskSource};

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
        use TaskSource::Constant;
        Self {
            blender: linear_curve,
            condition: Source(Constant(T::ZERO)),
            lower: Source(Constant(-T::ONE)),
            upper: Source(Constant(T::ONE)),
            falloff: Source(Constant(T::ZERO)),
            threshold: Source(Constant(T::from(0.5))),
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

    pub fn condition(&mut self, condition: TaskSource<T>) -> &mut Self {
        self.condition = NameOrSource::Source(condition);
        self
    }

    pub fn falloff(&mut self, falloff: TaskSource<T>) -> &mut Self {
        self.falloff = NameOrSource::Source(falloff);
        self
    }

    pub fn lower(&mut self, lower: TaskSource<T>) -> &mut Self {
        self.lower = NameOrSource::Source(lower);
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

    pub fn threadhold(&mut self, threshold: TaskSource<T>) -> &mut Self {
        self.threshold = NameOrSource::Source(threshold);
        self
    }

    pub fn upper(&mut self, upper: TaskSource<T>) -> &mut Self {
        self.upper = NameOrSource::Source(upper);
        self
    }
}
