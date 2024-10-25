//! Staticly-lengthed vectors of [scalars](scalar) with lengths between 2 and 4.

use crate::{construct::*, scalar::*};

mod alignment;
mod len;
pub use alignment::*;
pub use len::*;

pub mod inner;
pub mod or_scalar;

pub(crate) mod interfaces;

mod impl_construct;

/// Statically-lengthed vector generic over N - length, T - Scalar, and A - Alignment.
///
/// Storage affects the inner implementation of vector fns.
///
/// Only use this type when being generic over N, T, and S.
/// there are simpler type aliases to this type for when being not generic.
///
/// - ```Vector2<T, S>```, ```Vector3<T, S>```, and ```Vector4<T, S>``` fill N.
/// - ```VecN<N, T>```, ```Vec2<T>```, ```Vec3<T>```, and ```Vec4<T>``` use the default storage [```VecAligned```].
/// - ```VecNP<N, T>```, ```Vec2P<T>```, ```Vec3P<T>```, and ```Vec4P<T>``` use the non-default storage [```VecPacked```].
/// - [```scalar::aliases```](crate::scalar::aliases) contains aliases for each primitive.
///
/// # Examples
/// ```
/// fn print_vec<const N: usize, T: Scalar, S: VecStorage>(vec: Vector<N, T, S>)
/// where
///     ScalarCount<N>: VecLen<N>, // Required by Vector to ensure that N is either 2, 3, or 4.
/// {
///     println!("{vec}")
/// }
/// ```
#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, A: seal::VecAlignment>
where
    ScalarCount<N>: VecLen<N>,
{
    inner: inner::InnerVector<N, T, A>,
}

/// type alias to [```Vector```]```<2, T, S>```
pub type Vector2<T, A> = Vector<2, T, A>;

/// type alias to [```Vector```]```<3, T, S>```
pub type Vector3<T, A> = Vector<3, T, A>;

/// type alias to [```Vector```]```<4, T, S>```
pub type Vector4<T, A> = Vector<4, T, A>;

/// Statically-lengthed vector generic over N - length, and T - Scalar.
/// - type alias to [```Vector```]```<N, T, VecAligned>```
pub type VecN<const N: usize, T> = Vector<N, T, VecAligned>;

/// type alias to [```Vector```]```<2, T, VecAligned>```
pub type Vec2<T> = VecN<2, T>;

/// type alias to [```Vector```]```<3, T, VecAligned>```
pub type Vec3<T> = VecN<3, T>;

/// type alias to [```Vector```]```<4, T, VecAligned>```
pub type Vec4<T> = VecN<4, T>;

/// Statically-lengthed vector generic over N - length, and T - Scalar.
/// - type alias to [```Vector```]```<N, T, VecPacked>```
/// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```VecN```].
pub type VecNP<const N: usize, T> = Vector<N, T, VecPacked>;

/// - type alias to [```Vector```]```<2, T, VecPacked>```
/// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec2```].
pub type Vec2P<T> = VecNP<2, T>;

/// - type alias to [```Vector```]```<3, T, VecPacked>```
/// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec3```].
pub type Vec3P<T> = VecNP<3, T>;

/// - type alias to [```Vector```]```<4, T, VecPacked>```
/// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec4```].
pub type Vec4P<T> = VecNP<4, T>;
