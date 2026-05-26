//! A linear algebra library for games and graphics with generic SIMD types.
//!
//! - Vectors: [`Vec2<T>`], [`Vec3<T>`], [`Vec4<T>`].
//! - Square Matrices: [`Mat2<T>`], [`Mat3<T>`], [`Mat4<T>`].
//! - Quaternions: [`Quat<T>`].
//! - Affine Transforms: [`Affine2<T>`], [`Affine3<T>`].
//! - Masks: [`Mask2<T>`], [`Mask3<T>`], [`Mask4<T>`].
//!
//! SIMD variants:
//!
//! - Vectors: [`Vec2A<T>`], [`Vec3A<T>`], [`Vec4A<T>`].
//! - Square Matrices: [`Mat2A<T>`], [`Mat3A<T>`], [`Mat4A<T>`].
//! - Quaternions: [`QuatA<T>`].
//! - Affine Transforms: [`Affine2A<T>`], [`Affine3A<T>`].
//! - Masks: [`Mask2A<T>`], [`Mask3A<T>`], [`Mask4A<T>`].
//!
//! Underlying generic types:
//!
//! - [`Vector<N, T, A>`]
//! - [`Matrix<N, T, A>`]
//! - [`Quaternion<T, A>`]
//! - [`Affine<N, T, A>`]
//! - [`Mask<N, T, A>`]
//!
//! # SIMD
//!
//! SIMD variants use specialization to have appropriate alignment and to use
//! explicit SIMD in function implementations.
//!
//! SIMD results in faster computations, but can actually hurt performance if
//! the bottleneck is memory bandwidth rather than computation throughput. For
//! maximum performance, there are both SIMD and non-SIMD types.
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
//! > Currently support is limited to [`f32`] types on x86.
//!
//! # Generics
//!
//! Because types are generic over `T`, they support non-primitive scalar types
//! (see [`Scalar`]). Integration with [`fixed`] enables support for fixed-point
//! numbers, and integration with [`wide`] enables support for SoA.
//!
//! When Rust's type system is powerful enough, integration with
//! [`num-primitive`] will enable writing math code that is generic over
//! primitive types, for example functions generic over `T: PrimitiveFloat` will
//! have access to float-vector functionality.
//!
//! Types relative to each other (e.g., [`Vec2<T>`], [`Vec3<T>`], [`Vec4<T>`]
//! and SIMD variants) are not distinct types, instead they are all type aliases
//! to these const-generic structs:
//!
//! - [`Vector<N, T, A>`].
//! - [`Matrix<N, T, A>`].
//! - [`Quaternion<T, A>`].
//! - [`Affine<N, T, A>`].
//! - [`Mask<N, T, A>`].
//!
//! Where:
//!
//! - `N` is the length (2, 3, or 4).
//! - `T` is the scalar type (must implement [`Scalar`]).
//! - `A` is either [`Aligned`] or [`Unaligned`].
//!
//! Const generics eliminate the need for macros, making it easier to implement
//! functionality for all lengths (and both alignments). For example, instead of
//! defining seperate `Ray2` and `Ray3` types, it is possible to define a single
//! `Ray<N, T, A>` type then define type aliases for it.
//!
//! # Math conventions
//!
//! The library is coordinate-system agnostic, and should work for both
//! right-handed and left-handed coordinate systems.
//!
//! The library uses left-multiplication, meaning to transform a vector by a
//! matrix (or quaternion) you write `vector * matrix` and not
//! `matrix * vector`. This means matrices are stored in row-major order.
//!
//! Angles are in radians, but can be converted to and from degrees using
//! standard-library functions.
//!
//! # Optional features
//!
//! - `std` (default feature): Uses `std` as the backend for float
//!   functionality.
//!
//! - `bytemuck`: Implements `bytemuck` traits for `ggmath` types.
//!
//! - `fixed`: Implements `Scalar` for fixed-point numbers.
//!
//! - `libm`: Uses `libm` as the backend for float functionality. This makes the
//!   crate `no_std` even if the `std` feature is not disabled. Without `std` or
//!   `libm`, the crate compiles but all float functionality that relies on a
//!   backend is disabled.
//!
//! - `mint`: Implements conversions between `ggmath` and `mint` types.
//!
//! - `rand`: Implements `rand` traits for `ggmath` types.
//!
//! - `serde`: Implements `Serialize` and `Deserialize` for `ggmath` types.
//!
//! - `wide`: Implements `Scalar` for SIMD types.
//!
//! [`wide`]: https://crates.io/crates/wide
//! [`fixed`]: https://crates.io/crates/fixed
//! [`num-primitive`]: https://crates.io/crates/num-primitive
//! [Structure of Arrays]: https://en.wikipedia.org/wiki/AoS_and_SoA

#![forbid(missing_docs)]
#![cfg_attr(feature = "libm", no_std)]

mod affine;
mod backend;
mod euler_rot;
mod float_ext;
mod generics;
mod mask;
mod matrix;
mod quaternion;
mod vector;
pub use affine::*;
pub use backend::*;
pub use euler_rot::*;
pub use float_ext::*;
pub use generics::*;
pub use mask::*;
pub use matrix::*;
pub use quaternion::*;
pub use vector::*;

mod third_party;
mod utils;
