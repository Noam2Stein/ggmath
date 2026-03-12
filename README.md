# `ggmath`

A linear algebra library for games and graphics with generic SIMD types.

The library provides:

- Vectors: `Vec2<T>`, `Vec3<T>`, `Vec4<T>`.
- Square Matrices: `Mat2<T>`, `Mat3<T>`, `Mat4<T>`.
- Quaternions: `Quat<T>`.
- Affine Transforms: `Affine2<T>`, `Affine3<T>`.
- Masks: `Mask2<T>`, `Mask3<T>`, `Mask4<T>`.

## SIMD

Appropriate types have increased memory alignment in order to take advantage of
SIMD instructions that improve performance. For example, `Vec3<f32>`,
`Vec4<f32>`, `Mat3<f32>` and `Mat4<f32>` are aligned to 16 bytes on x86 targets
in order to take advantage of the SSE instruction set.

Although SIMD alignment generally results better performance, it can also result
in wasted space. For example, due to 16-byte alignment, `Vec3<f32>` has 4 bytes
of padding, and consequently `Mat3<f32>` has 12 bytes of padding. For scenarios
where better performance is not worth wasted space, math types have non-SIMD,
unaligned variants:

- Vectors: `Vec2U<T>`, `Vec3U<T>`, `Vec4U<T>`.
- Square Matrices: `Mat2U<T>`, `Mat3U<T>`, `Mat4U<T>`.
- Quaternions: `QuatU<T>`.
- Affine Transforms: `Affine2U<T>`, `Affine3U<T>`.
- Masks: `Mask2U<T>`, `Mask3U<T>`, `Mask4U<T>`.

Unaligned types are optimal in memory-critical scenarios, for example when
storing 3D models. In all other cases, aligned types are optimal and result in
better performance than unaligned types.

Currently SIMD optimizations are only implemented for `f32` types on x86
targets. These types are closely benchmarked against [`glam`] and generally
match its performance.

Integration with [`wide`] enables SoA ([Structure of Arrays]) SIMD, which lets
you perform operations concurrently on multiple values, for example with
`Vec3<f32x4>` which represents four values of `Vec3<f32>`. SoA requires modeling
algorithms in a very specific way, but can be much faster than normal types.

## Generics

Because types are generic over `T`, they support non-primitive scalar types.
Integration with [`fixed`] enables support for fixed-point numbers, and
integration with [`wide`] enables support for SoA.

When Rust's type system is powerful enough, integration with [`num-primitive`]
will enable writing math code that is generic over primitive types, for example
functions generic over `T: PrimitiveFloat` will have access to float-vector
functionality.

Types relative to each other (e.g., `Vec2<T>`, `Vec3<T>`, `Vec4<T>` and
unaligned variants) are not distinct types, instead they are all type aliases to
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

Vectors are treated as column matrices, meaning when transforming a vector with
a matrix, the matrix goes on the left.

Matrices are stored in column-major order, meaning each column is continuous in
memory.

Angles are in radians, but can be converted to and from degrees using
standard-library functions.

## Development status

`ggmath` is not mature yet but is under active development.

Feature List:

- [x] Vectors
- [x] Square Matrices
- [x] Quaternions
- [x] Affine Transforms
- [x] Masks
- [x] Sufficient Float-Vector functionality
- [x] Sufficient Int-Vector functionality
- [ ] Sufficient Matrix functionality
- [ ] Sufficient Quaternion functionality
- [ ] Sufficient Affine functionality

Crate Support:

- [x] [`bytemuck`](https://crates.io/crates/bytemuck)
- [ ] [`fixed`](https://crates.io/crates/fixed) (partially done)
- [ ] [`fixp`](https://crates.io/crates/fixp) (partially done)
- [x] [`libm`](https://crates.io/crates/libm)
- [x] [`mint`](https://crates.io/crates/mint)
- [x] [`serde`](https://crates.io/crates/serde)
- [ ] [`wide`](https://crates.io/crates/wide) (partially done)
- [ ] [`rand`](https://crates.io/crates/rand)

Performance:

- [x] `f32` SSE2 optimizations
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

Rust must be updated to version `1.90.0` or later.

Add this to your Cargo.toml:

```toml
[dependencies]
ggmath = "0.16.2"
```

For `no_std` support, enable the `libm` feature:

```toml
[dependencies]
ggmath = { version = "0.16.2", features = ["libm"] }
```

For `no_std` without `libm`, disable default features:

```toml
[dependencies]
ggmath = { version = "0.16.2", default-features = false }
```

Without `std` or `libm`, the crate compiles but all float functionality that
relies on a backend is disabled.

## Optional features

- `std` (default feature): Uses `std` as the backend for float functionality.

- `assertions`: Enables assertions in release mode. Assertions are panics that
  catch invalid input and are enabled by default in debug mode.

- `no-assertions`: Disables assertions in debug mode. Assertions should only be
  controlled by binary crates. Library crates should not set this flag directly.

- `bytemuck`: Implements `bytemuck` traits for `ggmath` types.

- `fixed`: Implements `Scalar` for fixed-point numbers.

- `fixp`: Implements `Scalar` for fixed-point numbers.

- `libm`: Uses `libm` as the backend for float functionality. This makes the
  crate `no_std` even if the `std` feature is not disabled.

- `mint`: Implements conversions between `ggmath` and `mint` types.

- `serde`: Implements `Serialize` and `Deserialize` for `ggmath` types.

- `wide`: Implements `Scalar` for SIMD types.

## License

Licensed under either Apache License Version 2.0 or MIT license at your option.

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).

Contributions in any form (issues, pull requests, etc.) to this project must
adhere to Rust's
[Code of Conduct](https://rust-lang.org/policies/code-of-conduct/).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Attribution

`ggmath` is heavily inspired by [`glam`] and ports most of its code from it, as
it serves the same purpose as `glam` but with generics.

[`glam`]: https://crates.io/crates/glam
[`wide`]: https://crates.io/crates/wide
[`fixed`]: https://crates.io/crates/fixed
[`num-primitive`]: https://crates.io/crates/num-primitive
[Structure of Arrays]: https://en.wikipedia.org/wiki/AoS_and_SoA
