//! Staticly-lengthed vectors of [scalars](scalar) with lengths between 2 and 4.

pub mod alignment;
pub mod inner;
pub mod length;

pub(crate) mod interfaces;

mod api;
mod impl_std;
#[allow(unused_imports)]
pub use api::*;
#[allow(unused_imports)]
pub use impl_std::*;

use crate::{construct::*, ggmath, scalar::*};
use alignment::*;
use length::*;

pub use ggmath_proc_macros::vector_aliases;

#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod f32_aliases for f32(F));
#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod f64_aliases for f64(D));

#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod u8_aliases for u8(U8));
#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod u16_aliases for u16(U16));
#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod u32_aliases for u32(U));
#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod u64_aliases for u64(U64));
#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod u128_aliases for u128(U128));
#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod usize_aliases for usize(USize));

#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod i8_aliases for i8(I8));
#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod i16_aliases for i16(I16));
#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod i32_aliases for i32(I));
#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod i64_aliases for i64(I64));
#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod i128_aliases for i128(I128));
#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod isize_aliases for isize(ISize));

#[cfg(feature = "primitive_aliases")]
vector_aliases!(pub mod bool_aliases for bool(B));

/// Statically-lengthed vector generic over N - length, T - Scalar, and A - Alignment.
///
/// Storage affects the inner implementation of vector fns.
///
/// Only use this type when being generic over N, T, and A.
/// there are simpler type aliases to this type for when being not generic.
///
/// - ```Vector2<T, A>```, ```Vector3<T, A>```, and ```Vector4<T, A>``` fill N.
/// - ```VecN<N, T>```, ```Vec2<T>```, ```Vec3<T>```, and ```Vec4<T>``` use the default storage [```VecAligned```].
/// - ```VecNP<N, T>```, ```Vec2P<T>```, ```Vec3P<T>```, and ```Vec4P<T>``` use the non-default storage [```VecPacked```].
/// - [```scalar::aliases```](crate::scalar::aliases) contains aliases for each primitive.
///
/// # Examples
/// ```
/// fn print_vec<const N: usize, T: Scalar, A: VecStorage>(vec: Vector<N, T, A>)
/// where
///     ScalarCount<N>: VecLen<N>, // Required by Vector to ensure that N is either 2, 3, or 4.
/// {
///     println!("{vec}")
/// }
/// ```
#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, A: alignment_seal::VecAlignment>
where
    length::ScalarCount<N>: length::VecLen<N>,
{
    inner: inner::InnerVector<N, T, A>,
}

/// type alias to [```Vector```]```<2, T, VecAligned>```
pub type Vec2<T> = Vector<2, T, VecAligned>;

/// type alias to [```Vector```]```<3, T, VecAligned>```
pub type Vec3<T> = Vector<3, T, VecAligned>;

/// type alias to [```Vector```]```<4, T, VecAligned>```
pub type Vec4<T> = Vector<4, T, VecAligned>;

/// - type alias to [```Vector```]```<2, T, VecPacked>```
/// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec2```].
pub type Vec2P<T> = Vector<2, T, VecPacked>;

/// - type alias to [```Vector```]```<3, T, VecPacked>```
/// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec3```].
pub type Vec3P<T> = Vector<3, T, VecPacked>;

/// - type alias to [```Vector```]```<4, T, VecPacked>```
/// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec4```].
pub type Vec4P<T> = Vector<4, T, VecPacked>;
