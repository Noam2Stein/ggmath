#![deny(missing_docs)]

//! Vectors.

use std::mem::MaybeUninit;

use crate::{construct::*, scalar::*, *};

mod api_core;
mod api_ext;
mod generics;
mod impl_std;
pub use api_core::*;
use derive_where::derive_where;
pub use generics::*;

/// The only vector type.
/// Statically-lengthed vector generic over `N` (length), `T` (scalar type), and `A` (alignment??).
///
/// This type, like all `ggmath` types, is generic over `A: VecAlignment`.
/// The value of `A` decides whether or not the vector is aligned for SIMD.
///
/// `VecAligned` => aligned.
///
/// `VecPacked` => not aligned, identical in memory to `[T; N]`.
///
/// There are short type aliases for this type.
///
/// `Vec2<T>` => `Vector<2, T, VecAligned>`
/// `Vec3<T>` => `Vector<3, T, VecAligned>`
/// `Vec4<T>` => `Vector<4, T, VecAligned>`
///
/// `Vec2P<T>` => `Vector<2, T, VecPacked>`
/// `Vec3P<T>` => `Vector<3, T, VecPacked>`
/// `Vec4P<T>` => `Vector<4, T, VecPacked>`
///
/// There are also type aliases for specific types through the `primitive_aliases` feature which is enabled by default.
///
/// ### Impl Pattern
///
/// Because of the complex generics, this is how you make an impl block that applies to all vectors:
///
/// ```
/// impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
/// where
///     Usize<N>: VecLen,
/// {
/// }
/// ```
#[derive_where(Clone, Copy)]
#[derive_where(Eq, Hash; T)]
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    array: [T; N],
    #[derive_where(skip)]
    _align: A::Alignment<N, T>,
}

/// type alias to [```Vector<2, T, VecAligned>```]
pub type Vec2<T> = Vector<2, T, VecAligned>;

/// type alias to [```Vector<3, T, VecAligned>```]
pub type Vec3<T> = Vector<3, T, VecAligned>;

/// type alias to [```Vector<4, T, VecAligned>```]
pub type Vec4<T> = Vector<4, T, VecAligned>;

/// type alias to [```Vector<2, T, VecPacked>```]
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec2```].
pub type Vec2P<T> = Vector<2, T, VecPacked>;

/// type alias to [```Vector<3, T, VecPacked>```]
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec3```].
pub type Vec3P<T> = Vector<3, T, VecPacked>;

/// type alias to [```Vector<4, T, VecPacked>```]
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec4```].
pub type Vec4P<T> = Vector<4, T, VecPacked>;
