use derive_where::derive_where;

use super::{construct::*, scalar::*, vector::*, *};

mod api;
mod generics;
mod impl_std;
pub use api::*;
pub use generics::*;

#[derive_where(Clone, Copy, PartialEq)]
pub struct Rectangle<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr>
where
    Usize<N>: VecLen,
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
