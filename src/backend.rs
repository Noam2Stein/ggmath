use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

use crate::{
    Aligned, Alignment, Length, Mask, PrimitiveFloat, PrimitiveInteger, PrimitiveSigned,
    Quaternion, Scalar, SupportedLength, Unaligned, Vector,
    utils::{Repr2, Repr3, Repr4},
};

cfg_select! {
    target_feature = "sse2" => {
        mod sse2;
    }
    all(target_arch = "aarch64", target_feature = "neon") => {
        mod neon;
    }
    _ => {
        mod fallback;
    }
}

pub(crate) trait DefaultBackend<const N: usize, A: Alignment>: Scalar {}

/// # Safety
///
/// [`Self::Inner`] must be implemented correctly. All other items are safe to
/// implement.
#[diagnostic::on_unimplemented(
    message = "`ggmath::Scalar` cannot be implemented directly",
    note = "see the documentation for `ggmath::Scalar`"
)]
pub(crate) unsafe trait VectorBackend<const N: usize, A: Alignment>
where
    Length<N>: SupportedLength,
{
    /// Controls the internal representation of [`Vector<N, Self, A>`].
    ///
    /// # Safety
    ///
    /// References to this type must be transmutable to and from references to
    /// `[T; N]`, meaning any bit-patterns accepted by `T` must be accepted by
    /// this type, and any bit-patterns accepted by this type must be accepted
    /// by `T`.
    ///
    /// For `A = Unaligned` this type must have the size and alignment of
    /// `[T; N]`.
    ///
    /// For `N = 2` and `N = 4` this type must have the size of `[T; N]` and may
    /// have additional alignment.
    ///
    /// For `N = 3, A = Aligned` this type must have the size of either `[T; 3]`
    /// or `[T; 4]` and may have additional alignment. If this type has the size
    /// of `[T; 4]` the padding must be initialized memory accept all
    /// bit-patterns.
    type Inner: Copy;

    fn vector_eq(vector: &Vector<N, Self, A>, other: &Vector<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq;

    fn vector_ne(vector: &Vector<N, Self, A>, other: &Vector<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq;

    #[track_caller]
    fn vector_neg(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Neg<Output = Self>;

    #[track_caller]
    fn vector_not(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Not<Output = Self>;

    #[track_caller]
    fn vector_add(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Add<Output = Self>;

    #[track_caller]
    fn vector_sub(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Sub<Output = Self>;

    #[track_caller]
    fn vector_mul(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Mul<Output = Self>;

    #[track_caller]
    fn vector_div(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Div<Output = Self>;

    #[track_caller]
    fn vector_rem(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Rem<Output = Self>;

    #[track_caller]
    fn vector_shl(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Shl<Output = Self>;

    #[track_caller]
    fn vector_shr(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Shr<Output = Self>;

    #[track_caller]
    fn vector_bitand(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitAnd<Output = Self>;

    #[track_caller]
    fn vector_bitor(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitOr<Output = Self>;

    #[track_caller]
    fn vector_bitxor(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitXor<Output = Self>;

    #[track_caller]
    fn vector_element_sum(vector: Vector<N, Self, A>) -> Self
    where
        Self: Scalar + Add<Output = Self>;

    #[track_caller]
    fn vector_element_product(vector: Vector<N, Self, A>) -> Self
    where
        Self: Scalar + Mul<Output = Self>;

    fn vector_eq_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialEq;

    fn vector_ne_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialEq;

    fn vector_lt_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialOrd;

    fn vector_gt_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialOrd;

    fn vector_le_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialOrd;

    fn vector_ge_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialOrd;
}

/// # Safety
///
/// The following statements must be true:
///
/// - `Inner` contains `Matrix<N, T, A>` followed by `Vector<N, T, A>` followed
///   by optional padding
///
/// - The optional padding satisfies the requirements of `Pod`, regardless of
///   whether `T` does
///
/// - `Inner` has the alignment of `Matrix<N, T, A>`
pub(crate) unsafe trait AffineBackend<const N: usize, A: Alignment> {
    type Inner: Copy;
}

pub(crate) trait QuaternionBackend<A: Alignment> {
    #[track_caller]
    fn quat_mul(quat: Quaternion<Self, A>, rhs: Quaternion<Self, A>) -> Quaternion<Self, A>
    where
        Self: Scalar
            + Neg<Output = Self>
            + Add<Output = Self>
            + Sub<Output = Self>
            + Mul<Output = Self>;
}

/// # Safety
///
/// [`Self::Inner`] must be implemented correctly. All other items are safe to
/// implement.
#[diagnostic::on_unimplemented(
    message = "`ggmath::Scalar` cannot be implemented directly",
    note = "see the documentation for `ggmath::Scalar`"
)]
pub(crate) unsafe trait MaskBackend<const N: usize, A: Alignment>
where
    Length<N>: SupportedLength,
{
    /// Controls the internal representation of [`Mask<N, Self, A>`].
    ///
    /// # Safety
    ///
    /// This type must only have initialized memory and must accept the zero
    /// bit-pattern.
    type Inner: Send + Sync + Copy;

    fn mask_from_array(array: [bool; N]) -> Mask<N, Self, A>
    where
        Self: Scalar;

    fn mask_splat(value: bool) -> Mask<N, Self, A>
    where
        Self: Scalar;

    fn mask_to_array(mask: Mask<N, Self, A>) -> [bool; N]
    where
        Self: Scalar;

    fn mask_all(mask: Mask<N, Self, A>) -> bool
    where
        Self: Scalar;

    fn mask_any(mask: Mask<N, Self, A>) -> bool
    where
        Self: Scalar;

    fn mask_select(
        mask: Mask<N, Self, A>,
        if_true: Vector<N, Self, A>,
        if_false: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Scalar;

    #[track_caller]
    fn mask_get(mask: Mask<N, Self, A>, index: usize) -> bool
    where
        Self: Scalar;

    #[track_caller]
    fn mask_set(mask: &mut Mask<N, Self, A>, index: usize, value: bool)
    where
        Self: Scalar;

    fn mask_eq(mask: &Mask<N, Self, A>, other: &Mask<N, Self, A>) -> bool
    where
        Self: Scalar;

    fn mask_ne(mask: &Mask<N, Self, A>, other: &Mask<N, Self, A>) -> bool
    where
        Self: Scalar;

    fn mask_not(mask: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar;

    fn mask_bitand(mask: Mask<N, Self, A>, rhs: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar;

    fn mask_bitor(mask: Mask<N, Self, A>, rhs: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar;

    fn mask_bitxor(mask: Mask<N, Self, A>, rhs: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar;
}

pub(crate) trait FloatVectorBackend<const N: usize, A: Alignment>: Scalar
where
    Length<N>: SupportedLength,
{
    fn vector_nan_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;

    fn vector_finite_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;

    fn vector_sign_positive_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;

    fn vector_sign_negative_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;

    fn vector_max(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_min(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_max_element(vector: Vector<N, Self, A>) -> Self;

    fn vector_min_element(vector: Vector<N, Self, A>) -> Self;

    fn vector_abs(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_signum(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_copysign(vector: Vector<N, Self, A>, sign: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_floor(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_ceil(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_round(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_trunc(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_mul_add(
        vector: Vector<N, Self, A>,
        a: Vector<N, Self, A>,
        b: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>;

    fn vector_div_euclid(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>)
    -> Vector<N, Self, A>;

    fn vector_rem_euclid(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>)
    -> Vector<N, Self, A>;

    fn vector_powf(vector: Vector<N, Self, A>, n: Self) -> Vector<N, Self, A>;

    fn vector_sqrt(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_exp(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_exp2(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_ln(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_log2(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_sin(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_cos(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_tan(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_asin(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_acos(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_atan(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_sin_cos(vector: Vector<N, Self, A>) -> (Vector<N, Self, A>, Vector<N, Self, A>);
}

pub(crate) trait IntegerVectorBackend<const N: usize, A: Alignment>: Scalar
where
    Length<N>: SupportedLength,
{
    fn vector_max(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_min(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_max_element(vector: Vector<N, Self, A>) -> Self;

    fn vector_min_element(vector: Vector<N, Self, A>) -> Self;

    fn vector_checked_add(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>;

    fn vector_checked_sub(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>;

    fn vector_checked_mul(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>;

    fn vector_checked_div(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>;

    fn vector_checked_rem(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>;

    fn vector_saturating_add(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>;

    fn vector_saturating_sub(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>;

    fn vector_saturating_mul(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>;

    #[track_caller]
    fn vector_saturating_div(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>;

    fn vector_wrapping_add(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>;

    fn vector_wrapping_sub(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>;

    fn vector_wrapping_mul(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>;

    #[track_caller]
    fn vector_wrapping_div(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>;

    #[track_caller]
    fn vector_wrapping_rem(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>;
}

pub(crate) trait SignedVectorBackend<const N: usize, A: Alignment>: Scalar
where
    Length<N>: SupportedLength,
{
    fn vector_wrapping_abs(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_signum(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn vector_positive_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;

    fn vector_negative_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;
}

impl DefaultBackend<2, Aligned> for f32 {}

impl<const N: usize> DefaultBackend<N, Unaligned> for f32 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for f64 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for i8 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for i16 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for i32 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for i64 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for i128 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for isize {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for u8 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for u16 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for u32 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for u64 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for u128 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for usize {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for bool {}

// SAFETY: `Inner` follows its requirements.
unsafe impl<T, A: Alignment> VectorBackend<2, A> for T
where
    T: DefaultBackend<2, A>,
{
    type Inner = Repr2<T>;

    #[inline]
    fn vector_eq(vector: &Vector<2, Self, A>, other: &Vector<2, Self, A>) -> bool
    where
        Self: PartialEq,
    {
        vector.x == other.x && vector.y == other.y
    }

    #[inline]
    fn vector_ne(vector: &Vector<2, Self, A>, other: &Vector<2, Self, A>) -> bool
    where
        Self: PartialEq,
    {
        !(vector == other)
    }

    #[inline]
    fn vector_neg(vector: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Neg<Output = Self>,
    {
        Vector::<2, Self, A>::new(-vector.x, -vector.y)
    }

    #[inline]
    fn vector_not(vector: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Not<Output = Self>,
    {
        Vector::<2, Self, A>::new(!vector.x, !vector.y)
    }

    #[inline]
    fn vector_add(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Add<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x + rhs.x, vector.y + rhs.y)
    }

    #[inline]
    fn vector_sub(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Sub<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x - rhs.x, vector.y - rhs.y)
    }

    #[inline]
    fn vector_mul(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Mul<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x * rhs.x, vector.y * rhs.y)
    }

    #[inline]
    fn vector_div(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Div<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x / rhs.x, vector.y / rhs.y)
    }

    #[inline]
    fn vector_rem(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Rem<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x % rhs.x, vector.y % rhs.y)
    }

    #[inline]
    fn vector_shl(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Shl<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x << rhs.x, vector.y << rhs.y)
    }

    #[inline]
    fn vector_shr(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Shr<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x >> rhs.x, vector.y >> rhs.y)
    }

    #[inline]
    fn vector_bitand(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: BitAnd<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x & rhs.x, vector.y & rhs.y)
    }

    #[inline]
    fn vector_bitor(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: BitOr<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x | rhs.x, vector.y | rhs.y)
    }

    #[inline]
    fn vector_bitxor(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: BitXor<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x ^ rhs.x, vector.y ^ rhs.y)
    }

    #[inline]
    fn vector_element_sum(vector: Vector<2, Self, A>) -> Self
    where
        Self: Add<Output = Self>,
    {
        vector.x + vector.y
    }

    #[inline]
    fn vector_element_product(vector: Vector<2, Self, A>) -> Self
    where
        Self: Mul<Output = Self>,
    {
        vector.x * vector.y
    }

    #[inline]
    fn vector_eq_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
    where
        Self: PartialEq,
    {
        Mask::<2, Self, A>::new(vector.x == other.x, vector.y == other.y)
    }

    #[inline]
    fn vector_ne_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
    where
        Self: PartialEq,
    {
        Mask::<2, Self, A>::new(vector.x != other.x, vector.y != other.y)
    }

    #[inline]
    fn vector_lt_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<2, Self, A>::new(vector.x < other.x, vector.y < other.y)
    }

    #[inline]
    fn vector_gt_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<2, Self, A>::new(vector.x > other.x, vector.y > other.y)
    }

    #[inline]
    fn vector_le_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<2, Self, A>::new(vector.x <= other.x, vector.y <= other.y)
    }

    #[inline]
    fn vector_ge_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<2, Self, A>::new(vector.x >= other.x, vector.y >= other.y)
    }
}

// SAFETY: `Inner` follows its requirements.
unsafe impl<T, A: Alignment> VectorBackend<3, A> for T
where
    T: DefaultBackend<3, A>,
{
    type Inner = Repr3<T>;

    #[inline]
    fn vector_eq(vector: &Vector<3, Self, A>, other: &Vector<3, Self, A>) -> bool
    where
        Self: PartialEq,
    {
        vector.x == other.x && vector.y == other.y && vector.z == other.z
    }

    #[inline]
    fn vector_ne(vector: &Vector<3, Self, A>, other: &Vector<3, Self, A>) -> bool
    where
        Self: PartialEq,
    {
        !(vector == other)
    }

    #[inline]
    fn vector_neg(vector: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Neg<Output = Self>,
    {
        Vector::<3, Self, A>::new(-vector.x, -vector.y, -vector.z)
    }

    #[inline]
    fn vector_not(vector: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Not<Output = Self>,
    {
        Vector::<3, Self, A>::new(!vector.x, !vector.y, !vector.z)
    }

    #[inline]
    fn vector_add(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Add<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x + rhs.x, vector.y + rhs.y, vector.z + rhs.z)
    }

    #[inline]
    fn vector_sub(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Sub<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x - rhs.x, vector.y - rhs.y, vector.z - rhs.z)
    }

    #[inline]
    fn vector_mul(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Mul<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x * rhs.x, vector.y * rhs.y, vector.z * rhs.z)
    }

    #[inline]
    fn vector_div(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Div<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x / rhs.x, vector.y / rhs.y, vector.z / rhs.z)
    }

    #[inline]
    fn vector_rem(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Rem<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x % rhs.x, vector.y % rhs.y, vector.z % rhs.z)
    }

    #[inline]
    fn vector_shl(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Shl<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x << rhs.x, vector.y << rhs.y, vector.z << rhs.z)
    }

    #[inline]
    fn vector_shr(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Shr<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x >> rhs.x, vector.y >> rhs.y, vector.z >> rhs.z)
    }

    #[inline]
    fn vector_bitand(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: BitAnd<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x & rhs.x, vector.y & rhs.y, vector.z & rhs.z)
    }

    #[inline]
    fn vector_bitor(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: BitOr<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x | rhs.x, vector.y | rhs.y, vector.z | rhs.z)
    }

    #[inline]
    fn vector_bitxor(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: BitXor<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x ^ rhs.x, vector.y ^ rhs.y, vector.z ^ rhs.z)
    }

    #[inline]
    fn vector_element_sum(vector: Vector<3, Self, A>) -> Self
    where
        Self: Add<Output = Self>,
    {
        vector.x + vector.y + vector.z
    }

    #[inline]
    fn vector_element_product(vector: Vector<3, Self, A>) -> Self
    where
        Self: Mul<Output = Self>,
    {
        vector.x * vector.y * vector.z
    }

    #[inline]
    fn vector_eq_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
    where
        Self: PartialEq,
    {
        Mask::<3, Self, A>::new(
            vector.x == other.x,
            vector.y == other.y,
            vector.z == other.z,
        )
    }

    #[inline]
    fn vector_ne_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
    where
        Self: PartialEq,
    {
        Mask::<3, Self, A>::new(
            vector.x != other.x,
            vector.y != other.y,
            vector.z != other.z,
        )
    }

    #[inline]
    fn vector_lt_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<3, Self, A>::new(vector.x < other.x, vector.y < other.y, vector.z < other.z)
    }

    #[inline]
    fn vector_gt_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<3, Self, A>::new(vector.x > other.x, vector.y > other.y, vector.z > other.z)
    }

    #[inline]
    fn vector_le_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<3, Self, A>::new(
            vector.x <= other.x,
            vector.y <= other.y,
            vector.z <= other.z,
        )
    }

    #[inline]
    fn vector_ge_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<3, Self, A>::new(
            vector.x >= other.x,
            vector.y >= other.y,
            vector.z >= other.z,
        )
    }
}

// SAFETY: `Inner` follows its requirements.
unsafe impl<T, A: Alignment> VectorBackend<4, A> for T
where
    T: DefaultBackend<4, A>,
{
    type Inner = Repr4<T>;

    #[inline]
    fn vector_eq(vector: &Vector<4, Self, A>, other: &Vector<4, Self, A>) -> bool
    where
        Self: PartialEq,
    {
        vector.x == other.x && vector.y == other.y && vector.z == other.z && vector.w == other.w
    }

    #[inline]
    fn vector_ne(vector: &Vector<4, Self, A>, other: &Vector<4, Self, A>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        !(vector == other)
    }

    #[inline]
    fn vector_neg(vector: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: Neg<Output = Self>,
    {
        Vector::<4, Self, A>::new(-vector.x, -vector.y, -vector.z, -vector.w)
    }

    #[inline]
    fn vector_not(vector: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: Not<Output = Self>,
    {
        Vector::<4, Self, A>::new(!vector.x, !vector.y, !vector.z, !vector.w)
    }

    #[inline]
    fn vector_add(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: Add<Output = Self>,
    {
        Vector::<4, Self, A>::new(
            vector.x + rhs.x,
            vector.y + rhs.y,
            vector.z + rhs.z,
            vector.w + rhs.w,
        )
    }

    #[inline]
    fn vector_sub(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: Sub<Output = Self>,
    {
        Vector::<4, Self, A>::new(
            vector.x - rhs.x,
            vector.y - rhs.y,
            vector.z - rhs.z,
            vector.w - rhs.w,
        )
    }

    #[inline]
    fn vector_mul(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: Mul<Output = Self>,
    {
        Vector::<4, Self, A>::new(
            vector.x * rhs.x,
            vector.y * rhs.y,
            vector.z * rhs.z,
            vector.w * rhs.w,
        )
    }

    #[inline]
    fn vector_div(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: Div<Output = Self>,
    {
        Vector::<4, Self, A>::new(
            vector.x / rhs.x,
            vector.y / rhs.y,
            vector.z / rhs.z,
            vector.w / rhs.w,
        )
    }

    #[inline]
    fn vector_rem(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: Rem<Output = Self>,
    {
        Vector::<4, Self, A>::new(
            vector.x % rhs.x,
            vector.y % rhs.y,
            vector.z % rhs.z,
            vector.w % rhs.w,
        )
    }

    #[inline]
    fn vector_shl(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: Shl<Output = Self>,
    {
        Vector::<4, Self, A>::new(
            vector.x << rhs.x,
            vector.y << rhs.y,
            vector.z << rhs.z,
            vector.w << rhs.w,
        )
    }

    #[inline]
    fn vector_shr(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: Shr<Output = Self>,
    {
        Vector::<4, Self, A>::new(
            vector.x >> rhs.x,
            vector.y >> rhs.y,
            vector.z >> rhs.z,
            vector.w >> rhs.w,
        )
    }

    #[inline]
    fn vector_bitand(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: BitAnd<Output = Self>,
    {
        Vector::<4, Self, A>::new(
            vector.x & rhs.x,
            vector.y & rhs.y,
            vector.z & rhs.z,
            vector.w & rhs.w,
        )
    }

    #[inline]
    fn vector_bitor(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: BitOr<Output = Self>,
    {
        Vector::<4, Self, A>::new(
            vector.x | rhs.x,
            vector.y | rhs.y,
            vector.z | rhs.z,
            vector.w | rhs.w,
        )
    }

    #[inline]
    fn vector_bitxor(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: BitXor<Output = Self>,
    {
        Vector::<4, Self, A>::new(
            vector.x ^ rhs.x,
            vector.y ^ rhs.y,
            vector.z ^ rhs.z,
            vector.w ^ rhs.w,
        )
    }

    #[inline]
    fn vector_element_sum(vector: Vector<4, Self, A>) -> Self
    where
        Self: Add<Output = Self>,
    {
        vector.x + vector.y + (vector.z + vector.w)
    }

    #[inline]
    fn vector_element_product(vector: Vector<4, Self, A>) -> Self
    where
        Self: Mul<Output = Self>,
    {
        vector.x * vector.y * (vector.z * vector.w)
    }

    #[inline]
    fn vector_eq_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
    where
        Self: PartialEq,
    {
        Mask::<4, Self, A>::new(
            vector.x == other.x,
            vector.y == other.y,
            vector.z == other.z,
            vector.w == other.w,
        )
    }

    #[inline]
    fn vector_ne_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
    where
        Self: PartialEq,
    {
        Mask::<4, Self, A>::new(
            vector.x != other.x,
            vector.y != other.y,
            vector.z != other.z,
            vector.w != other.w,
        )
    }

    #[inline]
    fn vector_lt_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<4, Self, A>::new(
            vector.x < other.x,
            vector.y < other.y,
            vector.z < other.z,
            vector.w < other.w,
        )
    }

    #[inline]
    fn vector_gt_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<4, Self, A>::new(
            vector.x > other.x,
            vector.y > other.y,
            vector.z > other.z,
            vector.w > other.w,
        )
    }

    #[inline]
    fn vector_le_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<4, Self, A>::new(
            vector.x <= other.x,
            vector.y <= other.y,
            vector.z <= other.z,
            vector.w <= other.w,
        )
    }

    #[inline]
    fn vector_ge_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<4, Self, A>::new(
            vector.x >= other.x,
            vector.y >= other.y,
            vector.z >= other.z,
            vector.w >= other.w,
        )
    }
}

// SAFETY: The two first vectors are the matrix, and there is a third vector.
// There is no padding, so the pod requirement is met. `Matrix<2, _, _>` is
// represented by `Vector<4, T, A>`, and the trait bound here ensures its
// alignment is `T`'s alignment, thus our alignment is correct.
unsafe impl<T, A: Alignment> AffineBackend<2, A> for T
where
    T: Scalar + DefaultBackend<4, A>,
{
    type Inner = [Vector<2, T, A>; 3];
}

// SAFETY: The three first vectors are the matrix, and there is a fourth vector.
// There is no padding, so the pod requirement is met. `Matrix<3, _, _>` always
// has the alignment of `Vector<3, _, _>`, so our `Inner` has the matrix
// alignment.
unsafe impl<T, A: Alignment> AffineBackend<3, A> for T
where
    T: Scalar,
{
    type Inner = [Vector<3, T, A>; 4];
}

// SAFETY: The three four vectors are the matrix, and there is a fifth vector.
// There is no padding, so the pod requirement is met. `Matrix<4, _, _>` always
// has the alignment of `Vector<4, _, _>`, so our `Inner` has the matrix
// alignment.
unsafe impl<T, A: Alignment> AffineBackend<4, A> for T
where
    T: Scalar,
{
    type Inner = [Vector<4, T, A>; 5];
}

impl<T, A: Alignment> QuaternionBackend<A> for T
where
    T: DefaultBackend<4, A>,
{
    #[inline]
    fn quat_mul(quat: Quaternion<Self, A>, rhs: Quaternion<Self, A>) -> Quaternion<Self, A>
    where
        Self: Neg<Output = Self> + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self>,
    {
        let [x0, y0, z0, w0] = quat.to_array();
        let [x1, y1, z1, w1] = rhs.to_array();

        Quaternion::from_xyzw(
            x0 * w1 + w0 * x1 + z0 * y1 - y0 * z1,
            y0 * w1 - z0 * x1 + w0 * y1 + x0 * z1,
            z0 * w1 + y0 * x1 - x0 * y1 + w0 * z1,
            w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1,
        )
    }
}

// SAFETY: `Inner` follows its requirements.
unsafe impl<T, A: Alignment> MaskBackend<2, A> for T
where
    T: DefaultBackend<2, A>,
{
    type Inner = Repr2<bool>;

    #[inline]
    fn mask_from_array(array: [bool; 2]) -> Mask<2, Self, A> {
        Mask::from_inner(Repr2(array[0], array[1]))
    }

    #[inline]
    fn mask_splat(value: bool) -> Mask<2, Self, A> {
        Mask::from_inner(Repr2(value, value))
    }

    #[inline]
    fn mask_to_array(mask: Mask<2, Self, A>) -> [bool; 2] {
        [mask.inner().0, mask.inner().1]
    }

    #[inline]
    fn mask_all(mask: Mask<2, Self, A>) -> bool {
        mask.inner().0 && mask.inner().1
    }

    #[inline]
    fn mask_any(mask: Mask<2, Self, A>) -> bool {
        mask.inner().0 || mask.inner().1
    }

    #[inline]
    fn mask_select(
        mask: Mask<2, Self, A>,
        if_true: Vector<2, Self, A>,
        if_false: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(
            if mask.inner().0 {
                if_true.x
            } else {
                if_false.x
            },
            if mask.inner().1 {
                if_true.y
            } else {
                if_false.y
            },
        )
    }

    #[inline]
    fn mask_get(mask: Mask<2, Self, A>, index: usize) -> bool {
        match index {
            0 => mask.inner().0,
            1 => mask.inner().1,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_set(mask: &mut Mask<2, Self, A>, index: usize, value: bool) {
        match index {
            0 => mask.inner_mut().0 = value,
            1 => mask.inner_mut().1 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_eq(mask: &Mask<2, Self, A>, other: &Mask<2, Self, A>) -> bool {
        mask.inner() == other.inner()
    }

    #[inline]
    fn mask_ne(mask: &Mask<2, Self, A>, other: &Mask<2, Self, A>) -> bool
    where
        Self: Scalar,
    {
        !(mask == other)
    }

    #[inline]
    fn mask_not(mask: Mask<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(!mask.inner().0, !mask.inner().1)
    }

    #[inline]
    fn mask_bitand(mask: Mask<2, Self, A>, rhs: Mask<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(
            mask.inner().0 & rhs.inner().0,
            mask.inner().1 & rhs.inner().1,
        )
    }

    #[inline]
    fn mask_bitor(mask: Mask<2, Self, A>, rhs: Mask<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(
            mask.inner().0 | rhs.inner().0,
            mask.inner().1 | rhs.inner().1,
        )
    }

    #[inline]
    fn mask_bitxor(mask: Mask<2, Self, A>, rhs: Mask<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(
            mask.inner().0 ^ rhs.inner().0,
            mask.inner().1 ^ rhs.inner().1,
        )
    }
}

// SAFETY: `Inner` follows its requirements.
unsafe impl<T, A: Alignment> MaskBackend<3, A> for T
where
    T: DefaultBackend<3, A>,
{
    type Inner = Repr3<bool>;

    #[inline]
    fn mask_from_array(array: [bool; 3]) -> Mask<3, Self, A> {
        Mask::from_inner(Repr3(array[0], array[1], array[2]))
    }

    #[inline]
    fn mask_splat(value: bool) -> Mask<3, Self, A> {
        Mask::from_inner(Repr3(value, value, value))
    }

    #[inline]
    fn mask_to_array(mask: Mask<3, Self, A>) -> [bool; 3] {
        [mask.inner().0, mask.inner().1, mask.inner().2]
    }

    #[inline]
    fn mask_all(mask: Mask<3, Self, A>) -> bool {
        mask.inner().0 && mask.inner().1 && mask.inner().2
    }

    #[inline]
    fn mask_any(mask: Mask<3, Self, A>) -> bool {
        mask.inner().0 || mask.inner().1 || mask.inner().2
    }

    #[inline]
    fn mask_select(
        mask: Mask<3, Self, A>,
        if_true: Vector<3, Self, A>,
        if_false: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            if mask.inner().0 {
                if_true.x
            } else {
                if_false.x
            },
            if mask.inner().1 {
                if_true.y
            } else {
                if_false.y
            },
            if mask.inner().2 {
                if_true.z
            } else {
                if_false.z
            },
        )
    }

    #[inline]
    fn mask_get(mask: Mask<3, Self, A>, index: usize) -> bool {
        match index {
            0 => mask.inner().0,
            1 => mask.inner().1,
            2 => mask.inner().2,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_set(mask: &mut Mask<3, Self, A>, index: usize, value: bool) {
        match index {
            0 => mask.inner_mut().0 = value,
            1 => mask.inner_mut().1 = value,
            2 => mask.inner_mut().2 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_eq(mask: &Mask<3, Self, A>, other: &Mask<3, Self, A>) -> bool {
        mask.inner() == other.inner()
    }

    #[inline]
    fn mask_ne(mask: &Mask<3, Self, A>, other: &Mask<3, Self, A>) -> bool
    where
        Self: Scalar,
    {
        !(mask == other)
    }

    #[inline]
    fn mask_not(mask: Mask<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(!mask.inner().0, !mask.inner().1, !mask.inner().2)
    }

    #[inline]
    fn mask_bitand(mask: Mask<3, Self, A>, rhs: Mask<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            mask.inner().0 & rhs.inner().0,
            mask.inner().1 & rhs.inner().1,
            mask.inner().2 & rhs.inner().2,
        )
    }

    #[inline]
    fn mask_bitor(mask: Mask<3, Self, A>, rhs: Mask<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            mask.inner().0 | rhs.inner().0,
            mask.inner().1 | rhs.inner().1,
            mask.inner().2 | rhs.inner().2,
        )
    }

    #[inline]
    fn mask_bitxor(mask: Mask<3, Self, A>, rhs: Mask<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            mask.inner().0 ^ rhs.inner().0,
            mask.inner().1 ^ rhs.inner().1,
            mask.inner().2 ^ rhs.inner().2,
        )
    }
}

// SAFETY: `Inner` follows its requirements.
unsafe impl<T, A: Alignment> MaskBackend<4, A> for T
where
    T: DefaultBackend<4, A>,
{
    type Inner = Repr4<bool>;

    #[inline]
    fn mask_from_array(array: [bool; 4]) -> Mask<4, Self, A> {
        Mask::from_inner(Repr4(array[0], array[1], array[2], array[3]))
    }

    #[inline]
    fn mask_splat(value: bool) -> Mask<4, Self, A> {
        Mask::from_inner(Repr4(value, value, value, value))
    }

    #[inline]
    fn mask_to_array(mask: Mask<4, Self, A>) -> [bool; 4] {
        [
            mask.inner().0,
            mask.inner().1,
            mask.inner().2,
            mask.inner().3,
        ]
    }

    #[inline]
    fn mask_all(mask: Mask<4, Self, A>) -> bool {
        mask.inner().0 && mask.inner().1 && mask.inner().2 && mask.inner().3
    }

    #[inline]
    fn mask_any(mask: Mask<4, Self, A>) -> bool {
        mask.inner().0 || mask.inner().1 || mask.inner().2 || mask.inner().3
    }

    #[inline]
    fn mask_select(
        mask: Mask<4, Self, A>,
        if_true: Vector<4, Self, A>,
        if_false: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            if mask.inner().0 {
                if_true.x
            } else {
                if_false.x
            },
            if mask.inner().1 {
                if_true.y
            } else {
                if_false.y
            },
            if mask.inner().2 {
                if_true.z
            } else {
                if_false.z
            },
            if mask.inner().3 {
                if_true.w
            } else {
                if_false.w
            },
        )
    }

    #[inline]
    fn mask_get(mask: Mask<4, Self, A>, index: usize) -> bool {
        match index {
            0 => mask.inner().0,
            1 => mask.inner().1,
            2 => mask.inner().2,
            3 => mask.inner().3,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_set(mask: &mut Mask<4, Self, A>, index: usize, value: bool) {
        match index {
            0 => mask.inner_mut().0 = value,
            1 => mask.inner_mut().1 = value,
            2 => mask.inner_mut().2 = value,
            3 => mask.inner_mut().3 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_eq(mask: &Mask<4, Self, A>, other: &Mask<4, Self, A>) -> bool {
        mask.inner() == other.inner()
    }

    #[inline]
    fn mask_ne(mask: &Mask<4, Self, A>, other: &Mask<4, Self, A>) -> bool
    where
        Self: Scalar,
    {
        !(mask == other)
    }

    #[inline]
    fn mask_not(mask: Mask<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            !mask.inner().0,
            !mask.inner().1,
            !mask.inner().2,
            !mask.inner().3,
        )
    }

    #[inline]
    fn mask_bitand(mask: Mask<4, Self, A>, rhs: Mask<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            mask.inner().0 & rhs.inner().0,
            mask.inner().1 & rhs.inner().1,
            mask.inner().2 & rhs.inner().2,
            mask.inner().3 & rhs.inner().3,
        )
    }

    #[inline]
    fn mask_bitor(mask: Mask<4, Self, A>, rhs: Mask<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            mask.inner().0 | rhs.inner().0,
            mask.inner().1 | rhs.inner().1,
            mask.inner().2 | rhs.inner().2,
            mask.inner().3 | rhs.inner().3,
        )
    }

    #[inline]
    fn mask_bitxor(mask: Mask<4, Self, A>, rhs: Mask<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            mask.inner().0 ^ rhs.inner().0,
            mask.inner().1 ^ rhs.inner().1,
            mask.inner().2 ^ rhs.inner().2,
            mask.inner().3 ^ rhs.inner().3,
        )
    }
}

impl<T, A: Alignment> FloatVectorBackend<2, A> for T
where
    T: PrimitiveFloat + DefaultBackend<2, A>,
{
    #[inline]
    fn vector_nan_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_nan(), vector.y.is_nan())
    }

    #[inline]
    fn vector_finite_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_finite(), vector.y.is_finite())
    }

    #[inline]
    fn vector_sign_positive_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_sign_positive(), vector.y.is_sign_positive())
    }

    #[inline]
    fn vector_sign_negative_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_sign_negative(), vector.y.is_sign_negative())
    }

    #[inline]
    fn vector_max(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(
            if vector.x > other.x {
                vector.x
            } else {
                other.x
            },
            if vector.y > other.y {
                vector.y
            } else {
                other.y
            },
        )
    }

    #[inline]
    fn vector_min(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(
            if vector.x < other.x {
                vector.x
            } else {
                other.x
            },
            if vector.y < other.y {
                vector.y
            } else {
                other.y
            },
        )
    }

    #[inline]
    fn vector_max_element(vector: Vector<2, Self, A>) -> Self {
        if vector.x > vector.y {
            vector.x
        } else {
            vector.y
        }
    }

    #[inline]
    fn vector_min_element(vector: Vector<2, Self, A>) -> Self {
        if vector.x < vector.y {
            vector.x
        } else {
            vector.y
        }
    }

    #[inline]
    fn vector_abs(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.abs(), vector.y.abs())
    }

    #[inline]
    fn vector_signum(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.signum(), vector.y.signum())
    }

    #[inline]
    fn vector_copysign(vector: Vector<2, Self, A>, sign: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.copysign(sign.x), vector.y.copysign(sign.y))
    }

    #[inline]
    fn vector_floor(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.floor(), vector.y.floor())
    }

    #[inline]
    fn vector_ceil(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.ceil(), vector.y.ceil())
    }

    #[inline]
    fn vector_round(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.round(), vector.y.round())
    }

    #[inline]
    fn vector_trunc(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.trunc(), vector.y.trunc())
    }

    #[inline]
    fn vector_mul_add(
        vector: Vector<2, Self, A>,
        a: Vector<2, Self, A>,
        b: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.mul_add(a.x, b.x), vector.y.mul_add(a.y, b.y))
    }

    #[inline]
    fn vector_div_euclid(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.div_euclid(rhs.x), vector.y.div_euclid(rhs.y))
    }

    #[inline]
    fn vector_rem_euclid(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.rem_euclid(rhs.x), vector.y.rem_euclid(rhs.y))
    }

    #[inline]
    fn vector_powf(vector: Vector<2, Self, A>, n: Self) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.powf(n), vector.y.powf(n))
    }

    #[inline]
    fn vector_sqrt(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.sqrt(), vector.y.sqrt())
    }

    #[inline]
    fn vector_exp(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.exp(), vector.y.exp())
    }

    #[inline]
    fn vector_exp2(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.exp2(), vector.y.exp2())
    }

    #[inline]
    fn vector_ln(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.ln(), vector.y.ln())
    }

    #[inline]
    fn vector_log2(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.log2(), vector.y.log2())
    }

    #[inline]
    fn vector_sin(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.sin(), vector.y.sin())
    }

    #[inline]
    fn vector_cos(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.cos(), vector.y.cos())
    }

    #[inline]
    fn vector_tan(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.tan(), vector.y.tan())
    }

    #[inline]
    fn vector_asin(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.asin(), vector.y.asin())
    }

    #[inline]
    fn vector_acos(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.acos(), vector.y.acos())
    }

    #[inline]
    fn vector_atan(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.atan(), vector.y.atan())
    }

    #[inline]
    fn vector_sin_cos(vector: Vector<2, Self, A>) -> (Vector<2, Self, A>, Vector<2, Self, A>) {
        let x_sin_cos = vector.x.sin_cos();
        let y_sin_cos = vector.y.sin_cos();

        (
            Vector::<2, Self, A>::new(x_sin_cos.0, y_sin_cos.0),
            Vector::<2, Self, A>::new(x_sin_cos.1, y_sin_cos.1),
        )
    }
}

impl<T, A: Alignment> FloatVectorBackend<3, A> for T
where
    T: PrimitiveFloat + DefaultBackend<3, A>,
{
    #[inline]
    fn vector_nan_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(vector.x.is_nan(), vector.y.is_nan(), vector.z.is_nan())
    }

    #[inline]
    fn vector_finite_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            vector.x.is_finite(),
            vector.y.is_finite(),
            vector.z.is_finite(),
        )
    }

    #[inline]
    fn vector_sign_positive_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            vector.x.is_sign_positive(),
            vector.y.is_sign_positive(),
            vector.z.is_sign_positive(),
        )
    }

    #[inline]
    fn vector_sign_negative_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            vector.x.is_sign_negative(),
            vector.y.is_sign_negative(),
            vector.z.is_sign_negative(),
        )
    }

    #[inline]
    fn vector_max(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            if vector.x > other.x {
                vector.x
            } else {
                other.x
            },
            if vector.y > other.y {
                vector.y
            } else {
                other.y
            },
            if vector.z > other.z {
                vector.z
            } else {
                other.z
            },
        )
    }

    #[inline]
    fn vector_min(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            if vector.x < other.x {
                vector.x
            } else {
                other.x
            },
            if vector.y < other.y {
                vector.y
            } else {
                other.y
            },
            if vector.z < other.z {
                vector.z
            } else {
                other.z
            },
        )
    }

    #[inline]
    fn vector_max_element(vector: Vector<3, Self, A>) -> Self {
        let mut result = vector.x;
        if vector.y > result {
            result = vector.y;
        }
        if vector.z > result {
            result = vector.z;
        }
        result
    }

    #[inline]
    fn vector_min_element(vector: Vector<3, Self, A>) -> Self {
        let mut result = vector.x;
        if vector.y < result {
            result = vector.y;
        }
        if vector.z < result {
            result = vector.z;
        }
        result
    }

    #[inline]
    fn vector_abs(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.abs(), vector.y.abs(), vector.z.abs())
    }

    #[inline]
    fn vector_signum(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.signum(), vector.y.signum(), vector.z.signum())
    }

    #[inline]
    fn vector_copysign(vector: Vector<3, Self, A>, sign: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.copysign(sign.x),
            vector.y.copysign(sign.y),
            vector.z.copysign(sign.z),
        )
    }

    #[inline]
    fn vector_floor(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.floor(), vector.y.floor(), vector.z.floor())
    }

    #[inline]
    fn vector_ceil(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.ceil(), vector.y.ceil(), vector.z.ceil())
    }

    #[inline]
    fn vector_round(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.round(), vector.y.round(), vector.z.round())
    }

    #[inline]
    fn vector_trunc(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.trunc(), vector.y.trunc(), vector.z.trunc())
    }

    #[inline]
    fn vector_mul_add(
        vector: Vector<3, Self, A>,
        a: Vector<3, Self, A>,
        b: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.mul_add(a.x, b.x),
            vector.y.mul_add(a.y, b.y),
            vector.z.mul_add(a.z, b.z),
        )
    }

    #[inline]
    fn vector_div_euclid(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.div_euclid(rhs.x),
            vector.y.div_euclid(rhs.y),
            vector.z.div_euclid(rhs.z),
        )
    }

    #[inline]
    fn vector_rem_euclid(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.rem_euclid(rhs.x),
            vector.y.rem_euclid(rhs.y),
            vector.z.rem_euclid(rhs.z),
        )
    }

    #[inline]
    fn vector_powf(vector: Vector<3, Self, A>, n: Self) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.powf(n), vector.y.powf(n), vector.z.powf(n))
    }

    #[inline]
    fn vector_sqrt(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.sqrt(), vector.y.sqrt(), vector.z.sqrt())
    }

    #[inline]
    fn vector_exp(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.exp(), vector.y.exp(), vector.z.exp())
    }

    #[inline]
    fn vector_exp2(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.exp2(), vector.y.exp2(), vector.z.exp2())
    }

    #[inline]
    fn vector_ln(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.ln(), vector.y.ln(), vector.z.ln())
    }

    #[inline]
    fn vector_log2(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.log2(), vector.y.log2(), vector.z.log2())
    }

    #[inline]
    fn vector_sin(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.sin(), vector.y.sin(), vector.z.sin())
    }

    #[inline]
    fn vector_cos(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.cos(), vector.y.cos(), vector.z.cos())
    }

    #[inline]
    fn vector_tan(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.tan(), vector.y.tan(), vector.z.tan())
    }

    #[inline]
    fn vector_asin(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.asin(), vector.y.asin(), vector.z.asin())
    }

    #[inline]
    fn vector_acos(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.acos(), vector.y.acos(), vector.z.acos())
    }

    #[inline]
    fn vector_atan(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.atan(), vector.y.atan(), vector.z.atan())
    }

    #[inline]
    fn vector_sin_cos(vector: Vector<3, Self, A>) -> (Vector<3, Self, A>, Vector<3, Self, A>) {
        let x_sin_cos = vector.x.sin_cos();
        let y_sin_cos = vector.y.sin_cos();
        let z_sin_cos = vector.z.sin_cos();

        (
            Vector::<3, Self, A>::new(x_sin_cos.0, y_sin_cos.0, z_sin_cos.0),
            Vector::<3, Self, A>::new(x_sin_cos.1, y_sin_cos.1, z_sin_cos.1),
        )
    }
}

impl<T, A: Alignment> FloatVectorBackend<4, A> for T
where
    T: PrimitiveFloat + DefaultBackend<4, A>,
{
    #[inline]
    fn vector_nan_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_nan(),
            vector.y.is_nan(),
            vector.z.is_nan(),
            vector.w.is_nan(),
        )
    }

    #[inline]
    fn vector_finite_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_finite(),
            vector.y.is_finite(),
            vector.z.is_finite(),
            vector.w.is_finite(),
        )
    }

    #[inline]
    fn vector_sign_positive_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_sign_positive(),
            vector.y.is_sign_positive(),
            vector.z.is_sign_positive(),
            vector.w.is_sign_positive(),
        )
    }

    #[inline]
    fn vector_sign_negative_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_sign_negative(),
            vector.y.is_sign_negative(),
            vector.z.is_sign_negative(),
            vector.w.is_sign_negative(),
        )
    }

    #[inline]
    fn vector_max(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            if vector.x > other.x {
                vector.x
            } else {
                other.x
            },
            if vector.y > other.y {
                vector.y
            } else {
                other.y
            },
            if vector.z > other.z {
                vector.z
            } else {
                other.z
            },
            if vector.w > other.w {
                vector.w
            } else {
                other.w
            },
        )
    }

    #[inline]
    fn vector_min(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            if vector.x < other.x {
                vector.x
            } else {
                other.x
            },
            if vector.y < other.y {
                vector.y
            } else {
                other.y
            },
            if vector.z < other.z {
                vector.z
            } else {
                other.z
            },
            if vector.w < other.w {
                vector.w
            } else {
                other.w
            },
        )
    }

    #[inline]
    fn vector_max_element(vector: Vector<4, Self, A>) -> Self {
        let mut result = vector.x;
        if vector.y > result {
            result = vector.y;
        }
        if vector.z > result {
            result = vector.z;
        }
        if vector.w > result {
            result = vector.w;
        }
        result
    }

    #[inline]
    fn vector_min_element(vector: Vector<4, Self, A>) -> Self {
        let mut result = vector.x;
        if vector.y < result {
            result = vector.y;
        }
        if vector.z < result {
            result = vector.z;
        }
        if vector.w < result {
            result = vector.w;
        }
        result
    }

    #[inline]
    fn vector_abs(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.abs(),
            vector.y.abs(),
            vector.z.abs(),
            vector.w.abs(),
        )
    }

    #[inline]
    fn vector_signum(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.signum(),
            vector.y.signum(),
            vector.z.signum(),
            vector.w.signum(),
        )
    }

    #[inline]
    fn vector_copysign(vector: Vector<4, Self, A>, sign: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.copysign(sign.x),
            vector.y.copysign(sign.y),
            vector.z.copysign(sign.z),
            vector.w.copysign(sign.w),
        )
    }

    #[inline]
    fn vector_floor(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.floor(),
            vector.y.floor(),
            vector.z.floor(),
            vector.w.floor(),
        )
    }

    #[inline]
    fn vector_ceil(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.ceil(),
            vector.y.ceil(),
            vector.z.ceil(),
            vector.w.ceil(),
        )
    }

    #[inline]
    fn vector_round(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.round(),
            vector.y.round(),
            vector.z.round(),
            vector.w.round(),
        )
    }

    #[inline]
    fn vector_trunc(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.trunc(),
            vector.y.trunc(),
            vector.z.trunc(),
            vector.w.trunc(),
        )
    }

    #[inline]
    fn vector_mul_add(
        vector: Vector<4, Self, A>,
        a: Vector<4, Self, A>,
        b: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.mul_add(a.x, b.x),
            vector.y.mul_add(a.y, b.y),
            vector.z.mul_add(a.z, b.z),
            vector.w.mul_add(a.w, b.w),
        )
    }

    #[inline]
    fn vector_div_euclid(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.div_euclid(rhs.x),
            vector.y.div_euclid(rhs.y),
            vector.z.div_euclid(rhs.z),
            vector.w.div_euclid(rhs.w),
        )
    }

    #[inline]
    fn vector_rem_euclid(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.rem_euclid(rhs.x),
            vector.y.rem_euclid(rhs.y),
            vector.z.rem_euclid(rhs.z),
            vector.w.rem_euclid(rhs.w),
        )
    }

    #[inline]
    fn vector_powf(vector: Vector<4, Self, A>, n: Self) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.powf(n),
            vector.y.powf(n),
            vector.z.powf(n),
            vector.w.powf(n),
        )
    }

    #[inline]
    fn vector_sqrt(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.sqrt(),
            vector.y.sqrt(),
            vector.z.sqrt(),
            vector.w.sqrt(),
        )
    }

    #[inline]
    fn vector_exp(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.exp(),
            vector.y.exp(),
            vector.z.exp(),
            vector.w.exp(),
        )
    }

    #[inline]
    fn vector_exp2(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.exp2(),
            vector.y.exp2(),
            vector.z.exp2(),
            vector.w.exp2(),
        )
    }

    #[inline]
    fn vector_ln(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(vector.x.ln(), vector.y.ln(), vector.z.ln(), vector.w.ln())
    }

    #[inline]
    fn vector_log2(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.log2(),
            vector.y.log2(),
            vector.z.log2(),
            vector.w.log2(),
        )
    }

    #[inline]
    fn vector_sin(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.sin(),
            vector.y.sin(),
            vector.z.sin(),
            vector.w.sin(),
        )
    }

    #[inline]
    fn vector_cos(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.cos(),
            vector.y.cos(),
            vector.z.cos(),
            vector.w.cos(),
        )
    }

    #[inline]
    fn vector_tan(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.tan(),
            vector.y.tan(),
            vector.z.tan(),
            vector.w.tan(),
        )
    }

    #[inline]
    fn vector_asin(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.asin(),
            vector.y.asin(),
            vector.z.asin(),
            vector.w.asin(),
        )
    }

    #[inline]
    fn vector_acos(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.acos(),
            vector.y.acos(),
            vector.z.acos(),
            vector.w.acos(),
        )
    }

    #[inline]
    fn vector_atan(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.atan(),
            vector.y.atan(),
            vector.z.atan(),
            vector.w.atan(),
        )
    }

    #[inline]
    fn vector_sin_cos(vector: Vector<4, Self, A>) -> (Vector<4, Self, A>, Vector<4, Self, A>) {
        let x_sin_cos = vector.x.sin_cos();
        let y_sin_cos = vector.y.sin_cos();
        let z_sin_cos = vector.z.sin_cos();
        let w_sin_cos = vector.w.sin_cos();

        (
            Vector::<4, Self, A>::new(x_sin_cos.0, y_sin_cos.0, z_sin_cos.0, w_sin_cos.0),
            Vector::<4, Self, A>::new(x_sin_cos.1, y_sin_cos.1, z_sin_cos.1, w_sin_cos.1),
        )
    }
}

impl<T, A: Alignment> IntegerVectorBackend<2, A> for T
where
    T: PrimitiveInteger + DefaultBackend<2, A>,
{
    #[inline]
    fn vector_max(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.max(other.x), vector.y.max(other.y))
    }

    #[inline]
    fn vector_min(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.min(other.x), vector.y.min(other.y))
    }

    #[inline]
    fn vector_max_element(vector: Vector<2, Self, A>) -> Self {
        vector.x.max(vector.y)
    }

    #[inline]
    fn vector_min_element(vector: Vector<2, Self, A>) -> Self {
        vector.x.min(vector.y)
    }

    #[inline]
    fn vector_checked_add(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Option<Vector<2, Self, A>> {
        Some(Vector::<2, Self, A>::new(
            vector.x.checked_add(rhs.x)?,
            vector.y.checked_add(rhs.y)?,
        ))
    }

    #[inline]
    fn vector_checked_sub(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Option<Vector<2, Self, A>> {
        Some(Vector::<2, Self, A>::new(
            vector.x.checked_sub(rhs.x)?,
            vector.y.checked_sub(rhs.y)?,
        ))
    }

    #[inline]
    fn vector_checked_mul(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Option<Vector<2, Self, A>> {
        Some(Vector::<2, Self, A>::new(
            vector.x.checked_mul(rhs.x)?,
            vector.y.checked_mul(rhs.y)?,
        ))
    }

    #[inline]
    fn vector_checked_div(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Option<Vector<2, Self, A>> {
        Some(Vector::<2, Self, A>::new(
            vector.x.checked_div(rhs.x)?,
            vector.y.checked_div(rhs.y)?,
        ))
    }

    #[inline]
    fn vector_checked_rem(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Option<Vector<2, Self, A>> {
        Some(Vector::<2, Self, A>::new(
            vector.x.checked_rem(rhs.x)?,
            vector.y.checked_rem(rhs.y)?,
        ))
    }

    #[inline]
    fn vector_saturating_add(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(
            vector.x.saturating_add(rhs.x),
            vector.y.saturating_add(rhs.y),
        )
    }

    #[inline]
    fn vector_saturating_sub(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(
            vector.x.saturating_sub(rhs.x),
            vector.y.saturating_sub(rhs.y),
        )
    }

    #[inline]
    fn vector_saturating_mul(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(
            vector.x.saturating_mul(rhs.x),
            vector.y.saturating_mul(rhs.y),
        )
    }

    #[inline]
    fn vector_saturating_div(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(
            vector.x.saturating_div(rhs.x),
            vector.y.saturating_div(rhs.y),
        )
    }

    #[inline]
    fn vector_wrapping_add(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.wrapping_add(rhs.x), vector.y.wrapping_add(rhs.y))
    }

    #[inline]
    fn vector_wrapping_sub(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.wrapping_sub(rhs.x), vector.y.wrapping_sub(rhs.y))
    }

    #[inline]
    fn vector_wrapping_mul(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.wrapping_mul(rhs.x), vector.y.wrapping_mul(rhs.y))
    }

    #[inline]
    fn vector_wrapping_div(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.wrapping_div(rhs.x), vector.y.wrapping_div(rhs.y))
    }

    #[inline]
    fn vector_wrapping_rem(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.wrapping_rem(rhs.x), vector.y.wrapping_rem(rhs.y))
    }
}

impl<T, A: Alignment> IntegerVectorBackend<3, A> for T
where
    T: PrimitiveInteger + DefaultBackend<3, A>,
{
    #[inline]
    fn vector_max(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.max(other.x),
            vector.y.max(other.y),
            vector.z.max(other.z),
        )
    }

    #[inline]
    fn vector_min(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.min(other.x),
            vector.y.min(other.y),
            vector.z.min(other.z),
        )
    }

    #[inline]
    fn vector_max_element(vector: Vector<3, Self, A>) -> Self {
        vector.x.max(vector.y).max(vector.z)
    }

    #[inline]
    fn vector_min_element(vector: Vector<3, Self, A>) -> Self {
        vector.x.min(vector.y).min(vector.z)
    }

    #[inline]
    fn vector_checked_add(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Option<Vector<3, Self, A>> {
        Some(Vector::<3, Self, A>::new(
            vector.x.checked_add(rhs.x)?,
            vector.y.checked_add(rhs.y)?,
            vector.z.checked_add(rhs.z)?,
        ))
    }

    #[inline]
    fn vector_checked_sub(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Option<Vector<3, Self, A>> {
        Some(Vector::<3, Self, A>::new(
            vector.x.checked_sub(rhs.x)?,
            vector.y.checked_sub(rhs.y)?,
            vector.z.checked_sub(rhs.z)?,
        ))
    }

    #[inline]
    fn vector_checked_mul(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Option<Vector<3, Self, A>> {
        Some(Vector::<3, Self, A>::new(
            vector.x.checked_mul(rhs.x)?,
            vector.y.checked_mul(rhs.y)?,
            vector.z.checked_mul(rhs.z)?,
        ))
    }

    #[inline]
    fn vector_checked_div(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Option<Vector<3, Self, A>> {
        Some(Vector::<3, Self, A>::new(
            vector.x.checked_div(rhs.x)?,
            vector.y.checked_div(rhs.y)?,
            vector.z.checked_div(rhs.z)?,
        ))
    }

    #[inline]
    fn vector_checked_rem(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Option<Vector<3, Self, A>> {
        Some(Vector::<3, Self, A>::new(
            vector.x.checked_rem(rhs.x)?,
            vector.y.checked_rem(rhs.y)?,
            vector.z.checked_rem(rhs.z)?,
        ))
    }

    #[inline]
    fn vector_saturating_add(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.saturating_add(rhs.x),
            vector.y.saturating_add(rhs.y),
            vector.z.saturating_add(rhs.z),
        )
    }

    #[inline]
    fn vector_saturating_sub(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.saturating_sub(rhs.x),
            vector.y.saturating_sub(rhs.y),
            vector.z.saturating_sub(rhs.z),
        )
    }

    #[inline]
    fn vector_saturating_mul(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.saturating_mul(rhs.x),
            vector.y.saturating_mul(rhs.y),
            vector.z.saturating_mul(rhs.z),
        )
    }

    #[inline]
    fn vector_saturating_div(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.saturating_div(rhs.x),
            vector.y.saturating_div(rhs.y),
            vector.z.saturating_div(rhs.z),
        )
    }

    #[inline]
    fn vector_wrapping_add(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.wrapping_add(rhs.x),
            vector.y.wrapping_add(rhs.y),
            vector.z.wrapping_add(rhs.z),
        )
    }

    #[inline]
    fn vector_wrapping_sub(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.wrapping_sub(rhs.x),
            vector.y.wrapping_sub(rhs.y),
            vector.z.wrapping_sub(rhs.z),
        )
    }

    #[inline]
    fn vector_wrapping_mul(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.wrapping_mul(rhs.x),
            vector.y.wrapping_mul(rhs.y),
            vector.z.wrapping_mul(rhs.z),
        )
    }

    #[inline]
    fn vector_wrapping_div(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.wrapping_div(rhs.x),
            vector.y.wrapping_div(rhs.y),
            vector.z.wrapping_div(rhs.z),
        )
    }

    #[inline]
    fn vector_wrapping_rem(
        vector: Vector<3, Self, A>,
        rhs: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.wrapping_rem(rhs.x),
            vector.y.wrapping_rem(rhs.y),
            vector.z.wrapping_rem(rhs.z),
        )
    }
}

impl<T, A: Alignment> IntegerVectorBackend<4, A> for T
where
    T: PrimitiveInteger + DefaultBackend<4, A>,
{
    #[inline]
    fn vector_max(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.max(other.x),
            vector.y.max(other.y),
            vector.z.max(other.z),
            vector.w.max(other.w),
        )
    }

    #[inline]
    fn vector_min(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.min(other.x),
            vector.y.min(other.y),
            vector.z.min(other.z),
            vector.w.min(other.w),
        )
    }

    #[inline]
    fn vector_max_element(vector: Vector<4, Self, A>) -> Self {
        vector.x.max(vector.y).max(vector.z.max(vector.w))
    }

    #[inline]
    fn vector_min_element(vector: Vector<4, Self, A>) -> Self {
        vector.x.min(vector.y).min(vector.z.min(vector.w))
    }

    #[inline]
    fn vector_checked_add(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Option<Vector<4, Self, A>> {
        Some(Vector::<4, Self, A>::new(
            vector.x.checked_add(rhs.x)?,
            vector.y.checked_add(rhs.y)?,
            vector.z.checked_add(rhs.z)?,
            vector.w.checked_add(rhs.w)?,
        ))
    }

    #[inline]
    fn vector_checked_sub(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Option<Vector<4, Self, A>> {
        Some(Vector::<4, Self, A>::new(
            vector.x.checked_sub(rhs.x)?,
            vector.y.checked_sub(rhs.y)?,
            vector.z.checked_sub(rhs.z)?,
            vector.w.checked_sub(rhs.w)?,
        ))
    }

    #[inline]
    fn vector_checked_mul(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Option<Vector<4, Self, A>> {
        Some(Vector::<4, Self, A>::new(
            vector.x.checked_mul(rhs.x)?,
            vector.y.checked_mul(rhs.y)?,
            vector.z.checked_mul(rhs.z)?,
            vector.w.checked_mul(rhs.w)?,
        ))
    }

    #[inline]
    fn vector_checked_div(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Option<Vector<4, Self, A>> {
        Some(Vector::<4, Self, A>::new(
            vector.x.checked_div(rhs.x)?,
            vector.y.checked_div(rhs.y)?,
            vector.z.checked_div(rhs.z)?,
            vector.w.checked_div(rhs.w)?,
        ))
    }

    #[inline]
    fn vector_checked_rem(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Option<Vector<4, Self, A>> {
        Some(Vector::<4, Self, A>::new(
            vector.x.checked_rem(rhs.x)?,
            vector.y.checked_rem(rhs.y)?,
            vector.z.checked_rem(rhs.z)?,
            vector.w.checked_rem(rhs.w)?,
        ))
    }

    #[inline]
    fn vector_saturating_add(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.saturating_add(rhs.x),
            vector.y.saturating_add(rhs.y),
            vector.z.saturating_add(rhs.z),
            vector.w.saturating_add(rhs.w),
        )
    }

    #[inline]
    fn vector_saturating_sub(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.saturating_sub(rhs.x),
            vector.y.saturating_sub(rhs.y),
            vector.z.saturating_sub(rhs.z),
            vector.w.saturating_sub(rhs.w),
        )
    }

    #[inline]
    fn vector_saturating_mul(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.saturating_mul(rhs.x),
            vector.y.saturating_mul(rhs.y),
            vector.z.saturating_mul(rhs.z),
            vector.w.saturating_mul(rhs.w),
        )
    }

    #[inline]
    fn vector_saturating_div(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.saturating_div(rhs.x),
            vector.y.saturating_div(rhs.y),
            vector.z.saturating_div(rhs.z),
            vector.w.saturating_div(rhs.w),
        )
    }

    #[inline]
    fn vector_wrapping_add(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.wrapping_add(rhs.x),
            vector.y.wrapping_add(rhs.y),
            vector.z.wrapping_add(rhs.z),
            vector.w.wrapping_add(rhs.w),
        )
    }

    #[inline]
    fn vector_wrapping_sub(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.wrapping_sub(rhs.x),
            vector.y.wrapping_sub(rhs.y),
            vector.z.wrapping_sub(rhs.z),
            vector.w.wrapping_sub(rhs.w),
        )
    }

    #[inline]
    fn vector_wrapping_mul(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.wrapping_mul(rhs.x),
            vector.y.wrapping_mul(rhs.y),
            vector.z.wrapping_mul(rhs.z),
            vector.w.wrapping_mul(rhs.w),
        )
    }

    #[inline]
    fn vector_wrapping_div(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.wrapping_div(rhs.x),
            vector.y.wrapping_div(rhs.y),
            vector.z.wrapping_div(rhs.z),
            vector.w.wrapping_div(rhs.w),
        )
    }

    #[inline]
    fn vector_wrapping_rem(
        vector: Vector<4, Self, A>,
        rhs: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.wrapping_rem(rhs.x),
            vector.y.wrapping_rem(rhs.y),
            vector.z.wrapping_rem(rhs.z),
            vector.w.wrapping_rem(rhs.w),
        )
    }
}

impl<T, A: Alignment> SignedVectorBackend<2, A> for T
where
    T: PrimitiveSigned + DefaultBackend<2, A>,
{
    #[inline]
    fn vector_wrapping_abs(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, T, A>::new(vector.x.wrapping_abs(), vector.y.wrapping_abs())
    }

    #[inline]
    fn vector_signum(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, T, A>::new(vector.x.signum(), vector.y.signum())
    }

    #[inline]
    fn vector_positive_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_positive(), vector.y.is_positive())
    }

    #[inline]
    fn vector_negative_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_negative(), vector.y.is_negative())
    }
}

impl<T, A: Alignment> SignedVectorBackend<3, A> for T
where
    T: PrimitiveSigned + DefaultBackend<3, A>,
{
    #[inline]
    fn vector_wrapping_abs(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, T, A>::new(
            vector.x.wrapping_abs(),
            vector.y.wrapping_abs(),
            vector.z.wrapping_abs(),
        )
    }

    #[inline]
    fn vector_signum(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, T, A>::new(vector.x.signum(), vector.y.signum(), vector.z.signum())
    }

    #[inline]
    fn vector_positive_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            vector.x.is_positive(),
            vector.y.is_positive(),
            vector.z.is_positive(),
        )
    }

    #[inline]
    fn vector_negative_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            vector.x.is_negative(),
            vector.y.is_negative(),
            vector.z.is_negative(),
        )
    }
}

impl<T, A: Alignment> SignedVectorBackend<4, A> for T
where
    T: PrimitiveSigned + DefaultBackend<4, A>,
{
    #[inline]
    fn vector_wrapping_abs(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, T, A>::new(
            vector.x.wrapping_abs(),
            vector.y.wrapping_abs(),
            vector.z.wrapping_abs(),
            vector.w.wrapping_abs(),
        )
    }

    #[inline]
    fn vector_signum(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, T, A>::new(
            vector.x.signum(),
            vector.y.signum(),
            vector.z.signum(),
            vector.w.signum(),
        )
    }

    #[inline]
    fn vector_positive_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_positive(),
            vector.y.is_positive(),
            vector.z.is_positive(),
            vector.w.is_positive(),
        )
    }

    #[inline]
    fn vector_negative_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_negative(),
            vector.y.is_negative(),
            vector.z.is_negative(),
            vector.w.is_negative(),
        )
    }
}
