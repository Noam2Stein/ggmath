[![Crates.io](https://img.shields.io/crates/v/ggmath.svg)](https://crates.io/crates/ggmath)
[![Docs.rs](https://docs.rs/ggmath/badge.svg)](https://docs.rs/ggmath)

## `ggmath`

A games/graphics math library with generic SIMD types.

> This crate is a work in progress. currently only vectors are implemented,
> matrices and quaternions are missing, many geometric vector functions are
> missing, and performance does not match `glam`.

## Generics

Instead of having a separate vector/matrix/etc type for each combination of
dimensions and element type, `ggmath` has single `Vector`, `Matrix`,
`Quaternion`, and `Aabb` types that are parameterized over their properties.

This approach has several advantages: it allows code to be generic over
dimensions or element type, and makes it easy for users to define their own
custom element types.

For non-generic use cases, `ggmath` has convenient type aliases like `FVec3`,
that make the API simpler to use.

## SIMD

`ggmath` allows vector element types to override the storage and function
implementations of vectors in order to enable SIMD optimizations.

Primitive types like `f32`, `f64`, and `u64` have SIMD backends on appropriate
build targets.

Custom element types can also take advantage of SIMD without using low-level
intrinsics by reusing existing primitive vector types as backends.

## When to Use This Crate

`ggmath` is a good choice for projects that need functionality similar to
[`glam`], but with **generic types** and support for **custom scalar types**.
If your project involves general-purpose linear algebra, you may find
[`nalgebra`] more suitable.

Compared to `glam`, `ggmath` can be a better option when your project requires
any form of generics, custom numeric types, or SIMD optimizations for integer
types. On the other hand, `glam` is often a better choice for projects that
primarily use `f32` types and don't rely on generics, as it compiles faster
and offers a simpler API when working exclusively with `f32`.

Note that these recommendations reflect the intended, more complete version of
`ggmath`. In its current state, missing features and optimizations may be deal
breakers for ***many*** projects.

## Features

- Vectors of length 2, 3, or 4, with either SIMD or scalar backends
- Type aliases for primitive vectors
- Partially optimized SIMD backends for `f32`, `i32` and `u32`

Planned Features:
- Matrices
- Quaternions
- Aabbs
- Possibly rotors

## Benchmarks & Testing

`ggmath` is benchmarked against [`glam`](https://github.com/bitshifter/glam-rs)
and [`wide`](https://github.com/Lokathor/wide), using both microbenchmarks
(with [`gungraun`](https://github.com/gungraun/gungraun)) and larger,
composite benchmarks (with [`criterion`](https://github.com/bheisler/criterion.rs)).
Currently `ggmath` is not optimized and benchmarked enough to beat glam.

`ggmath` aims for high test coverage that ensures SIMD optimizations behave
correctly.

## Installation

A Rust compiler version of at least `1.90.0` is required.

Simply add the following to your `Cargo.toml`:

```toml
[dependencies]
ggmath = "0.13.1"
```

## Cargo Features

Default Features:
- `primitive_aliases`: enables type aliases for primitives (e.g., `FVec3`).
- `std`: enables features that depend on `std` (disable for `no_std`).
- `swizzle`: enables functions for all swizzle combinations (e.g., `wzyx`).

Optional Features:
- `right`, `left`, `up`, `down`, `forwards`, and `backwards`: enable direction
  constants where the given direction is positive. For example, `right` enables
  `RIGHT` and `LEFT` constants where right is positive.

## License

Like most Rust crates, `ggmath` is licensed under the MIT License and the
Apache-2.0 License.