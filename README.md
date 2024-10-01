*** everything here is unfinished and not ready for production, and anything mentioned in the README may not be implemented yet. ***

[Trello board](https://trello.com/b/6NH6VXTh/gomath)

# gomath - generic-optimized-math
a generic, optimized, math rust-crate for games and graphics, that is flexible to be compatible with any crate's needs.

advantages:
* generic types that are optimized with SIMD with no unsafe features like effects
* size & alignment guarentees designed to work well with the gpu
* both column-major and row-major matricies

disadvantages:
* const contexts - operations in const contexts are not supported on stable rust because of the const-traits feature.

## Features

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
