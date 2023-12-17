macro_rules! bias_builder {
	($type: ty) => {
		pub struct BiasBuilder {
			bias: NameOrSource,
			source: NameOrSource,
			min: $type,
			max: $type,
		}

		impl Default for BiasBuilder {
			fn default() -> Self {
				Self {
					bias: NameOrSource::Source(0.0.into()),
					source: NameOrSource::Source(0.0.into()),
					min: 1.0,
					max: 4.0,
				}
			}
		}

		#[allow(dead_code)]
		impl BiasBuilder {
			pub fn new() -> Self {
				Self::default()
			}

			pub fn bias<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
				self.bias = NameOrSource::Source(task.into());
				self
			}

			pub fn build(&self) -> Bias {
				Bias {
					bias: source_or_message!(self.bias, BiasBuilder),
					source: source_or_message!(self.source, BiasBuilder),
					min: 1.0,
					max: 4.0,
				}
			}

			/// Link named tasks to their task tree values
			pub fn link(&mut self, tree: &TaskTree) -> &mut Self {
				match &self.bias {
					NameOrSource::Named(name) => {
						if let Some(task) = tree.get(name) {
							self.bias = NameOrSource::Source(task.clone());
						}
					}
					_ => {}
				}

				match &self.source {
					NameOrSource::Named(name) => {
						if let Some(task) = tree.get(name) {
							self.source = NameOrSource::Source(task.clone());
						}
					}
					_ => {}
				}

				self
			}

			pub fn max(&mut self, max: $type) -> &mut Self {
				self.max = max;
				self
			}

			pub fn min(&mut self, min: $type) -> &mut Self {
				self.min = min;
				self
			}

			pub fn named_bias<S: Into<String>>(&mut self, name: S) -> &mut Self {
				self.bias = NameOrSource::Named(name.into());
				self
			}

			pub fn named_source<S: Into<String>>(&mut self, name: S) -> &mut Self {
				self.source = NameOrSource::Named(name.into());
				self
			}

			pub fn source<V: Into<TaskSource>>(&mut self, task: V) -> &mut Self {
				self.source = NameOrSource::Source(task.into());
				self
			}
		}
	};
}

pub mod f32 {
    use crate::task::f32::{Bias, NameOrSource, TaskSource, TaskTree};
	use crate::task::source_or_message;
    bias_builder!(f32);
}

pub mod f64 {
	use crate::task::f64::{Bias, NameOrSource, TaskSource, TaskTree};
	use crate::task::source_or_message;
    bias_builder!(f64);
}
