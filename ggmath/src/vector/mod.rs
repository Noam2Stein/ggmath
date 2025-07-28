//! Staticly-lengthed vectors of [scalars](scalar) with lengths between 2 and 4.

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
/// ### When to reference this type?
///
/// If you want to reference a specific vector type, use these type aliases:
/// [`Vec2<T>`], [`Vec3<T>`], [`Vec4<T>`], [`Vec2P<T>`], [`Vec3P<T>`], [`Vec4P<T>`].
/// Or with the `primitive_aliases` feature (enabled by default), use aliases like:
/// [`FVec2`][f32_aliases::FVec2], [`BVec4`][bool_aliases::BVec4], [`U16Vec3`][u16_aliases::U16Vec3]...
///
/// If you want to be generic over `N` / `A`, use this type.
///
/// If you want to impl a trait for all vectors, use this type and write:
/// ```
/// use ggmath::vector::*;
///
/// trait MyTrait {
///     fn gg(self) -> bool;
/// }
///
/// impl<const N: usize, T: Scalar, A: VecAlignment> MyTrait for Vector<N, T, A>
/// where
///     Usize<N>: VecLen,
/// {
///     fn gg(self) -> bool {
///         true
///     }
/// }
/// ```
///
/// # Examples
/// ```
/// fn print_vector<const N: usize>(vec: Vector<N, impl Scalar, impl VecAlignment>)
/// where
///     Usize<N>: VecLen, // Required by Vector to ensure that N is either 2, 3, or 4.
/// {
///     println!("{vec}")
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
