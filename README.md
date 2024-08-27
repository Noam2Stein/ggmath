# ggmath
a **g**eneric-**g**ames-**math** crate which is flexible and heavily optimized.

This crate exists because the existing math-libraries had these missing features:

* generic types with SIMD support.
* both column-major and row-major matricies.

## Features

* the core of this crate contains vectors, matricies and quaternions. other data-types that are useful in game-development are optional features.

### Data-Types

* Vectors: Vec2, Vec3, Vec4 - generic over T: Element.
* Matricies: Mat3, Mat4 - generic over T: Element.
* Quat - generic over T: Float.

### Traits

the 'Element' trait represents types that can be stored inside the crate's data types.
'Element' is implemented by the bool, uint, int and float types.

each 'Element' has its own internal implementation of vector fns, for example: on supported platforms, the f32 'Element' uses SIMD vectors, and it's implementation of vector fns like addition and swizzle use swizzle to be more perforant.

'Element' has sub-traits which add more constraints over the type and allow data-types to implement more functions.

* Num: Element.
* SignedNum: Num.
* UnsignedNum: Num.
* Int: Num.
* Float: SignedNum.
* SignedInt: SignedNum + Int.
* UnsignedInt: UnsignedNum + Int.

### Matricies

Matricies can be represented either as column-major or row-major, and each variant is in a different module ('cm', 'rm').

Both of these variations implement matrix-traits that allow code to be generic over how matricies are stored.
for example a function that takes in a matrix could use 'impl Mat3' instead of 'cm::Mat3' or 'rm::Mat3'.

### Vector-Swizzle

* swizzle - getter.
* set_swizzle - sets the given components using a mutable reference.
* with_swizzle - returns a copy of self with the given components set.
* swizzle_mut - returns a reference (or multiple split references) to a continuous sub-section of the vector.
* constructor swizzle (building a vector out of existing vectors. for example: vec4((1.0, vec2((2.0, 3.0)), 4.0))).
