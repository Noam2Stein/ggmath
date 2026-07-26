use core::{
    fmt::Debug,
    iter::{Product, Sum},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub},
};

use crate::{
    Aligned, NegOne, One, Scalar, Unaligned, Zero,
    backend::{FloatVectorBackend, IntegerVectorBackend, SignedVectorBackend},
    utils::{FloatUtils, PrimitiveFloatUtils, PrimitiveIntegerUtils, PrimitiveSignedUtils},
};

/// A trait for all primitive floating-point types.
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
    + PrimitiveFloatUtils<Bits = <Self as PrimitiveFloat>::Bits>
    + FloatVectorBackend<2, Aligned>
    + FloatVectorBackend<3, Aligned>
    + FloatVectorBackend<4, Aligned>
    + FloatVectorBackend<2, Unaligned>
    + FloatVectorBackend<3, Unaligned>
    + FloatVectorBackend<4, Unaligned>
    + FloatUtils
    + num_primitive::PrimitiveFloat<Bits = <Self as PrimitiveFloat>::Bits>
{
    /// The unsigned integer type with equal width.
    type Bits: PrimitiveUnsigned;
}

/// A trait for all primitive integer types.
///
/// Implemented for [`i8`], [`i16`], [`i32`], [`i64`], [`i128`], [`isize`],
/// [`u8`], [`u16`], [`u32`], [`u64`], [`u128`] and [`usize`].
///
/// This trait can be used in generic contexts to access integer-specific
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
/// # use ggmath::{PrimitiveInteger, Vec3};
/// #
/// fn example<T: PrimitiveInteger>(vector: Vec3<T>) -> Vec3<T> {
///     vector.wrapping_add(vector)
/// }
///
/// assert_eq!(
///     example::<i32>(Vec3::new(1, 2, 3)),
///     Vec3::new(2, 4, 6),
/// );
/// ```
#[expect(private_bounds)]
pub trait PrimitiveInteger:
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
    + PrimitiveIntegerUtils
    + IntegerVectorBackend<2, Aligned>
    + IntegerVectorBackend<3, Aligned>
    + IntegerVectorBackend<4, Aligned>
    + IntegerVectorBackend<2, Unaligned>
    + IntegerVectorBackend<3, Unaligned>
    + IntegerVectorBackend<4, Unaligned>
    + num_primitive::PrimitiveInteger
{
}

/// A trait for all primitive signed integer types.
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
    + PrimitiveInteger
    + Neg<Output = Self>
    + NegOne
    + PrimitiveSignedUtils<Unsigned = <Self as PrimitiveSigned>::Unsigned>
    + SignedVectorBackend<2, Aligned>
    + SignedVectorBackend<3, Aligned>
    + SignedVectorBackend<4, Aligned>
    + SignedVectorBackend<2, Unaligned>
    + SignedVectorBackend<3, Unaligned>
    + SignedVectorBackend<4, Unaligned>
    + num_primitive::PrimitiveSigned<Unsigned = <Self as PrimitiveSigned>::Unsigned>
{
    /// The unsigned integer type with equal width.
    type Unsigned: PrimitiveUnsigned;
}

/// A trait for all primitive unsigned integer types.
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
    + PrimitiveInteger
    + num_primitive::PrimitiveUnsigned<Signed = <Self as PrimitiveUnsigned>::Signed>
{
    /// The signed integer type with equal width.
    type Signed: PrimitiveSigned;
}

impl PrimitiveFloat for f32 {
    type Bits = u32;
}
impl PrimitiveFloat for f64 {
    type Bits = u64;
}

impl PrimitiveInteger for i8 {}
impl PrimitiveInteger for i16 {}
impl PrimitiveInteger for i32 {}
impl PrimitiveInteger for i64 {}
impl PrimitiveInteger for i128 {}
impl PrimitiveInteger for isize {}
impl PrimitiveInteger for u8 {}
impl PrimitiveInteger for u16 {}
impl PrimitiveInteger for u32 {}
impl PrimitiveInteger for u64 {}
impl PrimitiveInteger for u128 {}
impl PrimitiveInteger for usize {}

impl PrimitiveSigned for i8 {
    type Unsigned = u8;
}
impl PrimitiveSigned for i16 {
    type Unsigned = u16;
}
impl PrimitiveSigned for i32 {
    type Unsigned = u32;
}
impl PrimitiveSigned for i64 {
    type Unsigned = u64;
}
impl PrimitiveSigned for i128 {
    type Unsigned = u128;
}
impl PrimitiveSigned for isize {
    type Unsigned = usize;
}

impl PrimitiveUnsigned for u8 {
    type Signed = i8;
}
impl PrimitiveUnsigned for u16 {
    type Signed = i16;
}
impl PrimitiveUnsigned for u32 {
    type Signed = i32;
}
impl PrimitiveUnsigned for u64 {
    type Signed = i64;
}
impl PrimitiveUnsigned for u128 {
    type Signed = i128;
}
impl PrimitiveUnsigned for usize {
    type Signed = isize;
}

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

/// A module that contains dummy traits for when the `num-primitive` feature
/// flag is disabled.
///
/// This will not be necessary once cfg on trait bounds is supported.
#[cfg(not(feature = "num-primitive"))]
mod num_primitive {
    pub trait PrimitiveFloat {
        type Bits;
    }

    pub trait PrimitiveInteger {}

    pub trait PrimitiveSigned {
        type Unsigned;
    }

    pub trait PrimitiveUnsigned {
        type Signed;
    }

    impl PrimitiveFloat for f32 {
        type Bits = u32;
    }
    impl PrimitiveFloat for f64 {
        type Bits = u64;
    }

    impl<T> PrimitiveInteger for T {}

    impl PrimitiveSigned for i8 {
        type Unsigned = u8;
    }
    impl PrimitiveSigned for i16 {
        type Unsigned = u16;
    }
    impl PrimitiveSigned for i32 {
        type Unsigned = u32;
    }
    impl PrimitiveSigned for i64 {
        type Unsigned = u64;
    }
    impl PrimitiveSigned for i128 {
        type Unsigned = u128;
    }
    impl PrimitiveSigned for isize {
        type Unsigned = usize;
    }

    impl PrimitiveUnsigned for u8 {
        type Signed = i8;
    }
    impl PrimitiveUnsigned for u16 {
        type Signed = i16;
    }
    impl PrimitiveUnsigned for u32 {
        type Signed = i32;
    }
    impl PrimitiveUnsigned for u64 {
        type Signed = i64;
    }
    impl PrimitiveUnsigned for u128 {
        type Signed = i128;
    }
    impl PrimitiveUnsigned for usize {
        type Signed = isize;
    }
}
