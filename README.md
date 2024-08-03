# gmath
A fast, flexible, and generic, rust math-library for games and graphics.

This library exists because the existing math-libraries had these missing features:

* Generic-types with 0 runtime-costs, proper traits and SIMD support.
* support for both column-major and row-major matricies.

## features

Types:
* Vectors:
  * Vec2
  * Vec3
  * Vec3A (memory-aligned)
  * Vec4
* Matricies:
  * Mat3
  * Mat3A
  * Mat4
* Quat

All of these types are generic over T: Element.
the 'Element' trait represents types that can be stored inside the library's data types.
'Element' is implemented by: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64 and bool.

There are sub-traits of 'Element' which add more constraints over T.

Element Traits:
* Num: Element - u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32 and f64.
* SignedNum: Num - i8, i16, i32, i64, i128, isize, f32 and f64.
* UnsignedNum: Num - u8, u16, u32, u64, u128 and usize.
* Int: Num - u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128 and isize.
* Float: SignedNum - f32 and f64.
* SignedInt: SignedNum + Int (auto-implemented).
* UnsignedInt: UnsignedNum + Int (auto-implemented).

