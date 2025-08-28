use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use crate::{Construct, Usize};

mod interface;

#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub inner: A::InnerVector<N, T>,
}

pub trait VecLen {
    const ENUM: VecLenEnum;

    type InnerAlignedVector<T: Scalar>: Construct;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VecLenEnum {
    Two,
    Three,
    Four,
}

pub trait Scalar: Construct {
    type InnerAlignedVec2: Construct;
    type InnerAlignedVec3: Construct;
    type InnerAlignedVec4: Construct;

    const GARBAGE: Self;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2;
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3;
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4;

    #[inline(always)]
    fn vec_eq<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> bool
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        for i in 0..N {
            if vector[i] != other[i] {
                return false;
            }
        }

        true
    }

    #[inline(always)]
    fn vec_ne<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> bool
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        !Self::vec_eq(vector, other)
    }
}

pub trait VecAlignment: 'static {
    const IS_ALIGNED: bool;

    type InnerVector<const N: usize, T: Scalar>: Construct
    where
        Usize<N>: VecLen;
}

pub struct VecAligned;
pub struct VecPacked;

#[macro_export]
macro_rules! vec2 {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<2, _, VecAligned>::from(($($expr),*,))
    };
}
#[macro_export]
macro_rules! vec3 {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<3, _, VecAligned>::from(($($expr),*,))
    };
}
#[macro_export]
macro_rules! vec4 {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<4, _, VecAligned>::from(($($expr),*,))
    };
}

#[macro_export]
macro_rules! vec2p {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<2, _, VecPacked>::from(($($expr),*,))
    };
}
#[macro_export]
macro_rules! vec3p {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<3, _, VecPacked>::from(($($expr),*,))
    };
}
#[macro_export]
macro_rules! vec4p {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<4, _, VecPacked>::from(($($expr),*,))
    };
}

#[macro_export]
macro_rules! vec2g {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<2, _, _>::from(($($expr),*,))
    };
}
#[macro_export]
macro_rules! vec3g {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<3, _, _>::from(($($expr),*,))
    };
}
#[macro_export]
macro_rules! vec4g {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<4, _, _>::from(($($expr),*,))
    };
}

impl VecLen for Usize<2> {
    const ENUM: VecLenEnum = VecLenEnum::Two;

    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec2;
}
impl VecLen for Usize<3> {
    const ENUM: VecLenEnum = VecLenEnum::Three;

    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec3;
}
impl VecLen for Usize<4> {
    const ENUM: VecLenEnum = VecLenEnum::Four;

    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec4;
}

impl VecAlignment for VecAligned {
    const IS_ALIGNED: bool = true;

    type InnerVector<const N: usize, T: Scalar>
        = <Usize<N> as VecLen>::InnerAlignedVector<T>
    where
        Usize<N>: VecLen;
}
impl VecAlignment for VecPacked {
    const IS_ALIGNED: bool = false;

    type InnerVector<const N: usize, T: Scalar>
        = [T; N]
    where
        Usize<N>: VecLen;
}

impl<const N: usize, T: Scalar, A: VecAlignment> Clone for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<const N: usize, T: Scalar, A: VecAlignment> Copy for Vector<N, T, A> where Usize<N>: VecLen {}

impl<const N: usize, T: Scalar + Debug, A: VecAlignment> Debug for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for i in 0..N {
            if i != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{:?}", self[i])?;
        }

        write!(f, ")")?;

        Ok(())
    }
}
impl<const N: usize, T: Scalar + Display, A: VecAlignment> Display for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for i in 0..N {
            if i != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", self[i])?;
        }

        write!(f, ")")?;

        Ok(())
    }
}

impl<const N: usize, T: Scalar + PartialEq<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    PartialEq<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Vector<N, T2, A2>) -> bool {
        T::vec_eq(self, other)
    }

    #[inline(always)]
    fn ne(&self, other: &Vector<N, T2, A2>) -> bool {
        T::vec_ne(self, other)
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment, I: SliceIndex<[T]>> Index<I> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = I::Output;

    #[inline(always)]
    fn index(&self, index: I) -> &Self::Output {
        &self.as_array()[index]
    }
}
impl<const N: usize, T: Scalar, A: VecAlignment, I: SliceIndex<[T]>> IndexMut<I> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.as_array_mut()[index]
    }
}

impl<T: Scalar, A: VecAlignment> From<(T, T)> for Vector<2, T, A> {
    #[inline(always)]
    fn from(value: (T, T)) -> Self {
        Self::from_array([value.0, value.1])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>,)> for Vector<2, T, A> {
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>,)) -> Self {
        Self::from_array([value.0[0], value.0[1]])
    }
}
impl<T: Scalar, A: VecAlignment> From<(T, T, T)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (T, T, T)) -> Self {
        Self::from_array([value.0, value.1, value.2])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, Vector<2, T, A0>)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (T, Vector<2, T, A0>)) -> Self {
        Self::from_array([value.0, value.1[0], value.1[1]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>, T)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>, T)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.1])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<3, T, A0>,)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (Vector<3, T, A0>,)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.0[2]])
    }
}
impl<T: Scalar, A: VecAlignment> From<(T, T, T, T)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (T, T, T, T)) -> Self {
        Self::from_array([value.0, value.1, value.2, value.3])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, T, Vector<2, T, A0>)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (T, T, Vector<2, T, A0>)) -> Self {
        Self::from_array([value.0, value.1, value.2[0], value.2[1]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, Vector<2, T, A0>, T)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (T, Vector<2, T, A0>, T)) -> Self {
        Self::from_array([value.0, value.1[0], value.1[1], value.2])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, Vector<3, T, A0>)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (T, Vector<3, T, A0>)) -> Self {
        Self::from_array([value.0, value.1[0], value.1[1], value.1[2]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>, T, T)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>, T, T)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.1, value.2])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>, Vector<2, T, A0>)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>, Vector<2, T, A0>)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.1[0], value.1[1]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<3, T, A0>, T)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (Vector<3, T, A0>, T)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.0[2], value.1])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<4, T, A0>,)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (Vector<4, T, A0>,)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.0[2], value.0[3]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _verify_construct_impl<const N: usize, T: Scalar, A: VecAlignment>()
    where
        Usize<N>: VecLen,
    {
        fn helper<T: Construct>() {}

        helper::<Vector<N, T, A>>();
    }
}
