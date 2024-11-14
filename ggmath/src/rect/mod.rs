use crate::{
    scalar::*,
    vector::{alignment::*, length::*, *},
};

pub struct Rectangle<const N: usize, T: Scalar, A: VecAlignment>
where
    ScalarCount<N>: VecLen<N>,
{
    pub min: Vector<N, T, A>,
    pub size: Vector<N, T, A>,
}

pub type Rect2<T> = Rectangle<2, T, VecAligned>;
pub type Rect3<T> = Rectangle<3, T, VecAligned>;
pub type Rect4<T> = Rectangle<4, T, VecAligned>;

pub type Rect2P<T> = Rectangle<2, T, VecPacked>;
pub type Rect3P<T> = Rectangle<3, T, VecPacked>;
pub type Rect4P<T> = Rectangle<4, T, VecPacked>;
