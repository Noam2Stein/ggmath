use newnum::Num;

use crate::{construct::*, ggmath, scalar::*, vector::*};

mod api;
mod generics;
mod impl_std;
pub use api::*;
pub use generics::*;

pub struct Rectangle<const N: usize, T: Scalar + Num, A: VecAlignment, R: RectRepr>
where
    ScalarCount<N>: VecLen,
{
    inner: R::InnerRectangle<N, T, A>,
}

pub type Rect2<T> = Rectangle<2, T, VecAligned, RectCornered>;
pub type Rect3<T> = Rectangle<3, T, VecAligned, RectCornered>;
pub type Rect4<T> = Rectangle<4, T, VecAligned, RectCornered>;
pub type Rect2P<T> = Rectangle<2, T, VecPacked, RectCornered>;
pub type Rect3P<T> = Rectangle<3, T, VecPacked, RectCornered>;
pub type Rect4P<T> = Rectangle<4, T, VecPacked, RectCornered>;

pub type Rect2C<T> = Rectangle<2, T, VecAligned, RectCentered>;
pub type Rect3C<T> = Rectangle<3, T, VecAligned, RectCentered>;
pub type Rect4C<T> = Rectangle<4, T, VecAligned, RectCentered>;
pub type Rect2CP<T> = Rectangle<2, T, VecPacked, RectCentered>;
pub type Rect3CP<T> = Rectangle<3, T, VecPacked, RectCentered>;
pub type Rect4CP<T> = Rectangle<4, T, VecPacked, RectCentered>;

pub type Rect2M<T> = Rectangle<2, T, VecAligned, RectMinMaxed>;
pub type Rect3M<T> = Rectangle<3, T, VecAligned, RectMinMaxed>;
pub type Rect4M<T> = Rectangle<4, T, VecAligned, RectMinMaxed>;
pub type Rect2MP<T> = Rectangle<2, T, VecPacked, RectMinMaxed>;
pub type Rect3MP<T> = Rectangle<3, T, VecPacked, RectMinMaxed>;
pub type Rect4MP<T> = Rectangle<4, T, VecPacked, RectMinMaxed>;

pub use ggmath_proc_macros::rectangle_aliases;

#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod f32_aliases for f32(F));
#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod f64_aliases for f64(D));

#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod u8_aliases for u8(U8));
#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod u16_aliases for u16(U16));
#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod u32_aliases for u32(U));
#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod u64_aliases for u64(U64));
#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod u128_aliases for u128(U128));
#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod usize_aliases for usize(Usize));

#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod i8_aliases for i8(I8));
#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod i16_aliases for i16(I16));
#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod i32_aliases for i32(I));
#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod i64_aliases for i64(I64));
#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod i128_aliases for i128(I128));
#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod isize_aliases for isize(Isize));

#[cfg(feature = "primitive_aliases")]
rectangle_aliases!(pub mod bool_aliases for bool(B));
