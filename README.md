Beware!
This readme promises features that are not yet implemented,
and will be implemented before uploading to crates.io.

# GGMath

[![Crates.io](https://img.shields.io/crates/v/ggmath.svg)](https://crates.io/crates/ggmath)
[![Docs.rs](https://docs.rs/ggmath/badge.svg)](https://docs.rs/ggmath)

"generic graphics math"

A Rust graphics math crate with heavily generic math types that fully support SIMD.

```rust ignore
use std::ops::{Add, Mul};

use ggmath::{f32_aliases::*, *};

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
- flexible types
- easier time creating new scalar types
- less code duplication

Additionally, when not needing generics,
type aliases make the API simple and very similar to other math crates.

## Performance

`ggmath` differs from other math crates with generics because it supports SIMD,
which is important for performance.

`ggmath` functions are benchmarked against `glam`, or `wide` for functions that `glam` doesn't have.
All benchmarked functions are verified to match `glam` and `wide` on x86,
and `ggmath` aims to benchmark all functions that use SIMD.

## Const Contexts

`ggmath` supports const contexts as much as is possible in present day Rust.
Right now alot of functions not `const` because of optimizations which require non-`const` functions,
but those functions have `const` variants that are slower but can be called from const contexts.

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
you should read the documentation of generic types like `Vector`.

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
