//! Vector related types and traits
use core::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut, Sub},
    slice::SliceIndex,
};

use crate::{Construct, Usize};

mod constructor;
mod dir;
mod interface;
mod optimization;
mod primitives;
pub use dir::*;

#[allow(unused_imports)]
pub use crate::generated::vector::*;

/// A generic vector type.
///
/// This type is generic over 3 parameters:
/// - `N`: The length of the vector, which currently supports 2, 3, and 4.
/// - `T`: The type of the vector, which must implement the [`Scalar`] trait.
/// - `A`: The "alignment" of the vector, which enables or disables SIMD memory alignment.
///
/// This type has very very useful type-aliases:
/// - `Vec{N}<T>` like `Vec2<f32>` is for SIMD aligned vectors
/// - `Vec{N}P<T>` like `Vec2P<f32>` is for non-SIMD aligned vectors
///
/// # Length
///
/// Currently only the lengths 2, 3, and 4 are supported in order to specialize their inner vector type.
/// In the future if rust gains more type-system features, more lengths will be supported.
///
/// Beware that code should never rely on the fact that 2, 3, and 4 are the only supported lengths.
/// Code that branches based on vector length should either properly handle all usize values or use [`VecLenEnum`].
///
/// # Alignment
///
/// The `A` generic parameter controls whether or not the vector is SIMD aligned,
/// and can be set to either `VecAligned` or `VecPacked`.
///
/// SIMD can improve performance of vector operations,
/// but it can also increase the size of the vector in memory.
///
/// `Vector<N, T, VecAligned>` vectors are SIMD aligned if it increases performance,
/// while `Vector<N, T, VecPacked>` vectors are not SIMD aligned and are always stored as `[T; N]`.
///
/// This means that `VecAligned` are for performance and `VecPacked` are for memory efficiency.
///
/// Beware that while `VecPacked` guarentees an exact memory layout of `[T; N]`, `VecAligned` does not guarantee a specific alignment rule/pattern.
/// For example, `Vector<3, f32, VecAligned`/`Vec3<f32>` isn't guaranteed to be aligned to a 128-bit boundary.
/// It is up to the implementation of [`Scalar`] to determine `VecAligned` alignment for whatever is most performant.
///
/// # Examples
/// ```
/// use ggmath::aliases::*;
///
/// // This is a non memory critical scenario so we should use `VecAligned`.
/// struct PlayerState {
///     // Vector<3, f32, VecAligned>
///     position: Vec3<f32>,
///     // ...
/// }
///
/// // This is a memory critical scenario so we should use `VecPacked`.
/// struct GpuVertex {
///     // Vector<3, f32, VecPacked>
///     position: Vec3P<f32>,
///     // Vector<3, f32, VecPacked>
///     normal: Vec3P<f32>,
///     // Vector<2, f32, VecPacked>
///     uv: Vec2P<f32>,
/// }
/// ```
#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>(pub A::InnerVector<N, T>)
where
    Usize<N>: VecLen;

/// A trait that marks a `Usize<N>` type as a valid vector length.
/// See [`Vector`] for more information.
pub trait VecLen {
    /// The length value as an enum.
    const ENUM: VecLenEnum;

    /// The inner type contained inside `Vector<N, T, VecAligned>`.
    ///
    /// This redirects to `T::InnerAlignedVec{N}`,
    /// for example `T::InnerAlignedVec2` for `Usize<2>`.
    type InnerAlignedVector<T: Scalar>: Construct;
}

/// An enum with all currently supported vector lengths.
///
/// The enum value of a generic `const N: usize` value can be accessed with [`<Usize<N> as VecLen>::ENUM`][`VecLen::ENUM`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VecLenEnum {
    /// `2`
    Two,
    /// `3`
    Three,
    /// `4`
    Four,
}

/// A trait that marks a type as a valid scalar type that can be used in a vector.
/// This trait is implemented for most primitive types, like `f32`, `f64`, `bool`, `usize`, etc.
///
/// # Implementing `Scalar`
///
/// When implementing `Scalar` you need to fill:
///
/// 1.
/// `InnerAlignedVec2`, `InnerAlignedVec3`, and `InnerAlignedVec4`
///
/// These are the inner types stored inside `VecAligned` vectors,
/// for example `Vector<3, f32, VecAligned>` is stored as `f32::InnerAlignedVec3`.
///
/// The reference of these types MUST be transmutable to `&[T; N]`,
/// if its not then using that vector is undefined behavior.
/// This means that you cannot do things like expand `Vec3<bool>` into a 128-bit SIMD register with 32-bit lanes.
///
/// 2.
/// `GARBAGE`, `INNER_ALIGNED_VEC2_GARBAGE`, `INNER_ALIGNED_VEC3_GARBAGE`, and `INNER_ALIGNED_VEC4_GARBAGE`
///
/// These need to be any valid value of `Self`, `Self::InnerAlignedVec2`, `Self::InnerAlignedVec3`, and `Self::InnerAlignedVec4`.
/// This is used to properly initialize aligned vectors.
///
/// # Examples
///
/// ```ignore
/// struct BigInt {
///     // private fields
/// }
///
/// // impl Add, Sub... for BigInt
///
/// // lets say BigInt cannot benefit from SIMD operations, or we just don't want to optimize it yet.
/// // When not wanting SIMD we can fill `InnerAlignedVec{N}` with `[Self; N]`.
/// impl Scalar for BigInt {
///     type InnerAlignedVec2 = [Self; 2];
///     type InnerAlignedVec3 = [Self; 3];
///     type InnerAlignedVec4 = [Self; 4];
///
///     const GARBAGE: Self = Self::ZERO;
///     const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = [Self::ZERO; 2];
///     const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = [Self::ZERO; 3];
///     const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = [Self::ZERO; 4];
/// }
///
/// struct SmallInt(i32);
///
/// // impl Add, Sub... for SmallInt
///
/// // lets say SmallInt can benefit from SIMD operations.
/// impl Scalar for SmallInt {
///     // use x86_64 simd types for aligned vectors.
///     type InnerAlignedVec2 = core::arch::x86_64::__m128i;
///     type InnerAlignedVec3 = core::arch::x86_64::__m128i;
///     type InnerAlignedVec4 = core::arch::x86_64::__m128i;
///
///     const GARBAGE: Self = Self(0);
///     const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = unsafe { core::mem::zeroed() };
///     const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = unsafe { core::mem::zeroed() };
///     const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = unsafe { core::mem::zeroed() };
/// }
/// ```
pub trait Scalar: Construct {
    /// The inner type of `Vector<2, Self, VecAligned>`.
    /// This type's reference MUST be transmutable to `[Self; 2]`.
    /// Not following this rule will cause undefined behavior when using the vector.
    type InnerAlignedVec2: Construct;
    /// The inner type of `Vector<3, Self, VecAligned>`.
    /// This type's reference MUST be transmutable to `[Self; 3]`.
    /// Not following this rule will cause undefined behavior when using the vector.
    type InnerAlignedVec3: Construct;
    /// The inner type of `Vector<4, Self, VecAligned>`.
    /// This type's reference MUST be transmutable to `[Self; 4]`.
    /// Not following this rule will cause undefined behavior when using the vector.
    type InnerAlignedVec4: Construct;

    /// Any valid value of `Self`.
    /// This is sometimes used to create temporary values.
    const GARBAGE: Self;
    /// Any valid value of `Self::InnerAlignedVec2`.
    /// This is used to properly initialize aligned vector padding.
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2;
    /// Any valid value of `Self::InnerAlignedVec3`.
    /// This is used to properly initialize aligned vector padding.
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3;
    /// Any valid value of `Self::InnerAlignedVec4`.
    /// This is used to properly initialize aligned vector padding.
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4;

    /// Overridable implementation of `Vector::abs_diff`.
    #[inline(always)]
    fn vec_abs_diff<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd + Sub<Output = Self>,
    {
        Vector::from_fn(|i| {
            if vec[i] < other[i] {
                other[i] - vec[i]
            } else {
                vec[i] - other[i]
            }
        })
    }
}

/// See [`Vector`] for information.
pub trait VecAlignment: 'static {
    /// Whether or not the vector is SIMD aligned.
    const IS_ALIGNED: bool;

    /// The inner type contained inside [`Vector`].
    ///
    /// For `VecAligned` vectors this is `T::InnerAlignedVec{N}`,
    /// for example `T::InnerAlignedVec2` for `Vec2`.
    ///
    /// For `VecPacked` vectors this is `[T; N]`,
    /// for example `[T; 2]` for `Vec2`.
    type InnerVector<const N: usize, T: Scalar>: Construct
    where
        Usize<N>: VecLen;
}

/// See [`Vector`] for information.
pub struct VecAligned;

/// See [`Vector`] for information.
pub struct VecPacked;

/// Creates a new vec2 where each component is the same value.
#[inline(always)]
pub const fn splat2<T: Scalar>(value: T) -> Vector<2, T, VecAligned> {
    Vector::<2, T, VecAligned>::splat(value)
}

/// Creates a new vec3 where each component is the same value.
#[inline(always)]
pub const fn splat3<T: Scalar>(value: T) -> Vector<3, T, VecAligned> {
    Vector::<3, T, VecAligned>::splat(value)
}

/// Creates a new vec4 where each component is the same value.
#[inline(always)]
pub const fn splat4<T: Scalar>(value: T) -> Vector<4, T, VecAligned> {
    Vector::<4, T, VecAligned>::splat(value)
}

/// Creates a new `VecPacked` vec2 where each component is the same value.
#[inline(always)]
pub const fn splat2p<T: Scalar>(value: T) -> Vector<2, T, VecPacked> {
    Vector::<2, T, VecPacked>::splat(value)
}

/// Creates a new `VecPacked` vec3 where each component is the same value.
#[inline(always)]
pub const fn splat3p<T: Scalar>(value: T) -> Vector<3, T, VecPacked> {
    Vector::<3, T, VecPacked>::splat(value)
}

/// Creates a new `VecPacked` vec4 where each component is the same value.
#[inline(always)]
pub const fn splat4p<T: Scalar>(value: T) -> Vector<4, T, VecPacked> {
    Vector::<4, T, VecPacked>::splat(value)
}

/// Creates a new vec2 where each component is the same value,
/// where type inference can be used to determine if it's aligned or packed.
#[inline(always)]
pub const fn splat2g<T: Scalar, A: VecAlignment>(value: T) -> Vector<2, T, A> {
    Vector::<2, T, A>::splat(value)
}

/// Creates a new vec3 where each component is the same value,
/// where type inference can be used to determine if it's aligned or packed.
#[inline(always)]
pub const fn splat3g<T: Scalar, A: VecAlignment>(value: T) -> Vector<3, T, A> {
    Vector::<3, T, A>::splat(value)
}

/// Creates a new vec4 where each component is the same value,
/// where type inference can be used to determine if it's aligned or packed.
#[inline(always)]
pub const fn splat4g<T: Scalar, A: VecAlignment>(value: T) -> Vector<4, T, A> {
    Vector::<4, T, A>::splat(value)
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
        (0..N).all(|i| self[i] == other[i])
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

impl<const N: usize, T: Scalar, A: VecAlignment> IntoIterator for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.to_array().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::Construct;

    use super::*;

    fn _verify_construct_impl<const N: usize, T: Scalar, A: VecAlignment>()
    where
        Usize<N>: VecLen,
    {
        fn helper<T: Construct>() {}

        helper::<Vector<N, T, A>>();
    }
}
