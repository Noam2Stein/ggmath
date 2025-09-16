⚠️ Beware!
This README promises features that are not yet implemented,
and will be implemented before uploading to crates.io.

# GGMath

[![Crates.io](https://img.shields.io/crates/v/ggmath.svg)](https://crates.io/crates/ggmath)
[![Docs.rs](https://docs.rs/ggmath/badge.svg)](https://docs.rs/ggmath)

"generic graphics math"

A math crate for games and graphics with heavily generic math types and SIMD.

## Features

- vectors
- matrices
- quaternions
- AABBs (via optional feature)

`ggmath` stands out for two key features:

1. Generic types

Instead of having multiple types for each math structure (e.g., `Vec2`, `Vec3`, `Vec4`...),
there is only one generic type for each (e.g., `Vector<N, T, A>`),
and type aliases of course.

This approach has multiple benefits:
- Custom scalar types automatically have vector functionality
- Flexible types (e.g., both column-major and row-major matrices)
- Less code duplication

2. SIMD

Math crates with generics usually do not use SIMD,
and performance suffers as a result.

`ggmath` types are SIMD-aligned and use explicit SIMD operations.
This means that `ggmath` beats other generic math crates in performance.

## Installation

Add this to your `Cargo.toml`:

```toml ignore
[dependencies]
ggmath = "0.13.0"
```

## Cargo Features

Default features:
- `vector`
- `matrix`
- `quaternion`
- `primitive_aliases` enables primitive-specific type aliases like `FVec3`
- `swizzle` enables functions for all swizzle combinations

Optional features:
- `aabb` enables the `Aabb` type (Axis-Aligned Bounding Box)

- `right`, `left`, `up`, `down`, `forwards`, and `backwards`
enable direction constants where the respective direction is positive.
For example, `right` enables `RIGHT` and `LEFT` constants where right is positive and left is negative.

- `crevice` and `serde` enable integration with their respective crates
