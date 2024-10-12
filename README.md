*** everything here is unfinished and not ready for production, and anything mentioned in the README may not be implemented yet. ***

[Trello board](https://trello.com/b/6NH6VXTh/gomath)

# GGMath - Generic Graphics Math
a generic graphics-math Rust crate

* simple API - when not using generics, the API is as simple as writing:
``` rust
fn main() {
  let _2_4_: FVec2 = vec2((2.0, 4.0));
  let vec: FVec4 = vec4((1.0, _2_4_, 3.0)).xywz();

  assert_eq!(vec, ve4(1.0, 2.0, 3.0, 4.0));
}
```

* powerful - has a fully generic API. for example: vectors are generic over length, scalar type, and storage (packed, aligned)
``` rust
struct MyStruct<const N: usize, S: VecStorage, T: Scalar> where ScalarCount<N>: VecLen<N> {
  gg: Vector<N, S, T>,
  og: VecN<N, T>,
  go: Vec3S<S, f32>,
  //oo: not in rust!
}
```

* flexible - doesn't force redundant restrictions. for example: supports both column-major and row-major matricies
``` rust
fn example(_mat: Mat4<RowMajor>) {}
```

* 0-cost abstraction - the built code is fully optimized with SIMD for scalars and targets that support it

restrictions:
* only supports static vectors (2, 3, 4) and matricies (2, 3, 4)x(2, 3, 4)
* will not support const-contexts beyond simple consts like ZERO and ONE on stable rust until const-traits are stablized

## Scalar

the ```Scalar``` trait allows types to be put inside math types (vecs, mats...) and has sub traits:

* operators: ScalarAdd, ScalarNot...
* num traits: ScalarFloat, ScalarSigned...

## Vector

the ```Vector``` struct is generic over:
* &lt;const N: usize&gt; where ScalarCount&lt;N&gt;: VecLen&lt;N&gt; - only 2, 3, and 4 are vector lengths
* &lt;S: VecStorage&gt; - VecAligned is faster, VecPacked saves memory
* &lt;T: Scalar&gt;

don't want to worry about Storage? use VecN&lt;N, T&gt; (type alias)

don't want to be generic over N? use Vec2/3/4&lt;T&gt;

don't want to be generic? use FVec2/IVec3/BVec4

## Matrix

the ```Matrix``` struct is generic over:
* &lt;const C: usize&gt; where ScalarCount&lt;C&gt;: VecLen&lt;C&gt; - column count
* &lt;const R: usize&gt; where ScalarCount&lt;R&gt;: VecLen&lt;R&gt; - row count
* &lt;M: MatrixMajorAxis&gt; - internally row-major or column major?
* &lt;S: VecStorage&gt; - a matrix is built off of vectors...
* &lt;T: Scalar&gt;

don't want to worry about Storage? use MatCxR&lt;C, R, M, T&gt;

don't want to worry about MatrixMajorAxis? use column_major/row_major/generic_major::MatCxR&lt;C, R, T&gt;

don't want to be generic over CxR? use Mat2/Mat4x3&lt;T&gt;

don't want to be generic? use FMat2/IMat3/BMat4

## GPU integration

when making Vertex structs, use VecPacked to save space

when making Uniform structs, use VecAligned which has size & alignment guarentees that uniforms require
