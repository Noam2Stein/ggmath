//! Trait for types that can be put inside math-types like ```Vector``` and ```Matrix```.
//! For example: [```f32```], [```u8```] and [```bool```] are scalars.

use super::*;

mod positive_dir;
pub use positive_dir::*;

/// Trait for types that can be put inside math-types like ```Vector``` and ```Matrix```.
/// For example: [```f32```], [```u8```] and [```bool```] are scalars.
///
/// All scalar types must be `Construct`,
/// which means `Copy` `'static` and that the value can be sent anywhere anytime.
///
/// References and vectors are NOT scalars.
///
/// ### Implementing this trait
///
/// When implementing this trait, you need to fill out the [`VecAligned`] alignments for your type.
///
/// ```
/// impl Scalar for f32 {
///     type Vec2Alignment = Align<8>;
///     type Vec3Alignment = Align<16>;
///     type Vec4Alignment = Align<16>;
/// }
/// ```
///
/// `ggmath` currently trusts that rustc will use the alignment to correctly use SIMD instructions.
/// This means you currently cannot override the implementations of built in functions like operators.
pub trait Scalar: Construct {
    /// Controls the alignment of `Vec2<Self>`.
    /// This will be the applied alignment only if the vector type is `VecAligned`.
    type Vec2Alignment: AlignTrait;

    /// Controls the alignment of `Vec3<Self>`.
    /// This will be the applied alignment only if the vector type is `VecAligned`.
    type Vec3Alignment: AlignTrait;

    /// Controls the alignment of `Vec4<Self>`.
    /// This will be the applied alignment only if the vector type is `VecAligned`.
    type Vec4Alignment: AlignTrait;

    /// Specifies if `neg` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `Neg` implementation,
    /// which will allow the compiler to optimize the vector operator implementation.
    const NEG_GARBAGE: Option<fn(Self) -> Self> = None;

    /// Specifies if `not` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `Not` implementation,
    /// which will allow the compiler to optimize the vector operator implementation.
    const NOT_GARBAGE: Option<fn(Self) -> Self> = None;

    /// Specifies if `add` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `Add` implementation,
    const ADD_GARBAGE: Option<fn(Self, Self) -> Self> = None;

    /// Specifies if `sub` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `Sub` implementation,
    const SUB_GARBAGE: Option<fn(Self, Self) -> Self> = None;

    /// Specifies if `mul` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `Mul` implementation,
    const MUL_GARBAGE: Option<fn(Self, Self) -> Self> = None;

    /// Specifies if `div` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `Div` implementation,
    const DIV_GARBAGE: Option<fn(Self, Self) -> Self> = None;

    /// Specifies if `rem` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `Rem` implementation,
    const REM_GARBAGE: Option<fn(Self, Self) -> Self> = None;

    /// Specifies if `bitand` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `BitAnd` implementation,
    const BITAND_GARBAGE: Option<fn(Self, Self) -> Self> = None;

    /// Specifies if `bitor` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `BitOr` implementation,
    const BITOR_GARBAGE: Option<fn(Self, Self) -> Self> = None;

    /// Specifies if `bitxor` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `BitXor` implementation,
    const BITXOR_GARBAGE: Option<fn(Self, Self) -> Self> = None;

    /// Specifies if `shl` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `Shl` implementation,
    const SHL_GARBAGE: Option<fn(Self, Self) -> Self> = None;

    /// Specifies if `shr` can be applied to garbage values.
    /// The input to this function is NOT guaranteed to be a valid value.
    ///
    /// This is used to enable SIMD for vectors with padding, usually `Vec3`.
    ///
    /// This is only useful if the implementation is very similar to the `Shr` implementation,
    const SHR_GARBAGE: Option<fn(Self, Self) -> Self> = None;
}
