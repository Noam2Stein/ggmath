# `ggmath`

A math library for games and graphics with support for generics and SIMD.

The library features:

- Vectors: `Vec2<T>`, `Vec3<T>`, `Vec4<T>`.
- Square Matrices (todo): `Mat2<T>`, `Mat3<T>`, `Mat4<T>`.
- Quaternion (todo): `Quat<T>`.
- Affine Transformations (todo): `Affine2<T>`, `Affine3<T>`.
- Masks: `Mask2<T>`, `Mask3<T>`, `Mask4<T>`.

For appropriate scalars, these types are aligned for SIMD to improve
performance. The library also features unaligned types:

- Vectors: `Vec2U<T>`, `Vec3U<T>`, `Vec4U<T>`.
- Square Matrices (todo): `Mat2U<T>`, `Mat3U<T>`, `Mat4U<T>`.
- Quaternion (todo): `QuatU<T>`.
- Affine Transformations (todo): `Affine2U<T>`, `Affine3U<T>`.
- Masks: `Mask2U<T>`, `Mask3U<T>`, `Mask4U<T>`.

Unaligned types have the same functionality as aligned types, but are not
aligned for SIMD meaning they take less memory but have slower operations.

All types are type aliases to these generic structs:

- `Vector<N, T, A>`.
- `Matrix<N, T, A>` (todo).
- `Quaternion<T, A>` (todo).
- `Affine<N, T, A>` (todo).
- `Mask<N, T, A>`.

Where:

- `N` is the length (2, 3, or 4).
- `T` is the scalar type.
- `A` is either `Aligned` or `Unaligned`.

Generic structs are used to implement functionality for all lengths and or both
alignments without duplicating code or using macros.

## Development Status

`ggmath` is not mature yet but is under active development.

Feature List:

- [x] Vectors
- [ ] Square Matrices
- [ ] Quaternion
- [ ] Affine Transformations
- [x] Masks
- [x] Sufficient Float-Vector functionality
- [x] Sufficient Int-Vector functionality
- [ ] Sufficient Matrix functionality
- [ ] Sufficient Quaternion functionality
- [ ] Sufficient Affine functionality

Crate Support:

- [x] [`bytemuck`](crates.io/crates/bytemuck)
- [ ] [`fixed`](crates.io/crates/fixed)
- [x] [`libm`](crates.io/crates/libm)
- [x] [`mint`](crates.io/crates/mint)
- [x] [`serde`](crates.io/crates/serde)
- [ ] [`wide`](crates.io/crates/wide)

Performance:

- [x] `f32` SSE2 optimizations
- [ ] `f32` SSE4.2+ optimizations
- [ ] `f64` AVX+ optimizations
- [ ] `i32` `u32` SSE2+ optimizations
- [ ] `i8` `u8` `bool` SSE2+ optimizations for `Mat4<T>`
- [ ] `i16` `u16` AVX2+ optimizations for `Mat4<T>`
- [ ] `f32` NEON optimizations
- [ ] `i32` `u32` NEON optimizations
- [ ] `i8` `u8` `bool` NEON optimizations for `Mat4<T>`
- [ ] `f32` WASM optimizations
- [ ] `i32` `u32` WASM optimizations
- [ ] `i8` `u8` `bool` WASM optimizations for `Mat4<T>`

## Comparison to `glam`

`glam` is more mature than `ggmath`. This comparison assumes `ggmath` has
reached the maturity of `glam`. If you need any of the features missing
from `ggmath`, you should use `glam`.

Pros:

- Fixed-point: You can implement `Scalar` for your fixed-point numbers and use
  them inside vectors.

- SoA (Struct of Arrays): You can implement `Scalar` for your SIMD types and use
  them inside vectors (e.g., `Vec3<f32x4>`).

- Generic Code: You can write code that is generic over scalar type (`T`), and
  dimension (`N`).

Cons:

- Compile Times: Because of generics, `ggmath` code takes longer to compile than
  `glam` code.

- Complexity: `ggmath` isn't as simple as `glam`.

If your project requires `ggmath`'s features, you should pick `ggmath`. If it
doesn't, you should pick `glam`. For example, a game engine that needs to
support a wide range of use cases should pick `ggmath`, while a game that only
uses `f32` types should pick `glam`.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
ggmath = "0.15.1"
```

For `no_std` support, enable the `libm` feature:

```toml
[dependencies]
ggmath = { version = "0.15.1", features = ["libm"] }
```

For `no_std` without `libm`, disable default features:

```toml
[dependencies]
ggmath = { version = "0.15.1", default-features = false }
```

Without `std` or `libm`, the crate compiles but all float functionality that
relies on a backend is disabled.

## Optional Features

- `std` (default feature): Uses `std` as the backend for float functionality.

- `assertions`: Enables assertions in release mode. Assertions are panics that
  catch invalid input and are enabled by default in debug mode.

- `no-assertions`: Disables assertions in debug mode. Library crates should not
  directly enable `assertions` or `no-assertions` and should leave the decision
  to binary crates.

- `bytemuck`: Implements `bytemuck` traits for all `ggmath` types.

- `libm`: Uses `libm` as the backend for float functionality. This makes the
  crate `no_std` even if the `std` feature isn't disabled.

- `mint`: Implements conversions between `ggmath` and `mint` types.

- `serde`: Implements `Serialize` and `Deserialize` for all `ggmath` types.

## Attribution

The design of `ggmath` is heavily influenced by `glam`, as it serves the same
purpose as `glam` but with generics.

Most optimizations in `ggmath` are taken directly from `glam` and `wide`.

## License

Licensed under either Apache License Version 2.0 or MIT license at your option.
