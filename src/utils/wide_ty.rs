use core::ops::{BitAnd, BitOr, BitXor, Not};

use wide::{
    CmpEq, CmpGt, CmpLt, f32x4, f32x8, f32x16, f64x2, f64x4, f64x8, i8x16, i8x32, i16x8, i16x16,
    i16x32, i32x4, i32x8, i32x16, i64x2, i64x4, i64x8, u8x16, u8x32, u16x8, u16x16, u16x32, u32x4,
    u32x8, u32x16, u64x2, u64x4, u64x8,
};

use crate::Scalar;

pub(crate) trait WideTy:
    Scalar
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + CmpEq<Output = Self>
    + CmpLt<Output = Self>
    + CmpGt<Output = Self>
{
    fn blend(self, t: Self, f: Self) -> Self;
}

macro_rules! impl_wide {
    ($Wide:ident, $T:ident, $LANES:literal) => {
        impl WideTy for $Wide {
            #[inline(always)]
            fn blend(self, t: Self, f: Self) -> Self {
                self.blend(t, f)
            }
        }
    };
}
impl_wide!(f32x4, f32, 4);
impl_wide!(f32x8, f32, 8);
impl_wide!(f32x16, f32, 16);
impl_wide!(f64x2, f64, 2);
impl_wide!(f64x4, f64, 4);
impl_wide!(f64x8, f64, 8);
impl_wide!(i8x16, i8, 16);
impl_wide!(i8x32, i8, 32);
impl_wide!(i16x8, i16, 8);
impl_wide!(i16x16, i16, 16);
impl_wide!(i16x32, i16, 32);
impl_wide!(i32x4, i32, 4);
impl_wide!(i32x8, i32, 8);
impl_wide!(i32x16, i32, 16);
impl_wide!(i64x2, i64, 2);
impl_wide!(i64x4, i64, 4);
impl_wide!(i64x8, i64, 8);
impl_wide!(u8x16, u8, 16);
impl_wide!(u8x32, u8, 32);
impl_wide!(u16x8, u16, 8);
impl_wide!(u16x16, u16, 16);
impl_wide!(u16x32, u16, 32);
impl_wide!(u32x4, u32, 4);
impl_wide!(u32x8, u32, 8);
impl_wide!(u32x16, u32, 16);
impl_wide!(u64x2, u64, 2);
impl_wide!(u64x4, u64, 4);
impl_wide!(u64x8, u64, 8);
