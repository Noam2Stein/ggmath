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
mod primitives;
pub use dir::*;
pub use primitives::*;

#[allow(unused_imports)]
pub use crate::generated::vector::*;

/// A generic vector type.
///
/// This type is generic over 3 parameters:
/// - `N`: The length of the vector, which currently supports 2, 3, and 4.
/// - `T`: The type of the vector, which must implement the [`Scalar`] trait.
/// - `A`: The "alignment" of the vector, which enables or disables SIMD memory alignment.
///
/// It is usually recommended to use vector type-aliases instead of the vector type directly.
/// `Vec{N}` type-aliases like `Vec2` are SIMD aligned and are considered the default.
/// `Vec{N}P` type-aliases like `Vec2P` are not SIMD aligned.
///
/// # Alignment
///
/// To understand alignment first see <https://doc.rust-lang.org/reference/type-layout.html>.
///
/// `A` must be either `VecAligned` or `VecPacked`.
/// A `VecAligned` vector is SIMD aligned while a `VecPacked` vector is not.
///
/// This means that `VecAligned` vectors result in faster operations than `VecPacked` vectors,
/// but `VecAligned` vectors have a larger size than `VecPacked` vectors.
/// When using a vector you need to choose between `VecAligned` and `VecPacked` based on priorities.
///
/// * `VecAligned` does not guarentee any specific alignment.
/// The alignment depends on the target architecture and the implementation of [`Scalar`].
///
/// * `VecPacked` guarentees that the vector has the exact same memory layout as `[T; N]`.
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
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>(
    /// The inner value that contains the actual vector data.
    ///
    /// For `VecAligned` vectors this will be `<T as Scalar>::InnerAlignedVec{N}`,
    /// like `<T as Scalar>::InnerAlignedVec2` for `Vec2`.
    ///
    /// For `VecPacked` vectors this will always be `[T; N]`.
    ///
    /// This field is public so that `Scalar` implementations that override vector function implementations
    /// can access the inner value.
    pub A::InnerVector<N, T>,
)
where
    Usize<N>: VecLen;

/// A trait that marks a `Usize<N>` type as a valid vector length.
pub trait VecLen {
    /// The length value as an enum.
    const ENUM: VecLenEnum;

    /// The inner aligned vector type which will be:
    /// - `T::InnerAlignedVec2` for `Usize<2>`
    /// - `T::InnerAlignedVec3` for `Usize<3>`
    /// - `T::InnerAlignedVec4` for `Usize<4>`
    type InnerAlignedVector<T: Scalar>: Construct;
}

/// An enum with all currently supported vector lengths.
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
/// - `InnerAlignedVec2`, `InnerAlignedVec3`, and `InnerAlignedVec4`
/// are the inner types inside `VecAligned` vectors.
/// Their reference MUST be transmutable to `&[T; N]`,
/// if its not then using that vector is undefined behavior.
///
/// - `GARBAGE`, `INNER_ALIGNED_VEC2_GARBAGE`, `INNER_ALIGNED_VEC3_GARBAGE`, and `INNER_ALIGNED_VEC4_GARBAGE`
/// need to be any valid value of `Self`, `Self::InnerAlignedVec2`, `Self::InnerAlignedVec3`, or `Self::InnerAlignedVec4` respectively.
///
/// # Examples
///
/// ```ignore
/// struct HeapInt {
///     // private fields
/// }
///
/// // impl Add, Sub... for HeapInt
///
/// // lets say HeapInt cannot benefit from SIMD operations, or we just don't want to optimize yet.
/// // When not wanting SIMD we can fill `InnerAlignedVec{N}` with `[Self; N]`.
/// impl Scalar for HeapInt {
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
/// struct MyScalar(i32);
///
/// // impl Add, Sub... for MyScalar
///
/// // lets say MyScalar can benefit from SIMD operations.
/// impl Scalar for MyScalar {
///     // use x86_64 simd types for aligned vectors.
///     type InnerAlignedVec2 = core::arch::x86_64::__m128i;
///     type InnerAlignedVec3 = core::arch::x86_64::__m128i;
///     type InnerAlignedVec4 = core::arch::x86_64::__m128i;
///
///     const GARBAGE: Self = Self(0);
///     const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = core::arch::x86_64::_mm_setzero_si128();
///     const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = core::arch::x86_64::_mm_setzero_si128();
///     const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = core::arch::x86_64::_mm_setzero_si128();
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

/// `Vector` is generic over `A: VecAlignment`,
/// which specifies if the vector is SIMD aligned or not.
///
/// This trait is implemented for `VecAligned` and `VecPacked`.
/// - `VecAligned` marks the vector as SIMD aligned.
/// - `VecPacked` marks the vector as not SIMD aligned.
///
/// See [`Vector`] alignment for more information.
pub trait VecAlignment: 'static {
    /// Whether the vector is SIMD aligned.
    const IS_ALIGNED: bool;

    /// The inner vector type based on `N` and `T`.
    /// This is the actual type that is stored in vectors.
    ///
    /// - `VecAligned` sets this to `T::InnerAlignedVec2` / `T::InnerAlignedVec3` / `T::InnerAlignedVec4`.
    /// - `VecPacked` sets this to `[T; N]`.
    type InnerVector<const N: usize, T: Scalar>: Construct
    where
        Usize<N>: VecLen;
}

/// Marks a `Vector` as SIMD aligned.
///
/// `Vector` can be marked as either `VecAligned` or `VecPacked`.
///
/// A `VecAligned` vector is aligned for optimal SIMD operations.
/// The exact type stored in aligned vectors is specified in the implementation of [`Scalar`].
///
/// A `VecPacked` vector is not aligned for SIMD operations.
/// It is guarenteed that a packed vector has the memory layout of `[T; N]`.
///
/// See [`Vector`] alignment for more information.
pub struct VecAligned;

/// Marks a `Vector` as not SIMD aligned.
///
/// `Vector` can be marked as either `VecAligned` or `VecPacked`.
///
/// A `VecAligned` vector is aligned for optimal SIMD operations.
/// The exact type stored in aligned vectors is specified in the implementation of [`Scalar`].
///
/// A `VecPacked` vector is not aligned for SIMD operations.
/// It is guarenteed that a packed vector has the memory layout of `[T; N]`.
///
/// See [`Vector`] alignment for more information.
pub struct VecPacked;

/// Creates a new aligned vec2 where each component is the same value.
#[inline(always)]
pub const fn splat2<T: Scalar>(value: T) -> Vector<2, T, VecAligned> {
    Vector::<2, T, VecAligned>::splat(value)
}

/// Creates a new aligned vec3 where each component is the same value.
#[inline(always)]
pub const fn splat3<T: Scalar>(value: T) -> Vector<3, T, VecAligned> {
    Vector::<3, T, VecAligned>::splat(value)
}

/// Creates a new aligned vec4 where each component is the same value.
#[inline(always)]
pub const fn splat4<T: Scalar>(value: T) -> Vector<4, T, VecAligned> {
    Vector::<4, T, VecAligned>::splat(value)
}

/// Creates a new packed vec2 where each component is the same value.
#[inline(always)]
pub const fn splat2p<T: Scalar>(value: T) -> Vector<2, T, VecPacked> {
    Vector::<2, T, VecPacked>::splat(value)
}

/// Creates a new packed vec3 where each component is the same value.
#[inline(always)]
pub const fn splat3p<T: Scalar>(value: T) -> Vector<3, T, VecPacked> {
    Vector::<3, T, VecPacked>::splat(value)
}

/// Creates a new packed vec4 where each component is the same value.
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
