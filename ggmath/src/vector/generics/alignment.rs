use super::*;

/// All `ggmath` types are generic over the *marker* type `A: VecAlignment`.
///
/// The value of `A` decides whether or not the vector is aligned for SIMD (see <https://doc.rust-lang.org/reference/type-layout.html>).
///
/// This marker trait is implemented by `VecAligned` and `VecPacked`.
///
/// `VecAligned` means that the vector is aligned for SIMD.
/// This does not promise a specific alignment.
/// The alignment is selected by the `Scalar` implementation,
/// to be whatever is most efficient for the target platform's SIMD.
///
/// `VecPacked` means that the vector is not aligned for SIMD, and is identical in memory to `[T; N]`.
#[allow(private_bounds)]
pub unsafe trait VecAlignment: Sized + 'static + Send + Sync {
    /// Is used internally by [`Vector`] to know if the vector is aligned in a generic function.
    const IS_ALIGNED: bool;

    /// Is used to "redirect" the vector to its alignment marker-type through this pattern:
    ///
    /// trait VecAlignment {
    ///     type Alignment<const N: usize, T: Scalar>: AlignTrait;
    /// }
    ///
    /// trait VecLen {
    ///     type Alignment<T: Scalar>: AlignTrait;
    /// }
    ///
    /// trait Scalar {
    ///     type Vec2Alignment: AlignTrait;
    ///     type Vec3Alignment: AlignTrait;
    ///     type Vec4Alignment: AlignTrait;
    /// }
    type Alignment<const N: usize, T: Scalar>: AlignTrait
    where
        Usize<N>: VecLen;
}

/// See [`VecAlignment`].
pub struct VecAligned;

/// See [`VecAlignment`].
pub struct VecPacked;

unsafe impl VecAlignment for VecAligned {
    const IS_ALIGNED: bool = true;

    type Alignment<const N: usize, T: Scalar>
        = <Usize<N> as VecLen>::Alignment<T>
    where
        Usize<N>: VecLen;
}

unsafe impl VecAlignment for VecPacked {
    const IS_ALIGNED: bool = false;

    type Alignment<const N: usize, T: Scalar>
        = Align<1>
    where
        Usize<N>: VecLen;
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Aligns the vector.
    /// - Cost: Nothing if `self` is already aligned. If `self` is packed, moves the vector to satisfy the alignment.
    #[inline(always)]
    pub const fn align(self) -> Vector<N, T, VecAligned> {
        self.to_layout()
    }

    /// "Unaligns" the vector.
    /// This just means casting the vector from `VecAligned` to `VecPacked`.
    ///
    /// This has no cost since an aligned vector always satisfies packed alignment.
    #[inline(always)]
    pub const fn unalign(self) -> Vector<N, T, VecPacked> {
        self.to_layout()
    }
}
