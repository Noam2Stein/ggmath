//! Staticly-lengthed vectors of [scalars](scalar) with lengths between 2 and 4.

use crate::{construct::*, scalar::*, *};

mod core_api;
mod ext_api;
mod generics;
mod impl_std;
pub use core_api::*;
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
///     ScalarCount<N>: VecLen,
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
///     ScalarCount<N>: VecLen, // Required by Vector to ensure that N is either 2, 3, or 4.
/// {
///     println!("{vec}")
/// }
/// ```
#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>(pub A::InnerVector<N, T>)
where
    ScalarCount<N>: VecLen;

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

pub use ggmath_proc_macros::{vec2, vec2p, vec3, vec3p, vec4, vec4p, vector_aliases};

#[cfg(feature = "primitive_aliases")]
mod primitive_aliases {
    use super::*;

    vector_aliases!(pub mod f32_aliases for f32(F));
    vector_aliases!(pub mod f64_aliases for f64(D));

    vector_aliases!(pub mod u8_aliases for u8(U8));
    vector_aliases!(pub mod u16_aliases for u16(U16));
    vector_aliases!(pub mod u32_aliases for u32(U));
    vector_aliases!(pub mod u64_aliases for u64(U64));
    vector_aliases!(pub mod u128_aliases for u128(U128));
    vector_aliases!(pub mod usize_aliases for usize(Usize));

    vector_aliases!(pub mod i8_aliases for i8(I8));
    vector_aliases!(pub mod i16_aliases for i16(I16));
    vector_aliases!(pub mod i32_aliases for i32(I));
    vector_aliases!(pub mod i64_aliases for i64(I64));
    vector_aliases!(pub mod i128_aliases for i128(I128));
    vector_aliases!(pub mod isize_aliases for isize(Isize));

    vector_aliases!(pub mod bool_aliases for bool(B));
}
#[cfg(feature = "primitive_aliases")]
pub use primitive_aliases::*;
