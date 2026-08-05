//! A fast linear algebra library for games and graphics.
//!
//! - Vectors: [`Vec2<T>`], [`Vec3<T>`], [`Vec4<T>`]
//! - Square Matrices: [`Mat2<T>`], [`Mat3<T>`], [`Mat4<T>`]
//! - Quaternions: [`Quat<T>`]
//! - Affine Transforms: [`Affine2<T>`], [`Affine3<T>`]
//! - Projective Transforms: [`Proj2<T>`], [`Proj3<T>`]
//! - Masks: [`Mask2<T>`], [`Mask3<T>`], [`Mask4<T>`]
//!
//! SIMD variants:
//!
//! - Vectors: [`Vec2A<T>`], [`Vec3A<T>`], [`Vec4A<T>`]
//! - Square Matrices: [`Mat2A<T>`], [`Mat3A<T>`], [`Mat4A<T>`]
//! - Quaternions: [`QuatA<T>`]
//! - Affine Transforms: [`Affine2A<T>`], [`Affine3A<T>`]
//! - Projective Transforms: [`Proj2A<T>`], [`Proj3A<T>`]
//! - Masks: [`Mask2A<T>`], [`Mask3A<T>`], [`Mask4A<T>`]
//!
//! Underlying generic types:
//!
//! - [`Vector<N, T, A>`]
//! - [`Matrix<N, T, A>`]
//! - [`Quaternion<T, A>`]
//! - [`Affine<N, T, A>`]
//! - [`Projective<N, T, A>`]
//! - [`Mask<N, T, A>`]
//!
//! # SIMD
//!
//! SIMD variants use specialization to have appropriate alignment and to use
//! explicit SIMD in function implementations.
//!
//! SIMD results in faster computations, but can actually hurt performance if
//! the bottleneck is memory bandwidth rather than computation throughput. For
//! maximum performance, there are both SIMD, non-SIMD and SoA types
//! ([see below](#soa)).
//!
//! | Type              | [`Vec3<f32>`] | [`Vec3A<f32>`] | [`Mat3<f32>`] | [`Mat3A<f32>`] |
//! | ----------------- | ------------- | -------------- | ------------- | -------------- |
//! | Size (bytes)      | 12            | 16             | 36            | 48             |
//! | Alignment (bytes) | 4             | 16             | 4             | 16             |
//! | Padding (bytes)   | 0             | 4              | 0             | 12             |
//!
//! | Type              | [`Vec4<f32>`] | [`Vec4A<f32>`] | [`Mat4<f32>`] | [`Mat4A<f32>`] |
//! | ----------------- | ------------- | -------------- | ------------- | -------------- |
//! | Size (bytes)      | 16            | 16             | 64            | 64             |
//! | Alignment (bytes) | 4             | 16             | 4             | 16             |
//! | Padding (bytes)   | 0             | 0              | 0             | 0              |
//!
//! > This table is true only for target architectures that have SIMD and are
//! > supported. Types incompatible with SIMD use fallback implementations.
//! > Currently support is limited to [`f32`] types on x86 and aarch64.
//!
//! # Generics
//!
//! The underlying types are generic over:
//!
//! - `T`: The element type
//! - `N`: The dimension
//! - `A`: The alignment mode (SIMD or non-SIMD)
//!
//! The traits [`PrimitiveFloat`], [`PrimitiveInteger`], [`PrimitiveSigned`] and
//! [`PrimitiveUnsigned`] give generic contexts access to most primitive
//! functionality. These traits do not expose functions directly, they only
//! enable functionality for vectors, matrices, etc. For complete primitive
//! generics, add the [`num-primitive`] crate as an optional dependency.
//!
//! # Affine and Projective Transforms
//!
//! Unlike many graphics math libraries, [`ggmath`] does not use [`Mat4`] to
//! represent every kind of 3D transformation, nor [`Mat3`] for every kind of 2D
//! transformation. Instead, there are three kinds of transform types, so that
//! common transformations can use more efficient representations:
//!
//! - Matrix types represent linear transformations. They can represent scale,
//!   rotation and shear, but not translation. Use these when translation is not
//!   needed.
//!
//! - Affine transforms are represented by a matrix and a translation vector.
//!   They can represent any linear transformation, plus translation. Use these
//!   for the transform of objects and cameras.
//!
//! - Projective transforms are represented by homogeneous matrices (e.g.,
//!   [`Proj3`] represented by [`Mat4`], and [`Proj2`] represented by [`Mat3`]).
//!   They can represent any affine transformation, plus perspective projection.
//!   Use these when projection is needed.
//!
//! You should generally pick the smallest type that satisfies your
//! requirements. Linear transforms (matrices) are more efficient than affine
//! transforms, which are more efficient than projective transforms. See
//! [benchmark results].
//!
//! | Type              | [`Mat2<f32>`] | [`Affine2<f32>`] | [`Proj2<f32>`] | [`Mat2A<f32>`] | [`Affine2A<f32>`] | [`Proj2A<f32>`] |
//! | ----------------- | ------------- | ---------------- | -------------- | -------------- | ----------------- | --------------- |
//! | Size (bytes)      | 16            | 24               | 36             | 16             | 32                | 48              |
//! | Alignment (bytes) | 4             | 4                | 4              | 16             | 16                | 16              |
//!
//! | Type              | [`Mat3<f32>`] | [`Affine3<f32>`] | [`Proj3<f32>`] | [`Mat3A<f32>`] | [`Affine3A<f32>`] | [`Proj3A<f32>`] |
//! | ----------------- | ------------- | ---------------- | -------------- | -------------- | ----------------- | --------------- |
//! | Size (bytes)      | 36            | 48               | 64             | 48             | 64                | 64              |
//! | Alignment (bytes) | 4             | 4                | 4              | 16             | 16                | 16              |
//!
//! > This table is true only for target architectures that have SIMD and are
//! > supported.
//!
//! # Masks
//!
//! Masks are boolean vectors optimized for specific vector types. For example,
//! [`Mask3A<f32>`] performs better than [`Vec3A<bool>`] for operations
//! involving [`Vec3A<f32>`].
//!
//! # SoA
//!
//! SoA, or Structure of Arrays, refers to math types where each element `T`
//! contains multiple values. For example, [`Vec3<f32x4>`] represents four 3D
//! vectors, stored in memory as:
//!
//! `x1, x2, x3, x4, y1, y2, y3, y4, z1, z2, z3, z4`
//!
//! SoA is faster than standard SIMD. For example, computing the dot product for
//! [`Vec3<f32>`] is quite slow because SIMD is not built for horizontal
//! operations, while for [`Vec3<f32x4>`] it is much faster because each element
//! is a SIMD register and there are no horizontal operations.
//!
//! However, SoA requires that algorithms are designed to process multiple
//! values at the same time, which can be quite challenging. Because of this, it
//! is best to only use SoA for performance-critical algorithms.
//!
//! SoA is supported through an optional dependency for the [`wide`] crate.
//! Almost all functionality that exists for standard types also exists for SoA
//! types.
//!
//! > [The `docs.rs` page] currently doesn't show [`wide`] support. See
//! > [this issue](https://github.com/Noam2Stein/ggmath/issues/45).
//!
//! # Fixed-point numbers
//!
//! Currently, there is only basic support for fixed-point numbers, through the
//! [`fixed`] feature flag which implements [`Scalar`] for [`fixed`] types. See
//! [this issue](https://github.com/Noam2Stein/ggmath/issues/46) for better
//! fixed-point number support.
//!
//! # Linear algebra conventions
//!
//! [`ggmath`] is coordinate-system agnostic, and should work for both
//! right-handed and left-handed coordinate systems.
//!
//! [`ggmath`] uses left-multiplication, meaning to transform a vector by a
//! matrix (or any other transformation) you write `vector * matrix` and not
//! `matrix * vector`. This means matrices are stored in row-major order.
//!
//! # Why another math crate?
//!
//! [`ggmath`] exists because existing similar libraries are missing certain
//! features:
//!
//! - SIMD alignment (e.g., `Vec3` is `__m128`, important for performance)
//! - Generics (over primitives or arbitrary types, avoids macros)
//! - SoA (niche, but important for game engines)
//! - Fixed-point numbers (niche too, but important for game engines that aim to
//!   be flexible)
//!
//! Existing similar libraries:
//!
//! - [`glam`]: Supports SIMD alignment, but does not use generics, and as a
//!   result SoA and fixed-point numbers are out of scope.
//!
//! - [`ultraviolet`]: Supports SoA, but does not support SIMD alignment because
//!   its types are simple scalar structs. Does not use generics, and as a
//!   result fixed-point numbers are probably out of scope.
//!
//! - [`cgmath`]: Supports generics (could also support SoA and fixed-point
//!   numbers) but does not support SIMD alignment, because its types are simple
//!   scalar structs.
//!
//! - [`nalgebra`]: Less graphics oriented and thus has a larger, more
//!   complicated API more suitable for general linear algebra.
//!
//! [`ggmath`] has a design where types are generic over `N` and `T`, but also
//! whether SIMD alignment is enabled or disabled, enabling it to support both
//! SIMD alignment and generics. Changing existing libraries to use this design
//! would be out of scope.
//!
//! # Usage
//!
//! Rust must be updated to version `1.95.0` or later.
//!
//! Add this to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! ggmath = "0.17.1"
//! ```
//!
//! For [`no_std`] support, enable the [`libm`] feature:
//!
//! ```toml
//! [dependencies]
//! ggmath = { version = "0.17.1", features = ["libm"] }
//! ```
//!
//! # Feature flags
//!
//! - [`bytemuck`]: Implements [`bytemuck`] traits for [`ggmath`] types.
//!
//! - [`fixed`]: Implements [`Scalar`] for fixed-point numbers.
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
//! - [`wide`]: Implements functionality for SoA types.
//!
//! [`ggmath`]: crate
//! [`num-primitive`]: https://crates.io/crates/num-primitive
//!
//! [benchmark results]: https://github.com/Noam2Stein/ggmath/blob/main/BENCH_RESULTS.md
//!
//! [`wide`]: https://crates.io/crates/wide
//! [The `docs.rs` page]: https://docs.rs/ggmath
//!
//! [`fixed`]: https://crates.io/crates/fixed
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
    constants::{NegOne, One, Zero},
    euler_rot::EulerRot,
    float_ext::FloatExt,
    length::{Length, SupportedLength},
    mask::{Mask, Mask2, Mask2A, Mask3, Mask3A, Mask4, Mask4A},
    matrix::{Mat2, Mat2A, Mat3, Mat3A, Mat4, Mat4A, Matrix},
    primitive_traits::{PrimitiveFloat, PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned},
    projective::{Proj2, Proj2A, Proj3, Proj3A, Projective},
    quaternion::{Quat, QuatA, Quaternion},
    scalar::{CustomScalar, Scalar},
    vector::{Vec2, Vec2A, Vec3, Vec3A, Vec4, Vec4A, Vector},
};

mod affine;
mod alignment;
mod backend;
mod constants;
mod euler_rot;
mod float_ext;
mod length;
mod mask;
mod matrix;
mod primitive_traits;
mod projective;
mod quaternion;
mod scalar;
mod third_party;
mod utils;
mod vector;

#[cfg(test)]
mod test_utils;
