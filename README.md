Beware!
This README promises features that are not yet implemented,
and will be implemented before uploading to crates.io.

# GGMath

[![Crates.io](https://img.shields.io/crates/v/ggmath.svg)](https://crates.io/crates/ggmath)
[![Docs.rs](https://docs.rs/ggmath/badge.svg)](https://docs.rs/ggmath)

"generic graphics math"

A math crate for games and graphics with heavily generic math types that use SIMD.

## Features

`ggmath` has a generic `Vector` type, `Matrix` type, `Quaternion` type, and `Aabb` type (opt-in).
These types are generic over their attributes, like dimension, scalar type, etc.

Every type can be either SIMD-backed or scalar-backed through a generic parameter that enables/disables SIMD.

`ggmath` is NEVER opinionated:
- You can use both column-major and row-major matrices
- You can opt in to right-handed / left-handed coordinate systems
- You can represent AABBs in a lot of ways (min-max, center-extents, etc.)

Additionally,
you don't actually need to write long generic type names,
like `Matrix<3, 3, f32, VecAligned, RowMajor>`.
All types have short type aliases, like `FMat3R` for the previous example.

## Performance

Because `ggmath` uses SIMD, it performs close to other SIMD crates like `glam`.

`ggmath` aims to benchmark all SIMD functions against `glam`, and to match `glam` in performance.
For functions that `glam` doesn't have, `ggmath` benchmarks against `wide`.

Explicit SIMD is achieved by allowing scalar types to override vector function implementations,
and is used to tune functions for better performance.

## Const Contexts

`ggmath` supports const contexts as much as is possible in present-day Rust.

Because of stable Rust limitations, most functions are not `const` directly.
Instead, those functions have `const` variants that are slower but can be called from const contexts.

## Installation

Add this to your `Cargo.toml`:

```toml ignore
[dependencies]
ggmath = "0.13.0"
```

## Usage

For non-generic use, look at the type-alias modules:
- `aliases` contains type aliases generic over `T`, like `Vec3<T>`
- `f32_aliases`, `u8_aliases`, etc. contain primitive-specific type aliases, like `FVec3`

For generic use, look at the central data types, like `Vector`, `Matrix`, etc.

## Cargo Features

Default features:
- `vector`
- `matrix`
- `quaternion`
- `aliases` enables type aliases generic over `T`, like `Vec3<T>`
- `primitive_aliases` enables primitive-specific type aliases like `FVec3`

Optional features:
- `aabb` enables the `Aabb` type (Axis Aligned Bounding Box)

- `right`, `left`, `up`, `down`, `forwards`, and `backwards`
enable direction constants where the respective direction is the positive direction of its axis.
For example, `right` enables `RIGHT` and `LEFT` constants where right is the positive direction.

- `crevice` and `serde` enable integration with their respective crates
