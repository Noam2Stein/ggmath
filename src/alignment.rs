/// A marker type specifying SIMD alignment.
///
/// See [`Alignment`] for more details.
pub struct Aligned;

/// A marker type specifying lack of SIMD alignment.
///
/// See [`Alignment`] for more details.
pub struct Unaligned;

/// A marker trait controlling SIMD alignment for types.
///
/// Types like [`Vector<N, T, A>`] are generic over `A: Alignment` which
/// controls SIMD alignment and is either [`Aligned`] or [`Unaligned`].
///
/// Appropriate [`Aligned`] types have increased memory alignment in order to
/// take advantage of SIMD instructions that improve performance. For example,
/// [`Vec3<f32>`], [`Vec4<f32>`], [`Mat3<f32>`] and [`Mat4<f32>`] are aligned to
/// 16 bytes on x86 targets in order to take advantage of the SSE instruction
/// set.
///
/// Although SIMD alignment generally results better performance, it can also
/// result in wasted space. For example, due to 16-byte alignment, [`Vec3<f32>`]
/// has 4 bytes of padding, and consequently [`Mat3<f32>`] has 12 bytes of
/// padding.
///
/// [`Unaligned`] types do not have SIMD alignment. They are optimal when better
/// performance is not worth wasted space, for example when storing 3D models.
/// In all other cases, [`Aligned`] types are optimal and result in better
/// performance than [`Unaligned`] types.
///
/// [`Vector<N, T, A>`]: crate::Vector
/// [`Vec3<f32>`]: crate::Vec3
/// [`Vec4<f32>`]: crate::Vec4
/// [`Mat3<f32>`]: crate::Mat3
/// [`Mat4<f32>`]: crate::Mat4
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
