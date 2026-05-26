# `ggmath`

A linear algebra library for games and graphics with generic SIMD types.

- Vectors: [`Vec2<T>`], [`Vec3<T>`], [`Vec4<T>`].
- Square Matrices: [`Mat2<T>`], [`Mat3<T>`], [`Mat4<T>`].
- Quaternions: [`Quat<T>`].
- Affine Transforms: [`Affine2<T>`], [`Affine3<T>`].
- Masks: [`Mask2<T>`], [`Mask3<T>`], [`Mask4<T>`].

SIMD variants:

- Vectors: [`Vec2A<T>`], [`Vec3A<T>`], [`Vec4A<T>`].
- Square Matrices: [`Mat2A<T>`], [`Mat3A<T>`], [`Mat4A<T>`].
- Quaternions: [`QuatA<T>`].
- Affine Transforms: [`Affine2A<T>`], [`Affine3A<T>`].
- Masks: [`Mask2A<T>`], [`Mask3A<T>`], [`Mask4A<T>`].

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
performance, there are both SIMD and non-SIMD types.

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
> Currently support is limited to [`f32`] types on x86.

## Generics

Because types are generic over `T`, they support non-primitive scalar types.
Integration with [`fixed`] enables support for fixed-point numbers, and
integration with [`wide`] enables support for SoA.

When Rust's type system is powerful enough, integration with [`num-primitive`]
will enable writing math code that is generic over primitive types, for example
functions generic over `T: PrimitiveFloat` will have access to float-vector
functionality.

Types relative to each other (e.g., `Vec2<T>`, `Vec3<T>`, `Vec4<T>` and
SIMD variants) are not distinct types, instead they are all type aliases to
these const-generic structs:

- `Vector<N, T, A>`.
- `Matrix<N, T, A>`.
- `Quaternion<T, A>`.
- `Affine<N, T, A>`.
- `Mask<N, T, A>`.

Where:

- `N` is the length (2, 3, or 4).
- `T` is the scalar type.
- `A` is either `Aligned` or `Unaligned`.

Const generics eliminate the need for macros, making it easier to implement
functionality for all lengths (and both alignments). For example, instead of
defining seperate `Ray2` and `Ray3` types, it is possible to define a single
`Ray<N, T, A>` type then define type aliases for it.

## Math conventions

`ggmath` is coordinate-system agnostic, and should work for both right-handed
and left-handed coordinate systems.

`ggmath` uses left-multiplication, meaning to transform a vector by a matrix (or
quaternion) you write `vector * matrix` and not `matrix * vector`. This means
matrices are stored in row-major order.

Angles are in radians, but can be converted to and from degrees using
standard-library functions.

## Development status

Feature List:

- [x] Vectors
- [x] Square Matrices
- [x] Quaternions
- [x] Affine Transforms
- [x] Masks
- [x] Sufficient Float-Vector functionality
- [x] Sufficient Int-Vector functionality
- [x] Sufficient Matrix functionality
- [x] Sufficient Quaternion functionality
- [x] Sufficient Affine functionality

Crate Support:

- [x] [`bytemuck`](https://crates.io/crates/bytemuck)
- [ ] [`fixed`](https://crates.io/crates/fixed) (partially done)
- [x] [`libm`](https://crates.io/crates/libm)
- [x] [`mint`](https://crates.io/crates/mint)
- [x] [`serde`](https://crates.io/crates/serde)
- [ ] [`wide`](https://crates.io/crates/wide) (partially done)
- [x] [`rand`](https://crates.io/crates/rand)

Performance:

- [x] `f32` vector SSE2 optimizations
- [ ] `f32` matrix SSE2 optimizations
- [ ] `f32` quaternion SSE2 optimizations
- [ ] `f32` affine SSE2 optimizations
- [ ] `f32` mask SSE2 optimizations
- [ ] `i32` `u32` SSE2 optimizations
- [ ] `f32` NEON optimizations
- [ ] `i32` `u32` NEON optimizations
- [ ] `f32` WASM optimizations
- [ ] `i32` `u32` WASM optimizations
- [ ] Niche `f32` SSE4.2+ optimizations
- [ ] Niche `i32` `u32` SSE4.2+ optimizations
- [ ] Niche `f64` AVX+ optimizations
- [ ] Niche `i8` `u8` `bool` SSE2+ optimizations
- [ ] Niche `i16` `u16` SSE2+ optimizations
- [ ] Niche `i8` `u8` `bool` NEON optimizations
- [ ] Niche `i16` `u16` NEON optimizations
- [ ] Niche `i8` `u8` `bool` WASM optimizations
- [ ] Niche `i16` `u16` WASM optimizations

## Usage

Rust must be updated to version `1.95.0` or later.

Add this to your Cargo.toml:

```toml
[dependencies]
ggmath = "0.16.7"
```

For `no_std` support, enable the `libm` feature:

```toml
[dependencies]
ggmath = { version = "0.16.7", features = ["libm"] }
```

## Optional features

- `bytemuck`: Implements `bytemuck` traits for `ggmath` types.

- `fixed`: Implements `Scalar` for fixed-point numbers.

- `libm`: Uses `libm` as the backend for float functionality instead of `std`.
  This makes the crate `no_std`.

- `mint`: Implements conversions between `ggmath` and `mint` types.

- `rand`: Implements `rand` traits for `ggmath` types.

- `serde`: Implements `Serialize` and `Deserialize` for `ggmath` types.

- `wide`: Implements `Scalar` for SIMD types.

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

`ggmath` is heavily inspired by [`glam`] and ports most of its code from it, as
it serves the same purpose as `glam` but with generics.

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
[`f32`]: https://doc.rust-lang.org/std/primitive.f32.html

[`glam`]: https://crates.io/crates/glam
[`wide`]: https://crates.io/crates/wide
[`fixed`]: https://crates.io/crates/fixed
[`num-primitive`]: https://crates.io/crates/num-primitive
