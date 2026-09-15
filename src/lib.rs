//! A fast linear algebra library for games and graphics.
//!
//! - Vectors: [`Vec2<T>`], [`Vec3<T>`], [`Vec4<T>`]
//! - Square Matrices: [`Mat2<T>`], [`Mat3<T>`], [`Mat4<T>`]
//! - Affine Transforms: [`Affine2<T>`], [`Affine3<T>`]
//! - Projective Transforms: [`Proj2<T>`], [`Proj3<T>`]
//! - Rotations: [`Rot2<T>`], [`Rotor3<T>`]
//! - Vector Masks: [`Mask2<T>`], [`Mask3<T>`], [`Mask4<T>`]
//!
//! SIMD-aligned types:
//!
//! - Vectors: [`Vec2A<T>`], [`Vec3A<T>`], [`Vec4A<T>`]
//! - Square Matrices: [`Mat2A<T>`], [`Mat3A<T>`], [`Mat4A<T>`]
//! - Affine Transforms: [`Affine2A<T>`], [`Affine3A<T>`]
//! - Projective Transforms: [`Proj2A<T>`], [`Proj3A<T>`]
//! - Rotations: [`Rot2A<T>`], [`Rotor3A<T>`]
//! - Vector Masks: [`Mask2A<T>`], [`Mask3A<T>`], [`Mask4A<T>`]
//!
//! Underlying generic types:
//!
//! - [`Vector<N, T, A>`]
//! - [`Matrix<N, T, A>`]
//! - [`Affine<N, T, A>`]
//! - [`Projective<N, T, A>`]
//! - [`Rotation2<T, A>`]
//! - [`Rotor<N, T, A>`]
//! - [`Mask<N, T, A>`]
//!
//! # SIMD-aligned types
//!
//! Math types come in two variants: scalar types and SIMD-aligned types. Scalar
//! types are named without a suffix, and SIMD-aligned types are named with an
//! `A` suffix.
//!
//! Scalar types have the alignment of their `T` type, and are considered the
//! default math types.
//!
//! For supported `T` types and target configurations, SIMD-aligned types have
//! additional alignment allowing them to be efficiently loaded into SIMD
//! registers. Operations on these types use specialized SIMD implementations.
//! For unsupported `T` types, SIMD-aligned types are identical to their scalar
//! counterparts.
//!
//! On target configurations `x86`/`x86_64` and `aarch64` with `neon`,
//! appropriate [`f32`] types have 16-byte alignment and use optimized SIMD
//! implementations. Exact representations are documented on each type.
//!
//! Generally, SIMD-aligned types result in faster computations, but consume
//! more memory due to alignment and/or padding. SIMD-aligned types tend to
//! improve performance for compute-bound algorithms, and tend to hurt
//! performance for memory-bound algorithms.
//!
//! # Generics
//!
//! Generics are used to simplify the API and avoid "type explosion". While
//! there are multiple marker traits due to type system limitations, all actual
//! functionality is provided through inherent implementations, not traits.
//!
//! Underlying types are useful when defining composite math types or extending
//! existing types with more functionality. They are generic over:
//!
//! - `N`: The dimension
//! - `T`: The element type
//! - `A`: The SIMD-alignment mode (aligned or unaligned)
//!
//! The marker traits [`PrimitiveFloat`], [`PrimitiveInteger`],
//! [`PrimitiveSigned`] and [`PrimitiveUnsigned`] give generic contexts access
//! to most functionality specific to primitive types. These traits do not
//! expose any functions directly, they only enable functionality for math
//! types.
//!
//! # SoA
//!
//! SoA, short for Structure of Arrays, refers to math types where each element
//! is a SIMD vector representing multiple values. Whereas SIMD-aligned types
//! make an entire [`Vector`] a SIMD vector, as in [`Vec3A<f32>`], SoA makes
//! each element a SIMD vector, as in [`Vec3<f32x4>`], which represents four 3D
//! vectors.
//!
//! SoA is supported through an optional dependency for the [`wide`] crate.
//! Almost all functionality that exists for standard types also exists for SoA
//! types.
//!
//! SoA generally results in way faster computations than SIMD-aligned types.
//! Mathematical operations on SIMD-aligned types frequently have to use shuffle
//! and extract instructions, and cannot use SIMD for single-element operations.
//! The same operations on SoA types never use shuffles nor extracts, and do use
//! SIMD for single-element operations, since each element is a SIMD vector.
//!
//! SoA also has downsides:
//!
//! - SoA values often need to be converted from and into AoS form, which
//!   depending on the algorithm, can have overhead higher than the speedup SoA
//!   provides.
//!
//! - SoA values take a lot of register space. A single [`Mat4<f32x4>`] takes 16
//!   SIMD registers, which is all that is available on some processors. If too
//!   many values are used at the same time, data will move to the stack,
//!   heavily hurting performance.
//!
//! # Linear algebra conventions
//!
//! [`ggmath`] is coordinate-system agnostic, and should work for both
//! left-handed and right-handed coordinate systems.
//!
//! [`ggmath`] uses left-multiplication and row-major matrices. To transform a
//! vector by a matrix (or any other transformation) you write
//! `vector * matrix`, not `matrix * vector`. To chain transformations you write
//! `first * second`, not `second * first`.
//!
//! # Why another math crate?
//!
//! The reason for creating a new math crate, instead of updating an existing
//! one, is to add certain features that cannot be added to those libraries, due
//! to incompatibilities that would be too big of a change to fix. These
//! features are:
//!
//! - SIMD-aligned types
//! - Generics
//! - SoA
//! - Fixed-point numbers
//!
//! While these features are not needed for every project, in a general-purpose
//! game engine it is important to design the math module with support for these
//! features in mind, since some developers will need them eventually. If you do
//! not design for generics and SIMD alignment from the start, they are hard to
//! add properly afterwards. Without generics, supporting multiple `T` types
//! quickly leads to macro hell and type explosion.
//!
//! Existing libraries:
//!
//! - [`glam`]: Supports SIMD-aligned types, but not generics.
//!
//! - [`ultraviolet`]: Supports SoA, but not generics.
//!
//! - [`cgmath`]: Supports generics, not SIMD-aligned types, since its types are
//!   simple scalar structs.
//!
//! - [`nalgebra`]: Less graphics oriented and has a larger, more complicated
//!   API more suitable for general linear algebra. Supports generics and SoA,
//!   but not SIMD-aligned types.
//!
//! # Usage
//!
//! Rust must be updated to version `1.95.0` or later.
//!
//! Add this to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! ggmath = "0.18.0"
//! ```
//!
//! For [`no_std`] support, enable the [`libm`] feature:
//!
//! ```toml
//! [dependencies]
//! ggmath = { version = "0.18.0", features = ["libm"] }
//! ```
//!
//! # Feature flags
//!
//! - [`bytemuck`]: Implements [`bytemuck`] traits for [`ggmath`] types.
//!
//! - [`fixed`]: Implements [`Element`] for fixed-point numbers.
//!
//! - [`half`]: Implements [`Element`] for [`f16`] and [`bf16`].
//!
//! - [`libm`]: Uses [`libm`] instead of [`std`] as the backend for
//!   floating-point functions. This makes the crate [`no_std`].
//!
//! - [`mint`]: Implements conversions between [`ggmath`] and [`mint`] types.
//!
//! - [`rand`]: Implements [`rand`] traits for [`ggmath`] types.
//!
//! - [`serde`]: Implements [`Serialize`] and [`Deserialize`] for [`ggmath`]
//!   types.
//!
//! - [`wide`]: Enables SoA functionality.
//!
//! [`wide`]: https://crates.io/crates/wide
//!
//! [`ggmath`]: crate
//!
//! [`glam`]: https://crates.io/crates/glam
//! [`ultraviolet`]: https://crates.io/crates/ultraviolet
//! [`cgmath`]: https://crates.io/crates/cgmath
//! [`nalgebra`]: https://crates.io/crates/nalgebra
//!
//! [`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
//! [`libm`]: https://crates.io/crates/libm
//!
//! [`bytemuck`]: https://crates.io/crates/bytemuck
//! [`fixed`]: https://crates.io/crates/fixed
//! [`half`]: https://crates.io/crates/half
//! [`f16`]: https://docs.rs/half/latest/half/struct.f16.html
//! [`bf16`]: https://docs.rs/half/latest/half/struct.bf16.html
//! [`std`]: https://doc.rust-lang.org/std
//! [`mint`]: https://crates.io/crates/mint
//! [`rand`]: https://crates.io/crates/rand
//! [`serde`]: https://crates.io/crates/serde
//! [`Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
//! [`Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html

#![forbid(missing_docs)]
#![cfg_attr(feature = "libm", no_std)]

pub use crate::{
    affine::{Affine, Affine2, Affine2A, Affine3, Affine3A},
    alignment::{Aligned, Alignment, Unaligned},
    dim::{Dim, TwoOrThree, TwoThreeOrFour},
    element::{CustomElement, Element, NegOne, One, Zero},
    eq_test::EqTest,
    euler_rot::EulerRot,
    float_ext::FloatExt,
    mask::{Mask, Mask2, Mask2A, Mask3, Mask3A, Mask4, Mask4A},
    matrix::{Mat2, Mat2A, Mat3, Mat3A, Mat4, Mat4A, Matrix},
    primitive_traits::{PrimitiveFloat, PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned},
    projective::{Proj2, Proj2A, Proj3, Proj3A, Projective},
    rotation2::{Rot2, Rot2A, Rotation2},
    rotor::{Rotor, Rotor3, Rotor3A},
    vector::{Vec2, Vec2A, Vec3, Vec3A, Vec4, Vec4A, Vector},
};

mod affine;
mod alignment;
mod backend;
mod dim;
mod element;
mod eq_test;
mod euler_rot;
mod float_ext;
mod mask;
mod matrix;
mod primitive_traits;
mod projective;
mod rotation2;
mod rotor;
mod third_party;
mod utils;
mod vector;

#[cfg(test)]
mod test_utils;
