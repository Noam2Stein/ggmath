//! Vector related types and traits

use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use crate::Usize;

mod constructor;
mod dir;
#[path = "../generated/vector/mod.rs"]
mod generated;
mod interface;
mod primitives;
mod scalar;
mod vec_alignment;
mod vec_len;
pub use dir::*;
#[allow(unused_imports)]
pub use generated::*;
pub use scalar::*;
pub use vec_alignment::*;
pub use vec_len::*;

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
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    /// The inner value that contains the actual vector data.
    ///
    /// For `VecAligned` vectors this will be `<T as Scalar>::InnerAlignedVec{N}`,
    /// like `<T as Scalar>::InnerAlignedVec2` for `Vec2`.
    ///
    /// For `VecPacked` vectors this will always be `[T; N]`.
    ///
    /// This field is public so that `Scalar` implementations that override vector function implementations
    /// can access the inner value.
    pub inner: A::InnerVector<N, T>,
}

/// Creates a new aligned vec2 where each component is the same value.
#[inline(always)]
pub const fn splat2<T: Scalar>(value: T) -> Vector<2, T, VecPacked> {
    Vector::<2, T, VecPacked>::splat(value)
}

/// Creates a new aligned vec3 where each component is the same value.
#[inline(always)]
pub const fn splat3<T: Scalar>(value: T) -> Vector<3, T, VecPacked> {
    Vector::<3, T, VecPacked>::splat(value)
}

/// Creates a new aligned vec4 where each component is the same value.
#[inline(always)]
pub const fn splat4<T: Scalar>(value: T) -> Vector<4, T, VecPacked> {
    Vector::<4, T, VecPacked>::splat(value)
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

/// **TODO: Write a good doc comment for this macro.**
#[macro_export]
macro_rules! vector_optimization {
    (
        ($($arg:ident => $arg_type_b:ty),* $(,)?) -> $out_type_a:ty => $out_type_b:ty: $($closure_tt:tt)*
    ) => {{
        fn _vector_optimization_helper<T: Copy + 'static>(_: T) -> core::any::TypeId {
            core::any::TypeId::of::<T>()
        }

        if $(core::any::TypeId::of::<$arg_type_b>() == _vector_optimization_helper($arg))&&* {
            let closure: fn($($arg_type_b),*) -> $out_type_a = { $($closure_tt)* };

            return unsafe {
                std::mem::transmute_copy::<$out_type_a, $out_type_b>(
                    &closure($(std::mem::transmute_copy::<_, $arg_type_b>(&$arg)),*)
                )
            };
        }
    }};
}
