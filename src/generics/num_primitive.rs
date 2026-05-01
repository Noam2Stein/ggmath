use core::{
    fmt::Debug,
    iter::{Product, Sum},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub},
};

use crate::{
    Aligned, PrimitiveFloatBackend, PrimitiveSignedBackend, PrimitiveUnsignedBackend, Scalar,
    Unaligned,
    constants::{Infinity, Max, Min, Nan, NegInfinity, NegOne, One, Zero},
    utils::{PrimitiveFloatFns, PrimitiveSignedFns, PrimitiveUnsignedFns},
};

/// Trait for all primitive floating-point types.
///
/// Implemented for [`f32`] and [`f64`].
///
/// This trait can be used in generic contexts to access float-specific
/// functionality for vectors, matrices, etc. This trait does not expose any
/// functions directly.
///
/// # Examples
///
/// ```
/// # use ggmath::{PrimitiveFloat, Vec3};
/// #
/// fn example<T: PrimitiveFloat>(vector: Vec3<T>) -> Vec3<T> {
///     vector.normalize()
/// }
///
/// assert_eq!(
///     example::<f32>(Vec3::new(2.0, 0.0, 0.0)),
///     Vec3::new(1.0, 0.0, 0.0),
/// );
/// ```
#[expect(private_bounds)]
pub trait PrimitiveFloat:
    Sealed
    + Debug
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Sum
    + Product
    + Scalar
    + Zero
    + One
    + NegOne
    + Min
    + Max
    + Nan
    + Infinity
    + NegInfinity
    + PrimitiveFloatFns
    + PrimitiveFloatBackend<2, Aligned>
    + PrimitiveFloatBackend<3, Aligned>
    + PrimitiveFloatBackend<4, Aligned>
    + PrimitiveFloatBackend<2, Unaligned>
    + PrimitiveFloatBackend<3, Unaligned>
    + PrimitiveFloatBackend<4, Unaligned>
{
}

/// Trait for all primitive signed integer types.
///
/// Implemented for [`i8`], [`i16`], [`i32`], [`i64`], [`i128`] and [`isize`].
///
/// This trait can be used in generic contexts to access signed-integer-specific
/// functionality for vectors, matrices, etc. This trait does not expose any
/// functions directly.
///
/// Currently many integer functions cannot be used in generic contexts because
/// their names conflict with floating-point functions. When the type system
/// allows this, all functions will be available.
///
/// # Examples
///
/// ```
/// # use ggmath::{PrimitiveSigned, Vec3};
/// #
/// fn example<T: PrimitiveSigned>(vector: Vec3<T>) -> Vec3<T> {
///     vector.wrapping_add(vector)
/// }
///
/// assert_eq!(
///     example::<i32>(Vec3::new(1, 2, 3)),
///     Vec3::new(2, 4, 6),
/// );
/// ```
#[expect(private_bounds)]
pub trait PrimitiveSigned:
    Sealed
    + Debug
    + Clone
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shl<Output = Self>
    + Shr<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Sum
    + Product
    + Scalar
    + Zero
    + One
    + NegOne
    + Min
    + Max
    + PrimitiveSignedFns
    + PrimitiveSignedBackend<2, Aligned>
    + PrimitiveSignedBackend<3, Aligned>
    + PrimitiveSignedBackend<4, Aligned>
    + PrimitiveSignedBackend<2, Unaligned>
    + PrimitiveSignedBackend<3, Unaligned>
    + PrimitiveSignedBackend<4, Unaligned>
{
}

/// Trait for all primitive unsigned integer types.
///
/// Implemented for [`u8`], [`u16`], [`u32`], [`u64`], [`u128`] and [`usize`].
///
/// This trait can be used in generic contexts to access
/// unsigned-integer-specific functionality for vectors, matrices, etc. This
/// trait does not expose any functions directly.
///
/// Currently many integer functions cannot be used in generic contexts because
/// their names conflict with floating-point functions. When the type system
/// allows this, all functions will be available.
#[expect(private_bounds)]
pub trait PrimitiveUnsigned:
    Sealed
    + Debug
    + Clone
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shl<Output = Self>
    + Shr<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Sum
    + Product
    + Scalar
    + Zero
    + One
    + Min
    + Max
    + PrimitiveUnsignedFns
    + PrimitiveUnsignedBackend<2, Aligned>
    + PrimitiveUnsignedBackend<3, Aligned>
    + PrimitiveUnsignedBackend<4, Aligned>
    + PrimitiveUnsignedBackend<2, Unaligned>
    + PrimitiveUnsignedBackend<3, Unaligned>
    + PrimitiveUnsignedBackend<4, Unaligned>
{
}

impl PrimitiveFloat for f32 {}
impl PrimitiveFloat for f64 {}

impl PrimitiveSigned for i8 {}
impl PrimitiveSigned for i16 {}
impl PrimitiveSigned for i32 {}
impl PrimitiveSigned for i64 {}
impl PrimitiveSigned for i128 {}
impl PrimitiveSigned for isize {}

impl PrimitiveUnsigned for u8 {}
impl PrimitiveUnsigned for u16 {}
impl PrimitiveUnsigned for u32 {}
impl PrimitiveUnsigned for u64 {}
impl PrimitiveUnsigned for u128 {}
impl PrimitiveUnsigned for usize {}

trait Sealed {}

impl Sealed for f32 {}
impl Sealed for f64 {}
impl Sealed for i8 {}
impl Sealed for i16 {}
impl Sealed for i32 {}
impl Sealed for i64 {}
impl Sealed for i128 {}
impl Sealed for isize {}
impl Sealed for u8 {}
impl Sealed for u16 {}
impl Sealed for u32 {}
impl Sealed for u64 {}
impl Sealed for u128 {}
impl Sealed for usize {}
