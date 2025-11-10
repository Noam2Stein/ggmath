[![Crates.io](https://img.shields.io/crates/v/ggmath.svg)](https://crates.io/crates/ggmath)
[![Docs.rs](https://docs.rs/ggmath/badge.svg)](https://docs.rs/ggmath)

## `ggmath`

A games/graphics math library with generic SIMD types.

> This crate is a work in progress. Currently only vectors are implemented,
> matrices and quaternions are missing, many geometric vector functions are
> missing, and performance does not match `glam`.

////////////////////////////////////////////////////////////////////////////////

`ggmath` serves a similar purpose to [`glam`](https://crates.io/crates/glam) and  
[`nalgebra`](https://crates.io/crates/nalgebra). However, `ggmath` stands out by  
combining both **SIMD alignment** *and* **generic types**.

`glam` vectors are SIMD aligned, but they aren't generic over `T` or `N`. In  
contrast, `ggmath` provides a single parameterized type, `Vector<N, T>` (with  
type aliases such as `Vec3f`), which enables generic code and makes it easier to  
define custom scalar types.

`nalgebra` vectors are generic over both `T` and `N`, but they aren't SIMD
aligned. `ggmath` uses specialization to apply SIMD optimizations where  
possible and, when it improves performance, ensures SIMD alignment as well.

## Features

- Vectors

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
ggmath = "0.14.0"
```

## Cargo Features

Default Features:
- `std`: enables features that depend on `std` (disable for `no_std`).

Optional Features:
- `right`, `left`, `up`, `down`, `forwards`, and `backwards`: enable direction
  constants where the given direction is positive. For example, `right` enables
  `RIGHT` and `LEFT` constants where right is positive.

## License

Like most Rust crates, `ggmath` is licensed under the MIT License and the
Apache-2.0 License.
