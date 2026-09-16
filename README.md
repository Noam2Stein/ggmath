# `ggmath`

A fast linear algebra library for games and graphics.

- Vectors: [`Vec2<T>`], [`Vec3<T>`], [`Vec4<T>`]
- Square Matrices: [`Mat2<T>`], [`Mat3<T>`], [`Mat4<T>`]
- Affine Transforms: [`Affine2<T>`], [`Affine3<T>`]
- Projective Transforms: [`Proj2<T>`], [`Proj3<T>`]
- Rotations: [`Rot2<T>`], [`Rotor3<T>`]
- Vector Masks: [`Mask2<T>`], [`Mask3<T>`], [`Mask4<T>`]

SIMD-aligned types:

- Vectors: [`Vec2A<T>`], [`Vec3A<T>`], [`Vec4A<T>`]
- Square Matrices: [`Mat2A<T>`], [`Mat3A<T>`], [`Mat4A<T>`]
- Affine Transforms: [`Affine2A<T>`], [`Affine3A<T>`]
- Projective Transforms: [`Proj2A<T>`], [`Proj3A<T>`]
- Rotations: [`Rot2A<T>`], [`Rotor3A<T>`]
- Vector Masks: [`Mask2A<T>`], [`Mask3A<T>`], [`Mask4A<T>`]

Underlying generic types:

- [`Vector<N, T, A>`]
- [`Matrix<N, T, A>`]
- [`Affine<N, T, A>`]
- [`Projective<N, T, A>`]
- [`Rotation2<T, A>`]
- [`Rotor<N, T, A>`]
- [`Mask<N, T, A>`]

## SIMD-aligned types

Math types come in two variants: scalar types and SIMD-aligned types. Scalar
types are named without a suffix, and SIMD-aligned types are named with an `A`
suffix.

Scalar types have the alignment of their `T` type, and are considered the
default math types.

For supported `T` types and target configurations, SIMD-aligned types have
additional alignment allowing them to be efficiently loaded into SIMD registers.
Operations on these types use specialized SIMD implementations. For unsupported
`T` types, SIMD-aligned types are identical to their scalar counterparts.

On target configurations `x86`/`x86_64` and `aarch64` with `neon`, appropriate
[`f32`] types have 16-byte alignment and use optimized SIMD implementations.
Exact representations are documented on each type.

Generally, SIMD-aligned types result in faster computations, but consume more
memory due to alignment and/or padding. SIMD-aligned types tend to improve
performance for compute-bound algorithms, and tend to hurt performance for
memory-bound algorithms.

## Generics

Generics are used to simplify the API and avoid "type explosion". While there
are multiple marker traits due to type system limitations, all actual
functionality is provided through inherent implementations, not traits.

Underlying types are useful when defining composite math types or extending
existing types with more functionality. They are generic over:

- `N`: The dimension
- `T`: The element type
- `A`: The SIMD-alignment mode (aligned or unaligned)

The marker traits [`PrimitiveFloat`], [`PrimitiveInteger`], [`PrimitiveSigned`]
and [`PrimitiveUnsigned`] give generic contexts access to most functionality
specific to primitive types. These traits do not expose any functions directly,
they only enable functionality for math types.

## SoA

SoA, short for Structure of Arrays, refers to math types where each element is a
SIMD vector representing multiple values. Whereas SIMD-aligned types make an
entire [`Vector`] a SIMD vector, as in [`Vec3A<f32>`], SoA makes each element a
SIMD vector, as in [`Vec3<f32x4>`], which represents four 3D vectors.

SoA is supported through an optional dependency for the [`wide`] crate. Almost
all functionality that exists for standard types also exists for SoA types.

SoA generally results in way faster computations than SIMD-aligned types.
Mathematical operations on SIMD-aligned types frequently have to use shuffle and
extract instructions, and cannot use SIMD for single-element operations. The
same operations on SoA types never use shuffles nor extracts, and do use SIMD
for single-element operations, since each element is a SIMD vector.

SoA also has downsides:

- SoA values often need to be converted from and into AoS form, which depending
  on the algorithm, can have overhead higher than the speedup SoA provides.

- SoA values take a lot of register space. A single [`Mat4<f32x4>`] takes 16
  SIMD registers, which is all that is available on some processors. If too many
  values are used at the same time, data will move to the stack, heavily hurting
  performance.

## Linear algebra conventions

[`ggmath`] is coordinate-system agnostic, and should work for both left-handed
and right-handed coordinate systems.

[`ggmath`] uses left-multiplication and provides row-major matrices. To
transform a vector by a matrix (or any other transformation) you write
`vector * matrix`, not `matrix * vector`. To chain transformations you write
`first * second`, not `second * first`.

## Why another math crate?

The reason for creating a new math crate, instead of updating an existing one, is
to add certain features that cannot be added to those libraries, due to
incompatibilities that would be too big of a change to fix. These features are:

- SIMD-aligned types
- Generics
- SoA
- Fixed-point numbers

While these features are not needed for every project, in a general-purpose game
engine it is important to design the math module with support for these features
in mind, since some developers will need them eventually. If you do not design
for generics and SIMD alignment from the start, they are hard to add properly
afterwards. Without generics, supporting multiple `T` types quickly leads to
macro hell and type explosion.

Existing libraries:

- [`glam`]: Supports SIMD-aligned types, but not generics.

- [`ultraviolet`]: Supports SoA, but not generics.

- [`cgmath`]: Supports generics, but not SIMD-aligned types, since its generic
  types are simple scalar structs.

- [`nalgebra`]: Less graphics oriented and has a larger, more complicated API
  more suitable for general linear algebra. Supports generics and SoA, but not
  SIMD-aligned types.

## Usage

Rust must be updated to version `1.95.0` or later.

Add this to your Cargo.toml:

```toml
[dependencies]
ggmath = "0.18.0"
```

For [`no_std`] support, enable the [`libm`] feature:

```toml
[dependencies]
ggmath = { version = "0.18.0", features = ["libm"] }
```

## Feature flags

- [`bytemuck`]: Implements [`bytemuck`] traits for [`ggmath`] types.

- [`fixed`]: Implements element traits for fixed-point numbers.

- [`half`]: Implements element traits for [`f16`] and [`bf16`].

- [`libm`]: Uses [`libm`] instead of [`std`] as the backend for
  floating-point functions. This makes the crate [`no_std`].

- [`mint`]: Implements conversions between [`ggmath`] and [`mint`] types.

- [`rand`]: Implements [`rand`] traits for [`ggmath`] types.

- [`serde`]: Implements [`Serialize`] and [`Deserialize`] for [`ggmath`] types.

- [`wide`]: Enables SoA functionality.

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

[`ggmath`] is heavily inspired by [`glam`] and ports a ton of code from it.

[`Vec2<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec2.html
[`Vec3<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec3.html
[`Vec4<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec4.html
[`Mat2<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat2.html
[`Mat3<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat3.html
[`Mat4<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat4.html
[`Affine2<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine2.html
[`Affine3<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine3.html
[`Proj2<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Proj2.html
[`Proj3<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Proj3.html
[`Rot2<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Rot2.html
[`Rotor3<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Rotor3.html
[`Mask2<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask2.html
[`Mask3<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask3.html
[`Mask4<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask4.html

[`Vec2A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec2A.html
[`Vec3A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec3A.html
[`Vec4A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec4A.html
[`Mat2A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat2A.html
[`Mat3A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat3A.html
[`Mat4A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat4A.html
[`Affine2A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine2A.html
[`Affine3A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Affine3A.html
[`Proj2A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Proj2A.html
[`Proj3A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Proj3A.html
[`Rot2A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Rot2A.html
[`Rotor3A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Rotor3A.html
[`Mask2A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask2A.html
[`Mask3A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask3A.html
[`Mask4A<T>`]: https://docs.rs/ggmath/latest/ggmath/type.Mask4A.html

[`Vector<N, T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Vector.html
[`Matrix<N, T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Matrix.html
[`Affine<N, T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Affine.html
[`Projective<N, T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Projective.html
[`Rotation2<T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Rotation2.html
[`Rotor<N, T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Rotor.html
[`Mask<N, T, A>`]: https://docs.rs/ggmath/latest/ggmath/struct.Mask.html

[`f32`]: https://doc.rust-lang.org/std/primitive.f32.html

[`PrimitiveFloat`]: https://docs.rs/ggmath/latest/ggmath/trait.PrimitiveFloat.html
[`PrimitiveInteger`]: https://docs.rs/ggmath/latest/ggmath/trait.PrimitiveInteger.html
[`PrimitiveSigned`]: https://docs.rs/ggmath/latest/ggmath/trait.PrimitiveSigned.html
[`PrimitiveUnsigned`]: https://docs.rs/ggmath/latest/ggmath/trait.PrimitiveUnsigned.html

[`Vector`]: https://docs.rs/ggmath/latest/ggmath/struct.Vector.html
[`Vec3A<f32>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec3A.html
[`Vec3<f32x4>`]: https://docs.rs/ggmath/latest/ggmath/type.Vec3.html
[`Mat4<f32x4>`]: https://docs.rs/ggmath/latest/ggmath/type.Mat4.html
[`wide`]: https://crates.io/crates/wide

[`ggmath`]: https://crates.io/crates/ggmath

[`glam`]: https://crates.io/crates/glam
[`ultraviolet`]: https://crates.io/crates/ultraviolet
[`cgmath`]: https://crates.io/crates/cgmath
[`nalgebra`]: https://crates.io/crates/nalgebra

[`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
[`libm`]: https://crates.io/crates/libm

[`bytemuck`]: https://crates.io/crates/bytemuck
[`fixed`]: https://crates.io/crates/fixed
[`half`]: https://crates.io/crates/half
[`f16`]: https://docs.rs/half/latest/half/struct.f16.html
[`bf16`]: https://docs.rs/half/latest/half/struct.bf16.html
[`std`]: https://doc.rust-lang.org/std
[`mint`]: https://crates.io/crates/mint
[`rand`]: https://crates.io/crates/rand
[`serde`]: https://crates.io/crates/serde
[`Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
[`Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html
