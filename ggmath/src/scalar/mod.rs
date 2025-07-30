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
/// You also need to specify the [`ScalarPadding`] for your type.
/// This controls how `ggmath` initializes the padding of a vector.
///
/// ```
/// impl Scalar for f32 {
///     type Vec2Alignment = Align<8>;
///     type Vec3Alignment = Align<16>;
///     type Vec4Alignment = Align<16>;
///
///     const PADDING: ScalarPadding<Self> = ScalarPadding::Init(0.0);
/// }
/// ```
///
/// `ggmath` currently trusts that rustc will use the alignment to correctly use SIMD instructions.
/// This means you currently cannot override the implementations of built in functions like operators..
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

    /// Specifies vector padding for this scalar type.
    ///
    /// The padding value in an aligned vector3 has to be a valid scalar.
    /// This is so that operator functions can use optimal SIMD instructions.
    ///
    /// Marking this as `Uninit` tells `ggmath` that ANY memory is valid padding.
    /// This lets initialization functions save a copy instruction because it doesn't have to initialize the padding.
    ///
    /// If NOT any memory is a valid padding value,
    /// use `ScalarPadding::Init(_)` which will tell `ggmath` to initialize the padding with the specified value.
    const PADDING: ScalarPadding<Self>;
}

/// Specifies vector padding for a scalar type.
///
/// The padding value in an aligned vector3 has to be a valid scalar.
/// This is so that operator functions can use optimal SIMD instructions.
///
/// Marking this as `Uninit` tells `ggmath` that ANY memory is valid padding.
/// This lets initialization functions save a copy instruction because it doesn't have to initialize the padding.
///
/// This value must not break operator functions.
/// It can produce any value, even NaN, but it must not have side-effects like a panic.
pub enum ScalarPadding<T: Scalar> {
    /// Tells `ggmath` that vector padding has to be initialized with the specified value.
    ///
    /// This adds an extra copy instruction to vector initialization.
    ///
    /// If you can verify that any memory is a valid padding value,
    /// then its better to use `ScalarPadding::Uninit` which saves the copy instruction.
    Init(T),

    /// Tells `ggmath` that any memory is a valid padding value.
    ///
    /// This saves a copy instruction to vector initialization because it doesn't have to initialize the padding.
    ///
    /// Only use this if ANY memory as a `T` will not break operator functions.
    Uninit,
}
