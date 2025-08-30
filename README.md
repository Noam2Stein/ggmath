# GGMath

[![Crates.io](https://img.shields.io/crates/v/ggmath.svg)](https://crates.io/crates/ggmath)
[![Docs.rs](https://docs.rs/ggmath/badge.svg)](https://docs.rs/ggmath)

"generic graphics math"

A Rust graphics math crate with heavily generic math types that fully support SIMD.

```rust ignore
use std::ops::{Add, Mul};

use ggmath::{f32_aliases::*, vector::*};

// does not add boilerplate when not using generics
fn main() {
    let vector: FVec4 = vec4!(1.0, vec2!(2.0, 3.0), 4.0);
    println!("{}", vector.zzy());
}

// supports generics over vector type and length, ... and whether the vector is SIMD aligned or not
fn generic_fn<const N: usize, T: Scalar, A: VecAlignment>(vector: Vector<N, T, A>) -> Vector<N, T, A>
where
    Usize<N>: VecLen,
    T: Add<Output = T> + Mul<Output = T>,
{
    vector + vector * vector
}
```

## Features

`ggmath` supports vectors, matrices, quaternions, and aabbs.

For vectors, `ggmath` defines a single `Vector` type that is generic over dimension, scalar type,
and alignment (whether the vector is SIMD aligned or not).

For matrices, `ggmath` defines a single `Matrix` type that is generic over dimensions, scalar type,
alignment (SIMD or not), and major axis (row-major / column-major).

The heavy use of generics provides:
- it allows for flexible types
- it makes creating new scalar types easier
- it can reduce code duplication

Additionally, when not needing generics,
type aliases make the API simple and very similar to other math crates like `glam`.

`ggmath` is also a zero-cost abstraction.
It usually results in the same assembly as other,
non-generic math crates that use SIMD.
This is achieved by specializing the implementation of vector methods for each scalar type.

## Installation

Add this to your `Cargo.toml`:

```toml ignore
[dependencies]
ggmath = "0.13.0"
```

## Usage

For most use cases where generics are not needed,
you should use type aliases like `FVec3`, `FMat3C`, etc.
When using type aliases the API is very similar to other math crates like `glam`.

For more advanced use cases where generics are needed,
you should read the documentation of types like `Vector`.

## Cargo Features

Default features:
- `vector`
- `matrix`
- `quaternion`
- `aliases` enables type aliases like `Vec3<T>`
- `primitive_aliases` enables primitive-specific type aliases like `FVec3`

Optional features:
- `aabb` enables the `Aabb` type (Axis Aligned Bounding Box)

- `right`, `left`, `up`, `down`, `forwards`, and `backwards`
enable direction constants where the respective direction is the positive direction of its axis.
For example, `right` enables `RIGHT` and `LEFT` constants where right is the positive direction.

- `crevice` and `serde` enable integration with the respective crates

## Dependencies

`ggmath` has no dependencies other than optional features that integrate with other crates,
and the `paste` crate which is used in the `aliases` feature to concat idents in a macro that generates type aliases.