*** everything here is unfinished and not ready for production, and anything mentioned in the README may not be implemented yet. ***

[Trello board](https://trello.com/b/6NH6VXTh/gomath)

# gomath - generic-optimized-math
a generic, optimized, math rust-crate for games and graphics, that is flexible to be compatible with any crate's needs.

advantages:
* generic types that are optimized with SIMD with no unsafe features like effects
* both column-major and row-major matricies
* size & alignment guarentees designed to work well with the gpu

# features

core features:
* elements: primitives
* vectors: Vec(2-4)<T: Element>
* matricies: Mat(2-4)x(2-4)<T: Element>
* quaternion: Quat<T: ElementFloat>

optional features:
* rectangles: Rect(2-4) - { min, size }
* bounds: Bounds(2-4) - { center, extents }
* rays: Ray(2-4)

nightly optional features:
* const - operations in const contexts

## Element

the Element trait allows types to be put inside math types (vecs, mats...) and has sub traits:

* operators: ElementAdd, ElementNot... - allows vecs to impl the same operators
* primitive categories: ElementFloat, ElementSigned... - allows abstracting over which type of float for example

## optimized-generic vectors

Vec2<T>, Vec3<T> and Vec4<T> are just transparent wrappers around their InnerVec(N) counterparts which are vector backends specified by the Element.

so each Element has its own vector backend. so on some platforms an f32 will use a SIMD impl while on other platforms that dont support SIMD f32 will use the ElementDefaultImpl which uses scalars and arrays.

## column & row major matricies

there are both column-major matricies and row-major matricies. both impl a common trait specific for the size but abstract for the actual layout in memory so alot of code can be written generic over the layout.

## size & alignment guarentees

math types are guarenteed to follow certain size & alignment rules:

* size_of::<Vec2<T>>() == size_of::<[T; 2]>()
* size_of::<Vec3<T>>() == size_of::<[T; 4]>() *** [T; 4], not [T; 3]
* size_of::<Vec4<T>>() == size_of::<[T; 4]>()
* align_of::<Vec(N)<T>>() == align_of::<T>()

these rules align well with both SIMD requirements, and GPU requirements (for sending into uniforms...).
