use super::*;

pub use ggmath_proc_macros::impl_from_split_transmute;
pub use ggmath_proc_macros::impl_from_splits_transmute;

pub trait FromVecSplit<S>: ElementContainer {
    fn from_split(split: S) -> Self;
}

impl<T: Element, S> FromVecSplit<S> for Vec2<T>
where
    T::InnerVec2: FromVecSplit<S>,
{
    fn from_split(split: S) -> Self {
        Self {
            inner: T::InnerVec2::from_split(split),
        }
    }
}
impl<T: Element, S> FromVecSplit<S> for Vec3<T>
where
    T::InnerVec3: FromVecSplit<S>,
{
    fn from_split(split: S) -> Self {
        Self {
            inner: T::InnerVec3::from_split(split),
        }
    }
}
impl<T: Element, S> FromVecSplit<S> for Vec4<T>
where
    T::InnerVec4: FromVecSplit<S>,
{
    fn from_split(split: S) -> Self {
        Self {
            inner: T::InnerVec4::from_split(split),
        }
    }
}

macro_rules! from_vec_splits {
    ($ident:ident: [$(($($vec:ident), * $(,)?)), * $(,)?]) => {
        #[allow(unused_parens)]
        pub trait $ident: ElementContainer + $(FromVecSplit<($($vec<<Self as ElementContainer>::T>), *)> +)* {

        }
    };
}
from_vec_splits!(
    FromVec2Splits: [
        (Vec1, Vec1),
        (Vec2),
    ]
);
from_vec_splits!(
    FromVec3Splits: [
        (Vec1, Vec1, Vec1),
        (Vec2, Vec1),
        (Vec1, Vec2),
        (Vec3),
    ]
);
from_vec_splits!(
    FromVec4Splits: [
        (Vec1, Vec1, Vec1, Vec1),
        (Vec2, Vec1, Vec1),
        (Vec1, Vec2, Vec1),
        (Vec1, Vec1, Vec2),
        (Vec2, Vec2),
        (Vec3, Vec1),
        (Vec1, Vec3),
        (Vec3),
    ]
);
