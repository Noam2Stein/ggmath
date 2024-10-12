*** everything here is unfinished and not ready for production, and anything mentioned in the README may not be implemented yet. ***

[Trello board](https://trello.com/b/6NH6VXTh/gomath)

# GGMath - generic-graphics-math
a generic graphics-math Rust crate

* flexible - doesn't force redundant restrictions. for example: supports both column-major and row-major matricies
* powerful - has a fully generic API. for example: vectors are generic over length (2, 3, 4), scalar type (f32, usize, bool...), and storage (packed, aligned)
* simple API - when not using generics, the API is as simple as writing ```let gg = fvec2(1.0, 2.0)```
* 0-cost abstraction - the built code is fully optimized with SIMD for scalars and targets that support it

restrictions:
* only supports static vectors (2, 3, 4) and matricies (2, 3, 4)x(2, 3, 4)
* will not support const-contexts beyond simple consts like ZERO and ONE on stable rust until const-traits are stablized

# features

core features:
* scalars
* vectors
* matricies
* quaternion

optional features:
* rectangles: { min, size }
* bounds: { center, extents }
* rays

nightly optional features:
* const - operations in const contexts

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

## column & row major matricies

there are both column-major matricies and row-major matricies. both impl a common trait specific for the size but abstract for the actual layout in memory so alot of code can be written generic over the layout.

## size & alignment guarentees

math types are guarenteed to follow certain size & alignment rules:

* size_of::<Vec2<T>>() == size_of::<[T; 2]>()
* size_of::<Vec3<T>>() == size_of::<[T; 4]>() *** [T; 4], not [T; 3]
* size_of::<Vec4<T>>() == size_of::<[T; 4]>()
* align_of::<Vec(N)<T>>() == align_of::<T>()

these rules align well with both SIMD requirements, and GPU requirements (for sending into uniforms...).
