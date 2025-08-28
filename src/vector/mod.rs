use std::{
    fmt::{Debug, Display},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
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

    #[inline(always)]
    fn vec_neg<const N: usize, A: VecAlignment>(
        vector: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Neg<Output: Scalar>,
    {
        vector.map(|x| -x)
    }

    #[inline(always)]
    fn vec_not<const N: usize, A: VecAlignment>(
        vector: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Not<Output: Scalar>,
    {
        vector.map(|x| !x)
    }

    #[inline(always)]
    fn vec_add<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Add<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Add<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] + other[i])
    }

    #[inline(always)]
    fn vec_sub<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Sub<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Sub<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] - other[i])
    }

    #[inline(always)]
    fn vec_mul<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Mul<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Mul<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] * other[i])
    }

    #[inline(always)]
    fn vec_div<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Div<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Div<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] / other[i])
    }

    #[inline(always)]
    fn vec_rem<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Rem<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Rem<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] % other[i])
    }

    #[inline(always)]
    fn vec_shl<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Shl<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Shl<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] << other[i])
    }

    #[inline(always)]
    fn vec_shr<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Shr<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Shr<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] >> other[i])
    }

    #[inline(always)]
    fn vec_bitand<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as BitAnd<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitAnd<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] & other[i])
    }

    #[inline(always)]
    fn vec_bitor<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as BitOr<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitOr<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] | other[i])
    }

    #[inline(always)]
    fn vec_bitxor<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as BitXor<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitXor<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] ^ other[i])
    }

    #[inline(always)]
    fn vec_add_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: AddAssign<T2>,
    {
        for i in 0..N {
            vector[i] += other[i];
        }
    }

    #[inline(always)]
    fn vec_sub_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: SubAssign<T2>,
    {
        for i in 0..N {
            vector[i] -= other[i];
        }
    }

    #[inline(always)]
    fn vec_mul_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: MulAssign<T2>,
    {
        for i in 0..N {
            vector[i] *= other[i];
        }
    }

    #[inline(always)]
    fn vec_div_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: DivAssign<T2>,
    {
        for i in 0..N {
            vector[i] /= other[i];
        }
    }

    #[inline(always)]
    fn vec_rem_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: RemAssign<T2>,
    {
        for i in 0..N {
            vector[i] %= other[i];
        }
    }

    #[inline(always)]
    fn vec_shl_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: ShlAssign<T2>,
    {
        for i in 0..N {
            vector[i] <<= other[i];
        }
    }

    #[inline(always)]
    fn vec_shr_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: ShrAssign<T2>,
    {
        for i in 0..N {
            vector[i] >>= other[i];
        }
    }

    #[inline(always)]
    fn vec_bitand_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: BitAndAssign<T2>,
    {
        for i in 0..N {
            vector[i] &= other[i];
        }
    }

    #[inline(always)]
    fn vec_bitor_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: BitOrAssign<T2>,
    {
        for i in 0..N {
            vector[i] |= other[i];
        }
    }

    #[inline(always)]
    fn vec_bitxor_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: BitXorAssign<T2>,
    {
        for i in 0..N {
            vector[i] ^= other[i];
        }
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

impl<const N: usize, T: Scalar + Neg<Output: Scalar>, A: VecAlignment> Neg for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        T::vec_neg(self)
    }
}

impl<const N: usize, T: Scalar + Not<Output: Scalar>, A: VecAlignment> Not for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        T::vec_not(self)
    }
}

impl<
    const N: usize,
    T: Scalar + Add<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Add<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn add(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_add(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Sub<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Sub<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn sub(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_sub(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Mul<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Mul<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn mul(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_mul(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Div<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Div<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn div(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_div(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Rem<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Rem<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn rem(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_rem(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Shl<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Shl<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shl(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_shl(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Shr<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Shr<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shr(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_shr(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + BitAnd<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> BitAnd<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitand(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_bitand(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + BitOr<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> BitOr<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitor(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_bitor(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + BitXor<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> BitXor<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitxor(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_bitxor(self, other)
    }
}

impl<const N: usize, T: Scalar + AddAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    AddAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn add_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_add_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + SubAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    SubAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn sub_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_sub_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + MulAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    MulAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn mul_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_mul_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + DivAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    DivAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn div_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_div_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + RemAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    RemAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn rem_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_rem_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + ShlAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    ShlAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn shl_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_shl_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + ShrAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    ShrAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn shr_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_shr_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + BitAndAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    BitAndAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitand_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_bitand_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + BitOrAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    BitOrAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitor_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_bitor_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + BitXorAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    BitXorAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitxor_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_bitxor_assign(self, other)
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
