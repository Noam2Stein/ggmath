use wide::{
    f32x4, f32x8, f32x16, f64x2, f64x4, f64x8, i8x16, i8x32, i16x8, i16x16, i16x32, i32x4, i32x8,
    i32x16, i64x2, i64x4, i64x8, u8x16, u8x32, u16x8, u16x16, u16x32, u32x4, u32x8, u32x16, u64x2,
    u64x4, u64x8,
};

use crate::{Alignment, Length, ScalarBackend, SupportedLength};

macro_rules! default_impl {
    ($T:ident) => {
        impl<const N: usize, A: Alignment> ScalarBackend<N, A> for $T where
            Length<N>: SupportedLength
        {
        }
    };
}
default_impl!(f32x4);
default_impl!(f32x8);
default_impl!(f32x16);
default_impl!(f64x2);
default_impl!(f64x4);
default_impl!(f64x8);
default_impl!(i8x16);
default_impl!(i8x32);
default_impl!(i16x8);
default_impl!(i16x16);
default_impl!(i16x32);
default_impl!(i32x4);
default_impl!(i32x8);
default_impl!(i32x16);
default_impl!(i64x2);
default_impl!(i64x4);
default_impl!(i64x8);
default_impl!(u8x16);
default_impl!(u8x32);
default_impl!(u16x8);
default_impl!(u16x16);
default_impl!(u16x32);
default_impl!(u32x4);
default_impl!(u32x8);
default_impl!(u32x16);
default_impl!(u64x2);
default_impl!(u64x4);
default_impl!(u64x8);
