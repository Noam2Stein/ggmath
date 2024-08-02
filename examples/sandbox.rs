use gmath::*;

pub fn main() {
    println!("{}", test::<f32>() / Vec4::splat(2.0));
}

pub fn test<T: Num>() -> Vec4<T> {
    Vec4::new(T::ZERO, T::ONE, T::ONE + T::ONE, T::ONE + T::ONE + T::ONE)
}