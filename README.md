# `ggmath`

A math library for games and graphics with support for generics and SIMD.

The library features:

- Vectors: `Vec2<T>`, `Vec3<T>`, `Vec4<T>`.
- Square Matrices (todo): `Mat2<T>`, `Mat3<T>`, `Mat4<T>`.
- Quaternion (todo): `Quat<T>`.
- Affine Transformations (todo): `Affine2<T>`, `Affine3<T>`.
- Masks (todo): `Mask2<T>`, `Mask3<T>`, `Mask4<T>`.

For appropriate scalars these types are SIMD aligned and use SIMD
instructions to maximize performance.

As a result of SIMD alignment, some of these types have padding. For example,
`Vec3<f32>` is aligned to 16 bytes and as a result has 4-bytes of padding. To
fix this the library features unaligned types which don't have padding but are
not backed by SIMD:

- Vectors: `Vec2U<T>`, `Vec3U<T>`, `Vec4U<T>`.
- Square Matrices (todo): `Mat2U<T>`, `Mat3U<T>`, `Mat4U<T>`.
- Quaternion (todo): `QuatU<T>`.
- Affine Transformations (todo): `Affine2U<T>`, `Affine3U<T>`.
- Masks (todo): `Mask2U<T>`, `Mask3U<T>`, `Mask4U<T>`.

Unaligned types are useful for situations where saving memory is high priority,
for example when storing large arrays of these types.

All of these types are specific cases of these generic structs:

- `Vector<N, T, A>`.
- `Matrix<N, T, A>` (todo).
- `Quaternion<T, A>` (todo).
- `Affine<N, T, A>` (todo).
- `Mask<N, T, A>` (todo).

Where:

- `N` is the length (2, 3, or 4).
- `T` is the scalar type.
- `A` is either `Aligned` or `Unaligned`.

The full generic form of these types are useful to write code that is generic
over length and or alignment.

## Development Status

`ggmath` is not mature yet but is under active development.

Feature List:

- [x] Vectors
- [ ] Square Matrices
- [ ] Quaternion
- [ ] Affine Transformations
- [ ] Masks
- [ ] Sufficient Float-Vector functionality
- [ ] Sufficient Int-Vector functionality
- [ ] Sufficient Matrix functionality
- [ ] Sufficient Quaternion functionality
- [ ] Sufficient Affine functionality

Crate Support:

- [x] [`bytemuck`](crates.io/crates/bytemuck)
- [ ] [`libm`](crates.io/crates/libm)
- [ ] [`mint`](crates.io/crates/mint)
- [x] [`serde`](crates.io/crates/serde)
- [ ] [`zerocopy`](crates.io/crates/zerocopy)

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

## Cargo Features

- `std` (default feature): Uses `std` as the backend for float functionality.

- `assertions`: Enables assertions in release mode. Assertions are panics that
  catch invalid input and are enabled by default in debug mode.

- `no-assertions`: Disables assertions in debug mode. Library crates should not
  directly enable `assertions` or `no-assertions` and should leave the decision
  to binary crates.

- `bytemuck`: Implements `bytemuck` traits for all `ggmath` types.

- `serde`: Implements `Serialize` and `Deserialize` for all `ggmath` types.

## Dependencies

`ggmath` has no dependencies.

## License

Licensed under either Apache License Version 2.0 or MIT license at your option.
