# `ggmath`

A math library for games and graphics with support for generics and SIMD.

The library features:

- Vectors: `Vec2<T>`, `Vec3<T>`, `Vec4<T>`.
- Square Matrices (todo): `Mat2<T>`, `Mat3<T>`, `Mat4<T>`.
- Quaternion (todo): `Quat<T>`.
- Affine Transformations (todo): `Affine2<T>`, `Affine3<T>`.
- Masks (todo): `Mask2<T>`, `Mask3<T>`, `Mask4<T>`.

For appropriate scalars, these types are aligned for SIMD to improve
performance. The library also features unaligned types:

- Vectors: `Vec2U<T>`, `Vec3U<T>`, `Vec4U<T>`.
- Square Matrices (todo): `Mat2U<T>`, `Mat3U<T>`, `Mat4U<T>`.
- Quaternion (todo): `QuatU<T>`.
- Affine Transformations (todo): `Affine2U<T>`, `Affine3U<T>`.
- Masks (todo): `Mask2U<T>`, `Mask3U<T>`, `Mask4U<T>`.

Unaligned types have the same functionality as aligned types, but are not
aligned for SIMD meaning they take less memory but have slower operations.

All types are type aliases to these generic structs:

- `Vector<N, T, A>`.
- `Matrix<N, T, A>` (todo).
- `Quaternion<T, A>` (todo).
- `Affine<N, T, A>` (todo).
- `Mask<N, T, A>` (todo).

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
- [ ] Masks
- [x] Sufficient Float-Vector functionality
- [ ] Sufficient Int-Vector functionality
- [ ] Sufficient Matrix functionality
- [ ] Sufficient Quaternion functionality
- [ ] Sufficient Affine functionality

Crate Support:

- [x] [`bytemuck`](crates.io/crates/bytemuck)
- [ ] [`fixed`](crates.io/crates/fixed)
- [ ] [`libm`](crates.io/crates/libm)
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

The primary difference between the two crates is that `ggmath` uses generics and
`glam` intentionally doesn't. You should pick a crate based on these pros and
cons:

### Code Duplication

Generics eliminate code duplication by enabling code that is generic over scalar
type, vector length, and alignment.

This is primarily useful for writing math functions that need to work for
multiple scalar types, all lengths, or both SIMD and no SIMD. For example, a
cross platform deterministic implementation of `sin` would need to be duplicated
multiple times when using `glam`, but will only need to be written once using
`ggmath`.

### Custom Scalar Types

Generics make it possible to define custom scalar types.

This is primarily useful for either fixed point numbers (e.g., support for the
`fixed` crate), or SoA storage (Struct of Arrays) (e.g., `Vec3<f32x4>` using the
`wide` crate).

### Compile Times

To properly support generics, `ggmath` internally uses many language tricks that
unfortunately add work for the compiler not only when compiling `ggmath`, but
also when compiling math code that uses it.

If you need the fastest compile times possible you should use `glam`.

### Complexity

Generics make `ggmath` harder to understand in advance scenarios.

While the API is mostly similar (`Vec3<f32>` vs `Vec3A`), `ggmath` has some
advanced features that are harder to understand than anything in `glam` (e.g.,
the full generic form `Vector<N, T, A>`).

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
ggmath = "0.15.0"
```

For `no_std` support, disable default features:

```toml
[dependencies]
ggmath = { version = "0.15.0", default-features = false }
```

While `std` can be disabled, `libm` support is missing so `no_std` doesn't
support most float functionality.

## Optional Features

- `std` (default feature): Uses `std` as the backend for float functionality.

- `assertions`: Enables assertions in release mode. Assertions are panics that
  catch invalid input and are enabled by default in debug mode.

- `no-assertions`: Disables assertions in debug mode. Library crates should not
  directly enable `assertions` or `no-assertions` and should leave the decision
  to binary crates.

- `bytemuck`: Implements `bytemuck` traits for all `ggmath` types.

- `mint`: Implements conversions between `ggmath` and `mint` types.

- `serde`: Implements `Serialize` and `Deserialize` for all `ggmath` types.

## Attribution

The design of `ggmath` is heavily influenced by `glam`, as it serves the same
purpose as `glam` but with generics.

Most optimizations in `ggmath` are taken directly from `glam` and `wide`.

## License

Licensed under either Apache License Version 2.0 or MIT license at your option.
