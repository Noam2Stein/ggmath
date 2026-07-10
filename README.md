# `ggmath`

A fast linear algebra library for games and graphics.

- Vectors: [`Vec2<T>`], [`Vec3<T>`], [`Vec4<T>`]
- Square Matrices: [`Mat2<T>`], [`Mat3<T>`], [`Mat4<T>`]
- Quaternions: [`Quat<T>`]
- Affine Transforms: [`Affine2<T>`], [`Affine3<T>`]
- Masks: [`Mask2<T>`], [`Mask3<T>`], [`Mask4<T>`]

SIMD variants:

- Vectors: [`Vec2A<T>`], [`Vec3A<T>`], [`Vec4A<T>`]
- Square Matrices: [`Mat2A<T>`], [`Mat3A<T>`], [`Mat4A<T>`]
- Quaternions: [`QuatA<T>`]
- Affine Transforms: [`Affine2A<T>`], [`Affine3A<T>`]
- Masks: [`Mask2A<T>`], [`Mask3A<T>`], [`Mask4A<T>`]

Underlying generic types:

- [`Vector<N, T, A>`]
- [`Matrix<N, T, A>`]
- [`Quaternion<T, A>`]
- [`Affine<N, T, A>`]
- [`Mask<N, T, A>`]

## SIMD

SIMD variants use specialization to have appropriate alignment and to use
explicit SIMD in function implementations.

SIMD results in faster computations, but can actually hurt performance if the
bottleneck is memory bandwidth rather than computation throughput. For maximum
performance, there are both SIMD, non-SIMD and SoA types ([see below](#soa)).

| Type              | [`Vec3<f32>`] | [`Vec3A<f32>`] | [`Mat3<f32>`] | [`Mat3A<f32>`] |
| ----------------- | ------------- | -------------- | ------------- | -------------- |
| Size (bytes)      | 12            | 16             | 36            | 48             |
| Alignment (bytes) | 4             | 16             | 4             | 16             |
| Padding (bytes)   | 0             | 4              | 0             | 12             |

| Type              | [`Vec4<f32>`] | [`Vec4A<f32>`] | [`Mat4<f32>`] | [`Mat4A<f32>`] |
| ----------------- | ------------- | -------------- | ------------- | -------------- |
| Size (bytes)      | 16            | 16             | 64            | 64             |
| Alignment (bytes) | 4             | 16             | 4             | 16             |
| Padding (bytes)   | 0             | 0              | 0             | 0              |

> This table is true only for target architectures that have SIMD and are
> supported. Types incompatible with SIMD use fallback implementations.
> Currently support is limited to [`f32`] types on x86 and aarch64.

## Generics

The underlying types are generic over:

- `T`: The element type
- `N`: The dimension
- `A`: The alignment mode (SIMD or non-SIMD)

The traits [`PrimitiveFloat`], [`PrimitiveInteger`], [`PrimitiveSigned`] and
[`PrimitiveUnsigned`] give generic contexts access to most primitive
functionality. These traits do not expose functions directly, they only enable
functionality for vectors, matrices, etc. For complete primitive generics, add
the [`num-primitive`] crate as an optional dependency.

## Affine transforms

An affine transform contains a linear transformation and a translation vector.
It can represent scale, rotation, shear and translation, but cannot represent
projections. [`Affine2<T>`] is equivalent to [`Mat3<T>`], and [`Affine3<T>`] is
equivalent to [`Mat4<T>`].

Affine transforms take less memory than matrices and perform better for select
operations (see [benchmark results]).

| Type              | [`Affine2<f32>`] | [`Mat3<f32>`] | [`Affine2A<f32>`] | [`Mat3A<f32>`] |
| ----------------- | ---------------- | ------------- | ----------------- | -------------- |
| Size (bytes)      | 24               | 36            | 32                | 48             |
| Alignment (bytes) | 4                | 4             | 16                | 16             |

| Type              | [`Affine3<f32>`] | [`Mat4<f32>`] | [`Affine3A<f32>`] | [`Mat4A<f32>`] |
| ----------------- | ---------------- | ------------- | ----------------- | -------------- |
| Size (bytes)      | 48               | 64            | 64                | 64             |
| Alignment (bytes) | 4                | 4             | 16                | 16             |

> This table is true only for target architectures that have SIMD and are
> supported.

## Masks

Masks are boolean vectors optimized for specific vector types. For example,
[`Mask3A<f32>`] performs better than [`Vec3A<bool>`] for operations involving
[`Vec3A<f32>`].

## SoA

SoA, or Structure of Arrays, refers to math types where each element `T`
contains multiple values. For example, [`Vec3<f32x4>`] represents four 3D
vectors, stored in memory as:

`x1, x2, x3, x4, y1, y2, y3, y4, z1, z2, z3, z4`

SoA is faster than standard SIMD. For example, computing the dot product for
[`Vec3<f32>`] is quite slow because SIMD is not built for horizontal operations,
while for [`Vec3<f32x4>`] it is much faster because each element is a SIMD
register and there are no horizontal operations.

However, SoA requires that algorithms are designed to process multiple values at
the same time, which can be quite challenging. Because of this, it is best to
only use SoA for performance-critical algorithms.

SoA is supported through an optional dependency for the [`wide`] crate. Almost
all functionality that exists for standard types also exists for SoA types.

> [The `docs.rs` page] currently doesn't show [`wide`] support. See
> [this issue](https://github.com/Noam2Stein/ggmath/issues/45).

## Fixed-point numbers

Currently, there is only basic support for fixed-point numbers, through the
[`fixed`] feature flag which implements [`Scalar`] for [`fixed`] types. See
[this issue](https://github.com/Noam2Stein/ggmath/issues/46) for better
fixed-point number support.

## Linear algebra conventions

[`ggmath`] is coordinate-system agnostic, and should work for both right-handed
and left-handed coordinate systems.

[`ggmath`] uses left-multiplication, meaning to transform a vector by a matrix
(or quaternion) you write `vector * matrix` and not `matrix * vector`. This
means matrices are stored in row-major order.

## Why another math crate?

[`ggmath`] exists because existing similar libraries are missing certain
features:

- SIMD alignment (e.g., `Vec3` is `__m128`, important for performance)
- Generics (over primitives or arbitrary types, avoids macros)
- SoA (niche, but important for game engines)
- Fixed-point numbers (niche too, but important for game engines that aim to be
  flexible)

Existing similar libraries:

- [`glam`]: Supports SIMD alignment, but does not use generics, and as a result
  SoA and fixed-point numbers are out of scope.

- [`ultraviolet`]: Supports SoA, but does not support SIMD alignment because its
  types are simple scalar structs. Does not use generics, and as a result
  fixed-point numbers are probably out of scope.

- [`cgmath`]: Supports generics (could also support SoA and fixed-point numbers)
  but does not support SIMD alignment, because its types are simple scalar
  structs.

- [`nalgebra`]: Less graphics oriented and thus has a larger, more complicated
  API more suitable for general linear algebra.

[`ggmath`] has a design where types are generic over `N` and `T`, but also
whether SIMD alignment is enabled or disabled, enabling it to support both SIMD
alignment and generics. Changing existing libraries to use this design would be
out of scope.

## Usage

Rust must be updated to version `1.95.0` or later.

Add this to your Cargo.toml:

```toml
[dependencies]
ggmath = "0.17.0"
```

For [`no_std`] support, enable the [`libm`] feature:

```toml
[dependencies]
ggmath = { version = "0.17.0", features = ["libm"] }
```

## Feature flags

- [`bytemuck`]: Implements [`bytemuck`] traits for [`ggmath`] types.

- [`fixed`]: Implements [`Scalar`] for fixed-point numbers.

- [`libm`]: Uses [`libm`] instead of [`std`] as the backend for
  floating-point functions. This makes the crate [`no_std`].

- [`mint`]: Implements conversions between [`ggmath`] and [`mint`] types.

- [`rand`]: Implements [`rand`] traits for [`ggmath`] types.

- [`serde`]: Implements [`Serialize`] and [`Deserialize`] for [`ggmath`] types.

- [`wide`]: Implements functionality for SoA types.

## License

Licensed under either Apache License Version 2.0 or MIT license at your option.

## Contribution

Contributions in any form (issues, pull requests, etc.) to this project must
adhere to Rust's
[Code of Conduct](https://rust-lang.org/policies/code-of-conduct/).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Attribution

[`ggmath`] is heavily inspired by [`glam`] and ports a ton of code from it, as
it serves the same purpose as [`glam`] but with generics.

[`Vec2<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec2.html
[`Vec3<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec3.html
[`Vec4<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec4.html
[`Mat2<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat2.html
[`Mat3<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat3.html
[`Mat4<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat4.html
[`Quat<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Quat.html
[`Affine2<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine2.html
[`Affine3<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine3.html
[`Mask2<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask2.html
[`Mask3<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask3.html
[`Mask4<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask4.html

[`Vec2A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec2A.html
[`Vec3A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec3A.html
[`Vec4A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec4A.html
[`Mat2A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat2A.html
[`Mat3A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat3A.html
[`Mat4A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat4A.html
[`QuatA<T>`]: https://docs.rs/ggmath/latest/ggmath/type.QuatA.html
[`Affine2A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine2A.html
[`Affine3A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine3A.html
[`Mask2A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask2A.html
[`Mask3A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask3A.html
[`Mask4A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask4A.html

[`Vector<N, T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Vector.html
[`Matrix<N, T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Matrix.html
[`Quaternion<T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Quaternion.html
[`Affine<N, T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Affine.html
[`Mask<N, T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Mask.html

[`Vec3<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec3.html
[`Vec3A<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec3A.html
[`Mat3<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat3.html
[`Mat3A<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat3A.html
[`Vec4<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec4.html
[`Vec4A<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec4A.html
[`Mat4<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat4.html
[`Mat4A<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat4A.html

[`PrimitiveFloat`]: https://docs.rs/ggmath/latest/ggmath/trait.PrimitiveFloat.html
[`PrimitiveInteger`]: https://docs.rs/ggmath/latest/ggmath/trait.PrimitiveInteger.html
[`PrimitiveSigned`]: https://docs.rs/ggmath/latest/ggmath/trait.PrimitiveSigned.html
[`PrimitiveUnsigned`]: https://docs.rs/ggmath/latest/ggmath/trait.PrimitiveUnsigned.html
[`num-primitive`]: https://crates.io/crates/num-primitive

[benchmark results]: https://github.com/Noam2Stein/ggmath/blob/main/BENCH_RESULTS.md
[`Affine2<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine2.html
[`Affine3<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine3.html
[`Affine2A<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine2A.html
[`Affine3A<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine3A.html

[`Mask3A<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask3A.html
[`Vec3A<bool>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec3A.html

[`Vec3<f32x4>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec3.html
[`wide`]: https://crates.io/crates/wide
[The `docs.rs` page]: https://docs.rs/ggmath

[`fixed`]: https://crates.io/crates/fixed
[`Scalar`]: https://docs.rs/ggmath/latest/ggmath/trait.Scalar.html

[`ggmath`]: https://crates.io/crates/ggmath

[`glam`]: https://crates.io/crates/glam
[`ultraviolet`]: https://crates.io/crates/ultraviolet
[`cgmath`]: https://crates.io/crates/cgmath
[`nalgebra`]: https://crates.io/crates/nalgebra

[`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
[`libm`]: https://crates.io/crates/libm

[`bytemuck`]: https://crates.io/crates/bytemuck
[`std`]: https://doc.rust-lang.org/std
[`mint`]: https://crates.io/crates/mint
[`rand`]: https://crates.io/crates/rand
[`serde`]: https://serde.rs
[`Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
[`Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html
