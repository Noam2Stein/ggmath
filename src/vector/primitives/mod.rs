mod bool;
mod f32;
mod f64;
mod i128;
mod i16;
mod i32;
mod i64;
mod i8;
mod isize;
mod option;
mod u128;
mod u16;
mod u32;
mod u64;
mod u8;
mod usize;

/// Wraps `T` in 16-byte alignment.
#[repr(align(16))]
#[derive(Clone, Copy)]
pub struct Aligned128<T>(pub T);

/// Wraps `T` in 32-byte alignment.
#[repr(align(32))]
#[derive(Clone, Copy)]
pub struct Aligned256<T>(pub T);

/// Wraps `T` in 64-byte alignment.
#[repr(align(64))]
#[derive(Clone, Copy)]
pub struct Aligned512<T>(pub T);

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::vector::{Scalar, VecAligned, VecPacked, Vector};

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

    #[cfg(feature = "primitive_aliases")]
    fn _test_primitive_aliases() {
        fn helper<T>() -> PhantomData<T> {
            PhantomData
        }

        let _: PhantomData<Vector<2, f32, VecAligned>> = helper::<crate::f32_aliases::FVec2>();
        let _: PhantomData<Vector<4, bool, VecPacked>> = helper::<crate::bool_aliases::BVec4P>();
    }
}
