*** everything here is unfinished and not ready for production, and anything mentioned in the README may not be implemented yet. ***

[Trello board](https://trello.com/b/6NH6VXTh/ggmath)

# GGMath - Generic Graphics Math
A graphics-math Rust crate with powerful generics that are truely zero-cost

* simple API
``` rust
use ggmath::vector::*;

fn main() {
  let _2_4_ = vec2!(2.0_f32, 4.0);
  let vec = vec4(1.0, _2_4_, 3.0).xywz();

  assert_eq!(vec, vec4(1.0, 2.0, 3.0, 4.0));
}
```

* powerful generics
``` rust
use ggmath::vector::{f32_aliases::*, *};

struct MyStruct<const N: usize, T: Scalar, A: VecAlignment> where ScalarCount<N>: VecLen {
  gg: Vector<N, T, A>,
  go: Vector<N, T, VecAligned>,
  og: FVec3,
  //oo: go back to Java
}
```

* both column-major and row-major matricies
``` rust
use ggmath::matrix::*;

fn example(row_major: Mat4R, column_major: Mat2x3C) {}
```

* 0-cost abstraction - fully optimized with SIMD on current stable Rust

restrictions:
* only supports static vectors (2, 3, 4) and matricies (2, 3, 4)x(2, 3, 4)
* won't support const-contexts until const-traits are stablized

## `Scalar`

the ```Scalar``` trait allows types to be put inside math types (vectors, matricies...) and has sub traits:

* operators: `ScalarAdd`, `ScalarBitOr`...
* num traits: `ScalarFloat`, `ScalarSigned`...

## `Vector`

the ```Vector``` struct is generic over:
* `<const N: usize> where ScalarCount<N>: VecLen` - only 2, 3, and 4 are vector lengths
* `<T: Scalar>`
* `<A: VecAlignment>` - doesn't affect API, VecAligned is faster, VecPacked saves memory

don't want to be generic? use type aliases! `Vec2<f32>`/`FVec2`/`IVec3`/`BVec4`

## `Matrix`

the ```Matrix``` struct is generic over:
* `<const C: usize> where ScalarCount<C>: VecLen` - column count
* `<const R: usize> where ScalarCount<R>: VecLen` - row count
* `<M: MatrixMajorAxis>` - internally row-major or column major?
* `<T: Scalar>`
* `<A: VecAlignment>` - a matrix is built off of vectors...

don't want to be generic? use `column_major::Mat3<f32>`/`row_major::FMat4x2`

## GPU integration

GGMath is built with GPU struct integration in mind.

when making Vertex structs, use `VecPacked` to save space

when making Uniform structs, use `VecAligned` which has size & alignment guarentees that uniforms require
