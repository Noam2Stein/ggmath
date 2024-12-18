*** UNFINISHED CRATE ***

# GGMath - Generic Graphics Math
graphics-math crate with powerful generics that are truely zero-cost

* simple API
``` rust
use ggmath::vector::*;

fn main() {
  let _2_4_ = vec2!(2.0_f32, 4.0);
  let vec = vec4!(1.0, _2_4_, 3.0).xywz();

  assert_eq!(vec, vec4!(1.0, 2.0, 3.0, 4.0));
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

The ```Scalar``` trait allows types to be put inside math types (vectors, matricies...)

The magic of this crate is that when you implement ```Scalar```,
you can override the implementation of ```Vector``` functions to make optimizations

* All ```Vector``` functions that are possible to optimize can be overriden

## `Vector`

the ```Vector``` struct is generic over:
* `<const N: usize> where ScalarCount<N>: VecLen` - only 2, 3, and 4 are valid vector lengths
* `<T: Scalar>` - f32? u8? isize? CustomF256?
* `<A: VecAlignment>` - doesn't affect API, ```VecAligned``` is faster, ```VecPacked``` saves memory

don't want to be generic? use type aliases! `Vec2<f32>`/`FVec2`/`IVec3`/`BVec4`

## `Matrix`

the ```Matrix``` struct is generic over:
* `<const C: usize> where ScalarCount<C>: VecLen` - column count
* `<const R: usize> where ScalarCount<R>: VecLen` - row count
* `<T: Scalar>` - "f32? u8? isize? CustomF256?"
* `<A: VecAlignment>` - a matrix is built of vectors...
* `<M: MatrixMajorAxis>` - internally row-major or column major?

don't want to be generic? use `Mat3C<f32>`/`row_major::FMat4x2RP`

## GPU integration

GGMath is built with GPU struct integration in mind.

when making Vertex structs, use `VecPacked` to save space

when making Uniform structs, use `VecAligned` which has size & alignment rules that uniforms require

## Type Aliases

ggmath's type aliases have naming rules:

### Vectors:

```Vec'N'<T>``` -> ```Vector<'N', T, VecAligned>```
For example: ```Vec2<f32>``` -> ```Vector<2, f32, VecAligned>```

```Vec'N'P<T>``` -> ```Vector<'N', T, VecPacked>```
For example: ```Vec4P<f32>``` -> ```Vector<4, f32, VecPacked>```

```'T'Vec'N'``` -> ```Vector<'N', 'T', VecAligned>```
For example: ```FVec2``` -> ```Vector<2, f32, VecAligned>```

```'T'Vec'N'P``` -> ```Vector<'N', 'T', VecPacked>```
For example: ```U8Vec3P``` -> ```Vector<3, u8, VecPacked>```

### Matricies

```Mat'C'x'R'C<T>``` -> ```Matrix<'C', 'R', T, VecAligned, ColumnMajor>```
For example: ```Mat2x3C<f32>``` -> ```Matrix<2, 3, f32, VecAligned, ColumnMajor>```

```Mat'C'x'R'R<T>``` -> ```Matrix<'C', 'R', T, VecAligned, RowMajor>```
For example: ```Mat4x3R<bool>``` -> ```Matrix<4, 3, bool, VecAligned, RowMajor>```

```Mat'C'x'R'CP<T>``` -> ```Matrix<'C', 'R', T, VecPacked, ColumnMajor>```
For example: ```Mat3CP<f64>``` -> ```Matrix<3, 3, f64, VecPacked, ColumnMajor>```

```Mat'C'x'R'RP<T>```, ```'T'Mat'C'x'R'C```...