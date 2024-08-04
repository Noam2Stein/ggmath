# gmath
A fast, flexible, and generic, rust math-library for games and graphics.

This library exists because the existing math-libraries had these missing features:

* Generic-typing with maximum performance, proper traits and SIMD support.
* Support for both column-major and row-major matricies.

## Features

### Data-Types

Vectors, matricies and quaternions are supported.

Types:
* Vectors:
  * Vec2
  * Vec3
  * Vec3A (memory-aligned for SIMD)
  * Vec4
* Matricies:
  * Mat3
  * Mat3A
  * Mat4
* Quat

All of these types are generic over T: Element.

### Traits

the 'Element' trait represents types that can be stored inside the library's data types.
'Element' is implemented by: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64 and bool.

There are sub-traits of 'Element' which add more constraints over T and allow the data-types to implement more functions.

Element sub-traits:
* Num: Element.
* SignedNum: Num.
* UnsignedNum: Num.
* Int: Num.
* Float: SignedNum.
* SignedInt: SignedNum + Int.
* UnsignedInt: UnsignedNum + Int.

### Matricies

Matricies can be represented either as column-major or row-major, and each variant is in a different module ('cm', 'rm').

Both of these variations implement Mat traits that allow code to be generic over how matricies are stored.

### Swizzle

Vectors support swizzling with 4 variations:
* swizzle (getter).
* set_swizzle (sets the given components using a mutable reference).
* with_swizzle (returns a copy of self with the given components set).
* swizzle_mut (returns a reference to a continuous sub-section of the vec as a smaller vec-type).
* constructor swizzle (combining vectors and elements to construct a new vector for example: vec4((1.0, vec2((2.0, 3.0)), 4.0))).

## Modules

* mod element
* mod vec
* mod mat
  * cm
  * rm
* mod quat
* use element::*
* use vec::*
* use mat::*
* use quat::*
