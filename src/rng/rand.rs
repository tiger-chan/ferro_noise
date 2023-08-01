pub trait Random<T> {
	fn next(&mut self) -> T;
}
