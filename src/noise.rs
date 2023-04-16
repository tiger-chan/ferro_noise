use crate::Float;


pub trait Noise1D<T: Float> {
    fn eval(&self, x: T) -> T;
}

pub trait Noise2D<T: Float> {
    fn eval(&self, x: T, y: T) -> T;
}

pub trait Noise3D<T: Float> {
    fn eval(&self, x: T, y: T, z: T) -> T;
}