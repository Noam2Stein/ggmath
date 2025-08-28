mod f32;
#[allow(unused_imports)]
pub use f32::*;

mod f64;
#[allow(unused_imports)]
pub use f64::*;

mod i8;
#[allow(unused_imports)]
pub use i8::*;

mod i16;
#[allow(unused_imports)]
pub use i16::*;

mod i32;
#[allow(unused_imports)]
pub use i32::*;

mod i64;
#[allow(unused_imports)]
pub use i64::*;

mod i128;
#[allow(unused_imports)]
pub use i128::*;

mod isize;
#[allow(unused_imports)]
pub use isize::*;

mod u8;
#[allow(unused_imports)]
pub use u8::*;

mod u16;
#[allow(unused_imports)]
pub use u16::*;

mod u32;
#[allow(unused_imports)]
pub use u32::*;

mod u64;
#[allow(unused_imports)]
pub use u64::*;

mod u128;
#[allow(unused_imports)]
pub use u128::*;

mod usize;
#[allow(unused_imports)]
pub use usize::*;

mod bool;
#[allow(unused_imports)]
pub use bool::*;

#[cfg(test)]
mod tests {
    use crate::vector::Scalar;

    fn _test_scalar_impl() {
        fn helper<T: Scalar>() {}

        helper::<f32>();
        helper::<f64>();
        helper::<i8>();
        helper::<i16>();
        helper::<i32>();
        helper::<i64>();
        helper::<i128>();
        helper::<isize>();
        helper::<u8>();
        helper::<u16>();
        helper::<u32>();
        helper::<u64>();
        helper::<u128>();
        helper::<usize>();
        helper::<bool>();
    }
}
