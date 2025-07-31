//! Scalar trait!

use super::*;

/// Trait for types that can be put inside math-types like `Vector` and `Matrix`.
/// For example: `f32`, `u8` and `bool` are scalars.
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

    /// Allows you to optimize the implementation of `Vec3<Self>::neg`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_neg(base: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = base;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::not`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_not(base: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = base;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::add`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_add(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::sub`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_sub(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::mul`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_mul(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::div`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_div(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::rem`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_rem(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::shl`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_shl(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::shr`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_shr(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::bitand`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_bitand(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::bitor`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_bitor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::bitxor`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_bitxor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }
}
