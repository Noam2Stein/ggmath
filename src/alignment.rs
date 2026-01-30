/// Marker type indicating SIMD-aligned math types.
///
/// For more details, see [`Alignment`].
pub struct Aligned;

/// Marker type indicating non SIMD-aligned math types.
///
/// For more details, see [`Alignment`].
pub struct Unaligned;

/// Marker trait controlling SIMD alignment for math types.
///
/// Math types like [`Vector`](crate::Vector) are generic over `A: Alignment`,
/// which can be one of two marker types:
///
/// - [`Aligned`]: The type may be SIMD-aligned when doing so improves
///   performance.
/// - [`Unaligned`]: The type is never SIMD-aligned.
///
/// The default type aliases (e.g., [`Vec2<T>`](crate::Vec2),
/// [`Vec3<T>`](crate::Vec3)) are `Aligned`. Aliases ending in `U` (e.g.,
/// [`Vec2U<T>`](crate::Vec2U), [`Vec3U<T>`](crate::Vec3U)) are `Unaligned`.
///
/// # Background
///
/// [SIMD instructions](https://en.wikipedia.org/wiki/Single_instruction,_multiple_data)
/// can significantly improve the performance of math code, but they require
/// stricter
/// [memory alignment](https://doc.rust-lang.org/reference/type-layout.html)
/// than scalar code.
///
/// For example, `Vec3<f32>` must be aligned to 16 bytes for it to take
/// advantage of SIMD. This increases its size from 12 bytes to 16 bytes,
/// meaning it has 4 bytes of padding.
///
/// In many cases padding is worthwhile for performance, but in other cases such
/// as storing large arrays, minimizing memory usage is more important.
///
/// To support both cases, math types like `Vector` are generic over
/// `A: Alignment`, allowing SIMD alignment to be enabled or disabled.
#[expect(private_bounds)]
pub trait Alignment: Sealed {
    #[doc(hidden)]
    const IS_ALIGNED: bool;

    #[doc(hidden)]
    type Select<A: Copy, U: Copy>: Copy;
}

impl Alignment for Aligned {
    const IS_ALIGNED: bool = true;

    type Select<A: Copy, U: Copy> = A;
}

impl Alignment for Unaligned {
    const IS_ALIGNED: bool = false;

    type Select<A: Copy, U: Copy> = U;
}

trait Sealed: 'static {}

impl Sealed for Aligned {}
impl Sealed for Unaligned {}
