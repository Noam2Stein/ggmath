//! Staticly-lengthed vectors of [scalars](scalar) with lengths between 2 and 4.

pub mod alignment;
pub mod inner;
pub mod length;

pub(crate) mod interfaces;

mod api;
pub use api::*;

mod impl_std;

use crate::{construct::*, scalar::*};
use alignment::*;
use length::*;

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
