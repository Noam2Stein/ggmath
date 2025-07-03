//! Trait for types that can be put inside math-types like ```Vector``` and ```Matrix```.
//! For example: [```f32```], [```u8```] and [```bool```] are scalars.

use super::*;

mod positive_dir;
pub use positive_dir::*;

/// Trait for types that can be put inside math-types like ```Vector``` and ```Matrix```.
/// For example: [```f32```], [```u8```] and [```bool```] are scalars.
///
/// - References are NOT scalars.
///
/// ### Implementing this trait
///
/// To implement this trait for a type that wraps another ```Scalar``` type
/// (for example ```struct Meters(f32);```),
/// use the inexistant wrapper system.
///
/// To implement this trait for a unique type use this example:
///
/// ```
/// struct u256(u128, u128);
///
/// scalar_inner_vectors!(u256(32)); // 32 - size in bytes
///
/// impl Scalar for u256 {}
/// ```
pub trait Scalar: Construct {
    type Vec2Alignment: Construct;
    type Vec3Alignment: Construct;
    type Vec4Alignment: Construct;
}
