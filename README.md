# `ggmath`

A math crate for games/graphics with generics and SIMD.

Note: This crate is a work in progress, and several features are missing.

# Overview

### Generics

`ggmath` provides generic math types such as `Vector<N, T, A>`, with convenient
type aliases like `Vec2f`, `Vec3fA`, and others.

The API is familiar, resembling other math libraries, but the use of generics:
- enables usage in generic contexts
- adds support for user-defined scalar types

### SIMD

Most math crates with generics do not support SIMD-aligned types, making them
slower than SIMD-based crates that lack generics.

`ggmath` supports both:
- SIMD-aligned types (`Vector<N, T, Aligned>`, e.g., `Vec3fA`)
- unaligned types (`Vector<N, T, Unaligned>`, e.g., `Vec3f`)

This flexibility allows `ggmath` to match the performance of SIMD-backed math
crates that lack generics.

# Features

- vectors
- `no_std` support

### Missing Features

- vectors: geometric functions for floats
- vectors: functions for integers
- vectors: functions for `bool`
- matrices
- quaternions
- masks
- `libm` support

# Performance

`ggmath` is designed to match the performance of highly optimized SIMD math
crates (like [glam](https://github.com/bitshifter/glam-rs)), though some (many!)
optimizations are still missing.

|       | x86 | ARM | WASM |
| ----- | --- | --- | ---- |
| `f32` | ❌ | ❌ | ❌ |
| `f64` | ❌ | ❌ | ❌ |
| `i32` | ❌ | ❌ | ❌ |
| `i64` | ❌ | ❌ | ❌ |
| `u32` | ❌ | ❌ | ❌ |
| `u64` | ❌ | ❌ | ❌ |

# `no_std` Support

`no_std` support can be enabled by turning off default features:

```toml
[dependencies]
ggmath = { version = "0.14.0", default-features = false, features = ["debug_assertions", "debug_overflow_checks"] }
```

Note: `libm` support is currently missing.

# Cargo Features

- `std`: uses `std` as a backend for floats.

- `assertions`: enables assertions which check the validity of arguments.

- `debug_assertions`: enables `assertions` in debug builds.

- `overflow_checks`: enables overflow checks for integer types.

- `debug_overflow_checks`: enables `overflow_checks` in debug builds.

`std`, `debug_assertions`, and `debug_overflow_checks` are enabled by default.

# Minimum Supported Rust Version

`ggmath` requires Rust `1.90.0`.

# License

`ggmath` is licensed under MIT OR Apache-2.0
