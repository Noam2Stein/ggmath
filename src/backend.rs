use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

use crate::{
    Aligned, Alignment, Dim, Element, Mask, PrimitiveFloat, PrimitiveInteger, PrimitiveSigned,
    Rotor, TwoThreeOrFour, Unaligned, Vector,
    dim::Three,
    utils::{Repr2, Repr3, Repr4},
};

cfg_select! {
    any(
        target_feature = "sse2",
        all(target_arch = "aarch64", target_feature = "neon"),
    ) => {
        mod f32x4;
    }
    _ => {
        impl DefaultBackend<3, Aligned> for f32 {}

        impl DefaultBackend<4, Aligned> for f32 {}
    }
}

pub(crate) trait DefaultBackend<const N: usize, A: Alignment>: Element {}

/// # Safety
///
/// [`Self::Inner`] must be implemented correctly. All other items are safe to
/// implement.
#[diagnostic::on_unimplemented(
    message = "`ggmath::Element` cannot be implemented directly",
    note = "see the documentation for `ggmath::Element`"
)]
pub(crate) unsafe trait VectorBackend<const N: usize, A: Alignment>
where
    Dim<N>: TwoThreeOrFour,
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

    fn eq(vector: &Vector<N, Self, A>, other: &Vector<N, Self, A>) -> bool
    where
        Self: Element + PartialEq;

    fn ne(vector: &Vector<N, Self, A>, other: &Vector<N, Self, A>) -> bool
    where
        Self: Element + PartialEq;

    #[track_caller]
    fn neg(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + Neg<Output = Self>;

    #[track_caller]
    fn not(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + Not<Output = Self>;

    #[track_caller]
    fn add(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + Add<Output = Self>;

    #[track_caller]
    fn sub(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + Sub<Output = Self>;

    #[track_caller]
    fn mul(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + Mul<Output = Self>;

    #[track_caller]
    fn div(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + Div<Output = Self>;

    #[track_caller]
    fn rem(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + Rem<Output = Self>;

    #[track_caller]
    fn shl(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + Shl<Output = Self>;

    #[track_caller]
    fn shr(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + Shr<Output = Self>;

    #[track_caller]
    fn bitand(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + BitAnd<Output = Self>;

    #[track_caller]
    fn bitor(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + BitOr<Output = Self>;

    #[track_caller]
    fn bitxor(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Element + BitXor<Output = Self>;

    #[track_caller]
    fn element_sum(vector: Vector<N, Self, A>) -> Self
    where
        Self: Element + Add<Output = Self>;

    #[track_caller]
    fn element_product(vector: Vector<N, Self, A>) -> Self
    where
        Self: Element + Mul<Output = Self>;

    fn eq_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Element + PartialEq;

    fn ne_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Element + PartialEq;

    fn lt_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Element + PartialOrd;

    fn gt_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Element + PartialOrd;

    fn le_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Element + PartialOrd;

    fn ge_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Element + PartialOrd;
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

pub(crate) trait RotorBackend<const N: usize, A: Alignment>
where
    Dim<N>: Three,
{
    #[track_caller]
    fn conjugate(rotor: Rotor<N, Self, A>) -> Rotor<N, Self, A>
    where
        Self: Element + Neg<Output = Self>;

    #[track_caller]
    fn mul(rotor: Rotor<N, Self, A>, rhs: Rotor<N, Self, A>) -> Rotor<N, Self, A>
    where
        Self: Element
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
    message = "`ggmath::Element` cannot be implemented directly",
    note = "see the documentation for `ggmath::Element`"
)]
pub(crate) unsafe trait MaskBackend<const N: usize, A: Alignment>
where
    Dim<N>: TwoThreeOrFour,
{
    /// Controls the internal representation of [`Mask<N, Self, A>`].
    ///
    /// # Safety
    ///
    /// This type must only have initialized memory and must accept the zero
    /// bit-pattern.
    type Inner: Send + Sync + Copy;

    fn from_array(array: [bool; N]) -> Mask<N, Self, A>
    where
        Self: Element;

    fn splat(value: bool) -> Mask<N, Self, A>
    where
        Self: Element;

    fn to_array(mask: Mask<N, Self, A>) -> [bool; N]
    where
        Self: Element;

    fn all(mask: Mask<N, Self, A>) -> bool
    where
        Self: Element;

    fn any(mask: Mask<N, Self, A>) -> bool
    where
        Self: Element;

    fn select(
        mask: Mask<N, Self, A>,
        if_true: Vector<N, Self, A>,
        if_false: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Element;

    #[track_caller]
    fn get(mask: Mask<N, Self, A>, index: usize) -> bool
    where
        Self: Element;

    #[track_caller]
    fn set(mask: &mut Mask<N, Self, A>, index: usize, value: bool)
    where
        Self: Element;

    fn eq(mask: &Mask<N, Self, A>, other: &Mask<N, Self, A>) -> bool
    where
        Self: Element;

    fn ne(mask: &Mask<N, Self, A>, other: &Mask<N, Self, A>) -> bool
    where
        Self: Element;

    fn not(mask: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Element;

    fn bitand(mask: Mask<N, Self, A>, rhs: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Element;

    fn bitor(mask: Mask<N, Self, A>, rhs: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Element;

    fn bitxor(mask: Mask<N, Self, A>, rhs: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Element;
}

pub(crate) trait FloatVectorBackend<const N: usize, A: Alignment>: Element
where
    Dim<N>: TwoThreeOrFour,
{
    fn nan_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;

    fn finite_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;

    fn sign_positive_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;

    fn sign_negative_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;

    fn max(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn min(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn max_element(vector: Vector<N, Self, A>) -> Self;

    fn min_element(vector: Vector<N, Self, A>) -> Self;

    fn abs(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn signum(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn copysign(vector: Vector<N, Self, A>, sign: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn floor(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn ceil(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn round(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn trunc(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn mul_add(
        vector: Vector<N, Self, A>,
        a: Vector<N, Self, A>,
        b: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>;

    fn div_euclid(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn rem_euclid(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn powf(vector: Vector<N, Self, A>, n: Self) -> Vector<N, Self, A>;

    fn sqrt(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn exp(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn exp2(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn ln(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn log2(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn sin(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn cos(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn tan(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn asin(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn acos(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn atan(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn sin_cos(vector: Vector<N, Self, A>) -> (Vector<N, Self, A>, Vector<N, Self, A>);
}

pub(crate) trait IntegerVectorBackend<const N: usize, A: Alignment>: Element
where
    Dim<N>: TwoThreeOrFour,
{
    fn max(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn min(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn max_element(vector: Vector<N, Self, A>) -> Self;

    fn min_element(vector: Vector<N, Self, A>) -> Self;

    fn checked_add(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>;

    fn checked_sub(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>;

    fn checked_mul(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>;

    fn checked_div(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>;

    fn checked_rem(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>;

    fn saturating_add(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn saturating_sub(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn saturating_mul(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>;

    #[track_caller]
    fn saturating_div(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn wrapping_add(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn wrapping_sub(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn wrapping_mul(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>;

    #[track_caller]
    fn wrapping_div(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>;

    #[track_caller]
    fn wrapping_rem(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>;
}

pub(crate) trait SignedVectorBackend<const N: usize, A: Alignment>: Element
where
    Dim<N>: TwoThreeOrFour,
{
    fn wrapping_abs(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn signum(vector: Vector<N, Self, A>) -> Vector<N, Self, A>;

    fn positive_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;

    fn negative_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>;
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
    fn eq(vector: &Vector<2, Self, A>, other: &Vector<2, Self, A>) -> bool
    where
        Self: PartialEq,
    {
        vector.x == other.x && vector.y == other.y
    }

    #[inline]
    fn ne(vector: &Vector<2, Self, A>, other: &Vector<2, Self, A>) -> bool
    where
        Self: PartialEq,
    {
        !(vector == other)
    }

    #[inline]
    fn neg(vector: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Neg<Output = Self>,
    {
        Vector::<2, Self, A>::new(-vector.x, -vector.y)
    }

    #[inline]
    fn not(vector: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Not<Output = Self>,
    {
        Vector::<2, Self, A>::new(!vector.x, !vector.y)
    }

    #[inline]
    fn add(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Add<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x + rhs.x, vector.y + rhs.y)
    }

    #[inline]
    fn sub(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Sub<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x - rhs.x, vector.y - rhs.y)
    }

    #[inline]
    fn mul(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Mul<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x * rhs.x, vector.y * rhs.y)
    }

    #[inline]
    fn div(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Div<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x / rhs.x, vector.y / rhs.y)
    }

    #[inline]
    fn rem(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Rem<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x % rhs.x, vector.y % rhs.y)
    }

    #[inline]
    fn shl(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Shl<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x << rhs.x, vector.y << rhs.y)
    }

    #[inline]
    fn shr(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: Shr<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x >> rhs.x, vector.y >> rhs.y)
    }

    #[inline]
    fn bitand(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: BitAnd<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x & rhs.x, vector.y & rhs.y)
    }

    #[inline]
    fn bitor(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: BitOr<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x | rhs.x, vector.y | rhs.y)
    }

    #[inline]
    fn bitxor(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A>
    where
        Self: BitXor<Output = Self>,
    {
        Vector::<2, Self, A>::new(vector.x ^ rhs.x, vector.y ^ rhs.y)
    }

    #[inline]
    fn element_sum(vector: Vector<2, Self, A>) -> Self
    where
        Self: Add<Output = Self>,
    {
        vector.x + vector.y
    }

    #[inline]
    fn element_product(vector: Vector<2, Self, A>) -> Self
    where
        Self: Mul<Output = Self>,
    {
        vector.x * vector.y
    }

    #[inline]
    fn eq_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
    where
        Self: PartialEq,
    {
        Mask::<2, Self, A>::new(vector.x == other.x, vector.y == other.y)
    }

    #[inline]
    fn ne_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
    where
        Self: PartialEq,
    {
        Mask::<2, Self, A>::new(vector.x != other.x, vector.y != other.y)
    }

    #[inline]
    fn lt_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<2, Self, A>::new(vector.x < other.x, vector.y < other.y)
    }

    #[inline]
    fn gt_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<2, Self, A>::new(vector.x > other.x, vector.y > other.y)
    }

    #[inline]
    fn le_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<2, Self, A>::new(vector.x <= other.x, vector.y <= other.y)
    }

    #[inline]
    fn ge_mask(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Mask<2, Self, A>
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
    fn eq(vector: &Vector<3, Self, A>, other: &Vector<3, Self, A>) -> bool
    where
        Self: PartialEq,
    {
        vector.x == other.x && vector.y == other.y && vector.z == other.z
    }

    #[inline]
    fn ne(vector: &Vector<3, Self, A>, other: &Vector<3, Self, A>) -> bool
    where
        Self: PartialEq,
    {
        !(vector == other)
    }

    #[inline]
    fn neg(vector: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Neg<Output = Self>,
    {
        Vector::<3, Self, A>::new(-vector.x, -vector.y, -vector.z)
    }

    #[inline]
    fn not(vector: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Not<Output = Self>,
    {
        Vector::<3, Self, A>::new(!vector.x, !vector.y, !vector.z)
    }

    #[inline]
    fn add(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Add<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x + rhs.x, vector.y + rhs.y, vector.z + rhs.z)
    }

    #[inline]
    fn sub(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Sub<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x - rhs.x, vector.y - rhs.y, vector.z - rhs.z)
    }

    #[inline]
    fn mul(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Mul<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x * rhs.x, vector.y * rhs.y, vector.z * rhs.z)
    }

    #[inline]
    fn div(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Div<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x / rhs.x, vector.y / rhs.y, vector.z / rhs.z)
    }

    #[inline]
    fn rem(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Rem<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x % rhs.x, vector.y % rhs.y, vector.z % rhs.z)
    }

    #[inline]
    fn shl(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Shl<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x << rhs.x, vector.y << rhs.y, vector.z << rhs.z)
    }

    #[inline]
    fn shr(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: Shr<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x >> rhs.x, vector.y >> rhs.y, vector.z >> rhs.z)
    }

    #[inline]
    fn bitand(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: BitAnd<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x & rhs.x, vector.y & rhs.y, vector.z & rhs.z)
    }

    #[inline]
    fn bitor(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: BitOr<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x | rhs.x, vector.y | rhs.y, vector.z | rhs.z)
    }

    #[inline]
    fn bitxor(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A>
    where
        Self: BitXor<Output = Self>,
    {
        Vector::<3, Self, A>::new(vector.x ^ rhs.x, vector.y ^ rhs.y, vector.z ^ rhs.z)
    }

    #[inline]
    fn element_sum(vector: Vector<3, Self, A>) -> Self
    where
        Self: Add<Output = Self>,
    {
        vector.x + vector.y + vector.z
    }

    #[inline]
    fn element_product(vector: Vector<3, Self, A>) -> Self
    where
        Self: Mul<Output = Self>,
    {
        vector.x * vector.y * vector.z
    }

    #[inline]
    fn eq_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
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
    fn ne_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
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
    fn lt_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<3, Self, A>::new(vector.x < other.x, vector.y < other.y, vector.z < other.z)
    }

    #[inline]
    fn gt_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
    where
        Self: PartialOrd,
    {
        Mask::<3, Self, A>::new(vector.x > other.x, vector.y > other.y, vector.z > other.z)
    }

    #[inline]
    fn le_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
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
    fn ge_mask(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Mask<3, Self, A>
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
    fn eq(vector: &Vector<4, Self, A>, other: &Vector<4, Self, A>) -> bool
    where
        Self: PartialEq,
    {
        vector.x == other.x && vector.y == other.y && vector.z == other.z && vector.w == other.w
    }

    #[inline]
    fn ne(vector: &Vector<4, Self, A>, other: &Vector<4, Self, A>) -> bool
    where
        Self: Element + PartialEq,
    {
        !(vector == other)
    }

    #[inline]
    fn neg(vector: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: Neg<Output = Self>,
    {
        Vector::<4, Self, A>::new(-vector.x, -vector.y, -vector.z, -vector.w)
    }

    #[inline]
    fn not(vector: Vector<4, Self, A>) -> Vector<4, Self, A>
    where
        Self: Not<Output = Self>,
    {
        Vector::<4, Self, A>::new(!vector.x, !vector.y, !vector.z, !vector.w)
    }

    #[inline]
    fn add(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
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
    fn sub(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
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
    fn mul(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
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
    fn div(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
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
    fn rem(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
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
    fn shl(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
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
    fn shr(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
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
    fn bitand(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
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
    fn bitor(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
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
    fn bitxor(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A>
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
    fn element_sum(vector: Vector<4, Self, A>) -> Self
    where
        Self: Add<Output = Self>,
    {
        vector.x + vector.y + (vector.z + vector.w)
    }

    #[inline]
    fn element_product(vector: Vector<4, Self, A>) -> Self
    where
        Self: Mul<Output = Self>,
    {
        vector.x * vector.y * (vector.z * vector.w)
    }

    #[inline]
    fn eq_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
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
    fn ne_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
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
    fn lt_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
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
    fn gt_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
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
    fn le_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
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
    fn ge_mask(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Mask<4, Self, A>
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
    T: Element + DefaultBackend<4, A>,
{
    type Inner = [Vector<2, T, A>; 3];
}

// SAFETY: The three first vectors are the matrix, and there is a fourth vector.
// There is no padding, so the pod requirement is met. `Matrix<3, _, _>` always
// has the alignment of `Vector<3, _, _>`, so our `Inner` has the matrix
// alignment.
unsafe impl<T, A: Alignment> AffineBackend<3, A> for T
where
    T: Element,
{
    type Inner = [Vector<3, T, A>; 4];
}

// SAFETY: The three four vectors are the matrix, and there is a fifth vector.
// There is no padding, so the pod requirement is met. `Matrix<4, _, _>` always
// has the alignment of `Vector<4, _, _>`, so our `Inner` has the matrix
// alignment.
unsafe impl<T, A: Alignment> AffineBackend<4, A> for T
where
    T: Element,
{
    type Inner = [Vector<4, T, A>; 5];
}

impl<T, A: Alignment> RotorBackend<3, A> for T
where
    T: DefaultBackend<4, A>,
{
    #[inline]
    fn conjugate(rotor: Rotor<3, Self, A>) -> Rotor<3, Self, A>
    where
        Self: Element + Neg<Output = Self>,
    {
        Rotor::from_elements(-rotor.yz, -rotor.zx, -rotor.xy, rotor.s)
    }

    #[inline]
    fn mul(rotor: Rotor<3, Self, A>, rhs: Rotor<3, Self, A>) -> Rotor<3, Self, A>
    where
        Self: Neg<Output = Self> + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self>,
    {
        let [yz0, zx0, xy0, s0] = rotor.to_array();
        let [yz1, zx1, xy1, s1] = rhs.to_array();

        Rotor::<3, Self, A>::from_elements(
            yz0 * s1 + s0 * yz1 + xy0 * zx1 - zx0 * xy1,
            zx0 * s1 - xy0 * yz1 + s0 * zx1 + yz0 * xy1,
            xy0 * s1 + zx0 * yz1 - yz0 * zx1 + s0 * xy1,
            s0 * s1 - yz0 * yz1 - zx0 * zx1 - xy0 * xy1,
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
    fn from_array(array: [bool; 2]) -> Mask<2, Self, A> {
        Mask::from_inner(Repr2(array[0], array[1]))
    }

    #[inline]
    fn splat(value: bool) -> Mask<2, Self, A> {
        Mask::from_inner(Repr2(value, value))
    }

    #[inline]
    fn to_array(mask: Mask<2, Self, A>) -> [bool; 2] {
        [mask.inner().0, mask.inner().1]
    }

    #[inline]
    fn all(mask: Mask<2, Self, A>) -> bool {
        mask.inner().0 && mask.inner().1
    }

    #[inline]
    fn any(mask: Mask<2, Self, A>) -> bool {
        mask.inner().0 || mask.inner().1
    }

    #[inline]
    fn select(
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
    fn get(mask: Mask<2, Self, A>, index: usize) -> bool {
        match index {
            0 => mask.inner().0,
            1 => mask.inner().1,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn set(mask: &mut Mask<2, Self, A>, index: usize, value: bool) {
        match index {
            0 => mask.inner_mut().0 = value,
            1 => mask.inner_mut().1 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn eq(mask: &Mask<2, Self, A>, other: &Mask<2, Self, A>) -> bool {
        mask.inner() == other.inner()
    }

    #[inline]
    fn ne(mask: &Mask<2, Self, A>, other: &Mask<2, Self, A>) -> bool
    where
        Self: Element,
    {
        !(mask == other)
    }

    #[inline]
    fn not(mask: Mask<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(!mask.inner().0, !mask.inner().1)
    }

    #[inline]
    fn bitand(mask: Mask<2, Self, A>, rhs: Mask<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(
            mask.inner().0 & rhs.inner().0,
            mask.inner().1 & rhs.inner().1,
        )
    }

    #[inline]
    fn bitor(mask: Mask<2, Self, A>, rhs: Mask<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(
            mask.inner().0 | rhs.inner().0,
            mask.inner().1 | rhs.inner().1,
        )
    }

    #[inline]
    fn bitxor(mask: Mask<2, Self, A>, rhs: Mask<2, Self, A>) -> Mask<2, Self, A> {
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
    fn from_array(array: [bool; 3]) -> Mask<3, Self, A> {
        Mask::from_inner(Repr3(array[0], array[1], array[2]))
    }

    #[inline]
    fn splat(value: bool) -> Mask<3, Self, A> {
        Mask::from_inner(Repr3(value, value, value))
    }

    #[inline]
    fn to_array(mask: Mask<3, Self, A>) -> [bool; 3] {
        [mask.inner().0, mask.inner().1, mask.inner().2]
    }

    #[inline]
    fn all(mask: Mask<3, Self, A>) -> bool {
        mask.inner().0 && mask.inner().1 && mask.inner().2
    }

    #[inline]
    fn any(mask: Mask<3, Self, A>) -> bool {
        mask.inner().0 || mask.inner().1 || mask.inner().2
    }

    #[inline]
    fn select(
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
    fn get(mask: Mask<3, Self, A>, index: usize) -> bool {
        match index {
            0 => mask.inner().0,
            1 => mask.inner().1,
            2 => mask.inner().2,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn set(mask: &mut Mask<3, Self, A>, index: usize, value: bool) {
        match index {
            0 => mask.inner_mut().0 = value,
            1 => mask.inner_mut().1 = value,
            2 => mask.inner_mut().2 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn eq(mask: &Mask<3, Self, A>, other: &Mask<3, Self, A>) -> bool {
        mask.inner() == other.inner()
    }

    #[inline]
    fn ne(mask: &Mask<3, Self, A>, other: &Mask<3, Self, A>) -> bool
    where
        Self: Element,
    {
        !(mask == other)
    }

    #[inline]
    fn not(mask: Mask<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(!mask.inner().0, !mask.inner().1, !mask.inner().2)
    }

    #[inline]
    fn bitand(mask: Mask<3, Self, A>, rhs: Mask<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            mask.inner().0 & rhs.inner().0,
            mask.inner().1 & rhs.inner().1,
            mask.inner().2 & rhs.inner().2,
        )
    }

    #[inline]
    fn bitor(mask: Mask<3, Self, A>, rhs: Mask<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            mask.inner().0 | rhs.inner().0,
            mask.inner().1 | rhs.inner().1,
            mask.inner().2 | rhs.inner().2,
        )
    }

    #[inline]
    fn bitxor(mask: Mask<3, Self, A>, rhs: Mask<3, Self, A>) -> Mask<3, Self, A> {
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
    fn from_array(array: [bool; 4]) -> Mask<4, Self, A> {
        Mask::from_inner(Repr4(array[0], array[1], array[2], array[3]))
    }

    #[inline]
    fn splat(value: bool) -> Mask<4, Self, A> {
        Mask::from_inner(Repr4(value, value, value, value))
    }

    #[inline]
    fn to_array(mask: Mask<4, Self, A>) -> [bool; 4] {
        [
            mask.inner().0,
            mask.inner().1,
            mask.inner().2,
            mask.inner().3,
        ]
    }

    #[inline]
    fn all(mask: Mask<4, Self, A>) -> bool {
        mask.inner().0 && mask.inner().1 && mask.inner().2 && mask.inner().3
    }

    #[inline]
    fn any(mask: Mask<4, Self, A>) -> bool {
        mask.inner().0 || mask.inner().1 || mask.inner().2 || mask.inner().3
    }

    #[inline]
    fn select(
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
    fn get(mask: Mask<4, Self, A>, index: usize) -> bool {
        match index {
            0 => mask.inner().0,
            1 => mask.inner().1,
            2 => mask.inner().2,
            3 => mask.inner().3,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn set(mask: &mut Mask<4, Self, A>, index: usize, value: bool) {
        match index {
            0 => mask.inner_mut().0 = value,
            1 => mask.inner_mut().1 = value,
            2 => mask.inner_mut().2 = value,
            3 => mask.inner_mut().3 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn eq(mask: &Mask<4, Self, A>, other: &Mask<4, Self, A>) -> bool {
        mask.inner() == other.inner()
    }

    #[inline]
    fn ne(mask: &Mask<4, Self, A>, other: &Mask<4, Self, A>) -> bool
    where
        Self: Element,
    {
        !(mask == other)
    }

    #[inline]
    fn not(mask: Mask<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            !mask.inner().0,
            !mask.inner().1,
            !mask.inner().2,
            !mask.inner().3,
        )
    }

    #[inline]
    fn bitand(mask: Mask<4, Self, A>, rhs: Mask<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            mask.inner().0 & rhs.inner().0,
            mask.inner().1 & rhs.inner().1,
            mask.inner().2 & rhs.inner().2,
            mask.inner().3 & rhs.inner().3,
        )
    }

    #[inline]
    fn bitor(mask: Mask<4, Self, A>, rhs: Mask<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            mask.inner().0 | rhs.inner().0,
            mask.inner().1 | rhs.inner().1,
            mask.inner().2 | rhs.inner().2,
            mask.inner().3 | rhs.inner().3,
        )
    }

    #[inline]
    fn bitxor(mask: Mask<4, Self, A>, rhs: Mask<4, Self, A>) -> Mask<4, Self, A> {
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
    fn nan_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_nan(), vector.y.is_nan())
    }

    #[inline]
    fn finite_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_finite(), vector.y.is_finite())
    }

    #[inline]
    fn sign_positive_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_sign_positive(), vector.y.is_sign_positive())
    }

    #[inline]
    fn sign_negative_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_sign_negative(), vector.y.is_sign_negative())
    }

    #[inline]
    fn max(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Vector<2, Self, A> {
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
    fn min(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Vector<2, Self, A> {
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
    fn max_element(vector: Vector<2, Self, A>) -> Self {
        if vector.x > vector.y {
            vector.x
        } else {
            vector.y
        }
    }

    #[inline]
    fn min_element(vector: Vector<2, Self, A>) -> Self {
        if vector.x < vector.y {
            vector.x
        } else {
            vector.y
        }
    }

    #[inline]
    fn abs(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.abs(), vector.y.abs())
    }

    #[inline]
    fn signum(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.signum(), vector.y.signum())
    }

    #[inline]
    fn copysign(vector: Vector<2, Self, A>, sign: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.copysign(sign.x), vector.y.copysign(sign.y))
    }

    #[inline]
    fn floor(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.floor(), vector.y.floor())
    }

    #[inline]
    fn ceil(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.ceil(), vector.y.ceil())
    }

    #[inline]
    fn round(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.round(), vector.y.round())
    }

    #[inline]
    fn trunc(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.trunc(), vector.y.trunc())
    }

    #[inline]
    fn mul_add(
        vector: Vector<2, Self, A>,
        a: Vector<2, Self, A>,
        b: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.mul_add(a.x, b.x), vector.y.mul_add(a.y, b.y))
    }

    #[inline]
    fn div_euclid(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.div_euclid(rhs.x), vector.y.div_euclid(rhs.y))
    }

    #[inline]
    fn rem_euclid(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.rem_euclid(rhs.x), vector.y.rem_euclid(rhs.y))
    }

    #[inline]
    fn powf(vector: Vector<2, Self, A>, n: Self) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.powf(n), vector.y.powf(n))
    }

    #[inline]
    fn sqrt(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.sqrt(), vector.y.sqrt())
    }

    #[inline]
    fn exp(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.exp(), vector.y.exp())
    }

    #[inline]
    fn exp2(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.exp2(), vector.y.exp2())
    }

    #[inline]
    fn ln(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.ln(), vector.y.ln())
    }

    #[inline]
    fn log2(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.log2(), vector.y.log2())
    }

    #[inline]
    fn sin(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.sin(), vector.y.sin())
    }

    #[inline]
    fn cos(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.cos(), vector.y.cos())
    }

    #[inline]
    fn tan(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.tan(), vector.y.tan())
    }

    #[inline]
    fn asin(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.asin(), vector.y.asin())
    }

    #[inline]
    fn acos(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.acos(), vector.y.acos())
    }

    #[inline]
    fn atan(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.atan(), vector.y.atan())
    }

    #[inline]
    fn sin_cos(vector: Vector<2, Self, A>) -> (Vector<2, Self, A>, Vector<2, Self, A>) {
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
    fn nan_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(vector.x.is_nan(), vector.y.is_nan(), vector.z.is_nan())
    }

    #[inline]
    fn finite_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            vector.x.is_finite(),
            vector.y.is_finite(),
            vector.z.is_finite(),
        )
    }

    #[inline]
    fn sign_positive_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            vector.x.is_sign_positive(),
            vector.y.is_sign_positive(),
            vector.z.is_sign_positive(),
        )
    }

    #[inline]
    fn sign_negative_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            vector.x.is_sign_negative(),
            vector.y.is_sign_negative(),
            vector.z.is_sign_negative(),
        )
    }

    #[inline]
    fn max(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Vector<3, Self, A> {
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
    fn min(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Vector<3, Self, A> {
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
    fn max_element(vector: Vector<3, Self, A>) -> Self {
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
    fn min_element(vector: Vector<3, Self, A>) -> Self {
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
    fn abs(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.abs(), vector.y.abs(), vector.z.abs())
    }

    #[inline]
    fn signum(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.signum(), vector.y.signum(), vector.z.signum())
    }

    #[inline]
    fn copysign(vector: Vector<3, Self, A>, sign: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.copysign(sign.x),
            vector.y.copysign(sign.y),
            vector.z.copysign(sign.z),
        )
    }

    #[inline]
    fn floor(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.floor(), vector.y.floor(), vector.z.floor())
    }

    #[inline]
    fn ceil(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.ceil(), vector.y.ceil(), vector.z.ceil())
    }

    #[inline]
    fn round(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.round(), vector.y.round(), vector.z.round())
    }

    #[inline]
    fn trunc(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.trunc(), vector.y.trunc(), vector.z.trunc())
    }

    #[inline]
    fn mul_add(
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
    fn div_euclid(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.div_euclid(rhs.x),
            vector.y.div_euclid(rhs.y),
            vector.z.div_euclid(rhs.z),
        )
    }

    #[inline]
    fn rem_euclid(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.rem_euclid(rhs.x),
            vector.y.rem_euclid(rhs.y),
            vector.z.rem_euclid(rhs.z),
        )
    }

    #[inline]
    fn powf(vector: Vector<3, Self, A>, n: Self) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.powf(n), vector.y.powf(n), vector.z.powf(n))
    }

    #[inline]
    fn sqrt(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.sqrt(), vector.y.sqrt(), vector.z.sqrt())
    }

    #[inline]
    fn exp(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.exp(), vector.y.exp(), vector.z.exp())
    }

    #[inline]
    fn exp2(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.exp2(), vector.y.exp2(), vector.z.exp2())
    }

    #[inline]
    fn ln(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.ln(), vector.y.ln(), vector.z.ln())
    }

    #[inline]
    fn log2(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.log2(), vector.y.log2(), vector.z.log2())
    }

    #[inline]
    fn sin(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.sin(), vector.y.sin(), vector.z.sin())
    }

    #[inline]
    fn cos(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.cos(), vector.y.cos(), vector.z.cos())
    }

    #[inline]
    fn tan(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.tan(), vector.y.tan(), vector.z.tan())
    }

    #[inline]
    fn asin(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.asin(), vector.y.asin(), vector.z.asin())
    }

    #[inline]
    fn acos(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.acos(), vector.y.acos(), vector.z.acos())
    }

    #[inline]
    fn atan(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(vector.x.atan(), vector.y.atan(), vector.z.atan())
    }

    #[inline]
    fn sin_cos(vector: Vector<3, Self, A>) -> (Vector<3, Self, A>, Vector<3, Self, A>) {
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
    fn nan_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_nan(),
            vector.y.is_nan(),
            vector.z.is_nan(),
            vector.w.is_nan(),
        )
    }

    #[inline]
    fn finite_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_finite(),
            vector.y.is_finite(),
            vector.z.is_finite(),
            vector.w.is_finite(),
        )
    }

    #[inline]
    fn sign_positive_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_sign_positive(),
            vector.y.is_sign_positive(),
            vector.z.is_sign_positive(),
            vector.w.is_sign_positive(),
        )
    }

    #[inline]
    fn sign_negative_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_sign_negative(),
            vector.y.is_sign_negative(),
            vector.z.is_sign_negative(),
            vector.w.is_sign_negative(),
        )
    }

    #[inline]
    fn max(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Vector<4, Self, A> {
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
    fn min(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Vector<4, Self, A> {
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
    fn max_element(vector: Vector<4, Self, A>) -> Self {
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
    fn min_element(vector: Vector<4, Self, A>) -> Self {
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
    fn abs(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.abs(),
            vector.y.abs(),
            vector.z.abs(),
            vector.w.abs(),
        )
    }

    #[inline]
    fn signum(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.signum(),
            vector.y.signum(),
            vector.z.signum(),
            vector.w.signum(),
        )
    }

    #[inline]
    fn copysign(vector: Vector<4, Self, A>, sign: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.copysign(sign.x),
            vector.y.copysign(sign.y),
            vector.z.copysign(sign.z),
            vector.w.copysign(sign.w),
        )
    }

    #[inline]
    fn floor(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.floor(),
            vector.y.floor(),
            vector.z.floor(),
            vector.w.floor(),
        )
    }

    #[inline]
    fn ceil(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.ceil(),
            vector.y.ceil(),
            vector.z.ceil(),
            vector.w.ceil(),
        )
    }

    #[inline]
    fn round(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.round(),
            vector.y.round(),
            vector.z.round(),
            vector.w.round(),
        )
    }

    #[inline]
    fn trunc(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.trunc(),
            vector.y.trunc(),
            vector.z.trunc(),
            vector.w.trunc(),
        )
    }

    #[inline]
    fn mul_add(
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
    fn div_euclid(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.div_euclid(rhs.x),
            vector.y.div_euclid(rhs.y),
            vector.z.div_euclid(rhs.z),
            vector.w.div_euclid(rhs.w),
        )
    }

    #[inline]
    fn rem_euclid(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.rem_euclid(rhs.x),
            vector.y.rem_euclid(rhs.y),
            vector.z.rem_euclid(rhs.z),
            vector.w.rem_euclid(rhs.w),
        )
    }

    #[inline]
    fn powf(vector: Vector<4, Self, A>, n: Self) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.powf(n),
            vector.y.powf(n),
            vector.z.powf(n),
            vector.w.powf(n),
        )
    }

    #[inline]
    fn sqrt(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.sqrt(),
            vector.y.sqrt(),
            vector.z.sqrt(),
            vector.w.sqrt(),
        )
    }

    #[inline]
    fn exp(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.exp(),
            vector.y.exp(),
            vector.z.exp(),
            vector.w.exp(),
        )
    }

    #[inline]
    fn exp2(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.exp2(),
            vector.y.exp2(),
            vector.z.exp2(),
            vector.w.exp2(),
        )
    }

    #[inline]
    fn ln(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(vector.x.ln(), vector.y.ln(), vector.z.ln(), vector.w.ln())
    }

    #[inline]
    fn log2(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.log2(),
            vector.y.log2(),
            vector.z.log2(),
            vector.w.log2(),
        )
    }

    #[inline]
    fn sin(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.sin(),
            vector.y.sin(),
            vector.z.sin(),
            vector.w.sin(),
        )
    }

    #[inline]
    fn cos(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.cos(),
            vector.y.cos(),
            vector.z.cos(),
            vector.w.cos(),
        )
    }

    #[inline]
    fn tan(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.tan(),
            vector.y.tan(),
            vector.z.tan(),
            vector.w.tan(),
        )
    }

    #[inline]
    fn asin(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.asin(),
            vector.y.asin(),
            vector.z.asin(),
            vector.w.asin(),
        )
    }

    #[inline]
    fn acos(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.acos(),
            vector.y.acos(),
            vector.z.acos(),
            vector.w.acos(),
        )
    }

    #[inline]
    fn atan(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.atan(),
            vector.y.atan(),
            vector.z.atan(),
            vector.w.atan(),
        )
    }

    #[inline]
    fn sin_cos(vector: Vector<4, Self, A>) -> (Vector<4, Self, A>, Vector<4, Self, A>) {
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
    fn max(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.max(other.x), vector.y.max(other.y))
    }

    #[inline]
    fn min(vector: Vector<2, Self, A>, other: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.min(other.x), vector.y.min(other.y))
    }

    #[inline]
    fn max_element(vector: Vector<2, Self, A>) -> Self {
        vector.x.max(vector.y)
    }

    #[inline]
    fn min_element(vector: Vector<2, Self, A>) -> Self {
        vector.x.min(vector.y)
    }

    #[inline]
    fn checked_add(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Option<Vector<2, Self, A>> {
        Some(Vector::<2, Self, A>::new(
            vector.x.checked_add(rhs.x)?,
            vector.y.checked_add(rhs.y)?,
        ))
    }

    #[inline]
    fn checked_sub(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Option<Vector<2, Self, A>> {
        Some(Vector::<2, Self, A>::new(
            vector.x.checked_sub(rhs.x)?,
            vector.y.checked_sub(rhs.y)?,
        ))
    }

    #[inline]
    fn checked_mul(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Option<Vector<2, Self, A>> {
        Some(Vector::<2, Self, A>::new(
            vector.x.checked_mul(rhs.x)?,
            vector.y.checked_mul(rhs.y)?,
        ))
    }

    #[inline]
    fn checked_div(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Option<Vector<2, Self, A>> {
        Some(Vector::<2, Self, A>::new(
            vector.x.checked_div(rhs.x)?,
            vector.y.checked_div(rhs.y)?,
        ))
    }

    #[inline]
    fn checked_rem(
        vector: Vector<2, Self, A>,
        rhs: Vector<2, Self, A>,
    ) -> Option<Vector<2, Self, A>> {
        Some(Vector::<2, Self, A>::new(
            vector.x.checked_rem(rhs.x)?,
            vector.y.checked_rem(rhs.y)?,
        ))
    }

    #[inline]
    fn saturating_add(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(
            vector.x.saturating_add(rhs.x),
            vector.y.saturating_add(rhs.y),
        )
    }

    #[inline]
    fn saturating_sub(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(
            vector.x.saturating_sub(rhs.x),
            vector.y.saturating_sub(rhs.y),
        )
    }

    #[inline]
    fn saturating_mul(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(
            vector.x.saturating_mul(rhs.x),
            vector.y.saturating_mul(rhs.y),
        )
    }

    #[inline]
    fn saturating_div(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(
            vector.x.saturating_div(rhs.x),
            vector.y.saturating_div(rhs.y),
        )
    }

    #[inline]
    fn wrapping_add(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.wrapping_add(rhs.x), vector.y.wrapping_add(rhs.y))
    }

    #[inline]
    fn wrapping_sub(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.wrapping_sub(rhs.x), vector.y.wrapping_sub(rhs.y))
    }

    #[inline]
    fn wrapping_mul(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.wrapping_mul(rhs.x), vector.y.wrapping_mul(rhs.y))
    }

    #[inline]
    fn wrapping_div(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.wrapping_div(rhs.x), vector.y.wrapping_div(rhs.y))
    }

    #[inline]
    fn wrapping_rem(vector: Vector<2, Self, A>, rhs: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, Self, A>::new(vector.x.wrapping_rem(rhs.x), vector.y.wrapping_rem(rhs.y))
    }
}

impl<T, A: Alignment> IntegerVectorBackend<3, A> for T
where
    T: PrimitiveInteger + DefaultBackend<3, A>,
{
    #[inline]
    fn max(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.max(other.x),
            vector.y.max(other.y),
            vector.z.max(other.z),
        )
    }

    #[inline]
    fn min(vector: Vector<3, Self, A>, other: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.min(other.x),
            vector.y.min(other.y),
            vector.z.min(other.z),
        )
    }

    #[inline]
    fn max_element(vector: Vector<3, Self, A>) -> Self {
        vector.x.max(vector.y).max(vector.z)
    }

    #[inline]
    fn min_element(vector: Vector<3, Self, A>) -> Self {
        vector.x.min(vector.y).min(vector.z)
    }

    #[inline]
    fn checked_add(
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
    fn checked_sub(
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
    fn checked_mul(
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
    fn checked_div(
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
    fn checked_rem(
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
    fn saturating_add(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.saturating_add(rhs.x),
            vector.y.saturating_add(rhs.y),
            vector.z.saturating_add(rhs.z),
        )
    }

    #[inline]
    fn saturating_sub(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.saturating_sub(rhs.x),
            vector.y.saturating_sub(rhs.y),
            vector.z.saturating_sub(rhs.z),
        )
    }

    #[inline]
    fn saturating_mul(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.saturating_mul(rhs.x),
            vector.y.saturating_mul(rhs.y),
            vector.z.saturating_mul(rhs.z),
        )
    }

    #[inline]
    fn saturating_div(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.saturating_div(rhs.x),
            vector.y.saturating_div(rhs.y),
            vector.z.saturating_div(rhs.z),
        )
    }

    #[inline]
    fn wrapping_add(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.wrapping_add(rhs.x),
            vector.y.wrapping_add(rhs.y),
            vector.z.wrapping_add(rhs.z),
        )
    }

    #[inline]
    fn wrapping_sub(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.wrapping_sub(rhs.x),
            vector.y.wrapping_sub(rhs.y),
            vector.z.wrapping_sub(rhs.z),
        )
    }

    #[inline]
    fn wrapping_mul(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.wrapping_mul(rhs.x),
            vector.y.wrapping_mul(rhs.y),
            vector.z.wrapping_mul(rhs.z),
        )
    }

    #[inline]
    fn wrapping_div(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, Self, A>::new(
            vector.x.wrapping_div(rhs.x),
            vector.y.wrapping_div(rhs.y),
            vector.z.wrapping_div(rhs.z),
        )
    }

    #[inline]
    fn wrapping_rem(vector: Vector<3, Self, A>, rhs: Vector<3, Self, A>) -> Vector<3, Self, A> {
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
    fn max(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.max(other.x),
            vector.y.max(other.y),
            vector.z.max(other.z),
            vector.w.max(other.w),
        )
    }

    #[inline]
    fn min(vector: Vector<4, Self, A>, other: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.min(other.x),
            vector.y.min(other.y),
            vector.z.min(other.z),
            vector.w.min(other.w),
        )
    }

    #[inline]
    fn max_element(vector: Vector<4, Self, A>) -> Self {
        vector.x.max(vector.y).max(vector.z.max(vector.w))
    }

    #[inline]
    fn min_element(vector: Vector<4, Self, A>) -> Self {
        vector.x.min(vector.y).min(vector.z.min(vector.w))
    }

    #[inline]
    fn checked_add(
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
    fn checked_sub(
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
    fn checked_mul(
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
    fn checked_div(
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
    fn checked_rem(
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
    fn saturating_add(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.saturating_add(rhs.x),
            vector.y.saturating_add(rhs.y),
            vector.z.saturating_add(rhs.z),
            vector.w.saturating_add(rhs.w),
        )
    }

    #[inline]
    fn saturating_sub(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.saturating_sub(rhs.x),
            vector.y.saturating_sub(rhs.y),
            vector.z.saturating_sub(rhs.z),
            vector.w.saturating_sub(rhs.w),
        )
    }

    #[inline]
    fn saturating_mul(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.saturating_mul(rhs.x),
            vector.y.saturating_mul(rhs.y),
            vector.z.saturating_mul(rhs.z),
            vector.w.saturating_mul(rhs.w),
        )
    }

    #[inline]
    fn saturating_div(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.saturating_div(rhs.x),
            vector.y.saturating_div(rhs.y),
            vector.z.saturating_div(rhs.z),
            vector.w.saturating_div(rhs.w),
        )
    }

    #[inline]
    fn wrapping_add(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.wrapping_add(rhs.x),
            vector.y.wrapping_add(rhs.y),
            vector.z.wrapping_add(rhs.z),
            vector.w.wrapping_add(rhs.w),
        )
    }

    #[inline]
    fn wrapping_sub(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.wrapping_sub(rhs.x),
            vector.y.wrapping_sub(rhs.y),
            vector.z.wrapping_sub(rhs.z),
            vector.w.wrapping_sub(rhs.w),
        )
    }

    #[inline]
    fn wrapping_mul(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.wrapping_mul(rhs.x),
            vector.y.wrapping_mul(rhs.y),
            vector.z.wrapping_mul(rhs.z),
            vector.w.wrapping_mul(rhs.w),
        )
    }

    #[inline]
    fn wrapping_div(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, Self, A>::new(
            vector.x.wrapping_div(rhs.x),
            vector.y.wrapping_div(rhs.y),
            vector.z.wrapping_div(rhs.z),
            vector.w.wrapping_div(rhs.w),
        )
    }

    #[inline]
    fn wrapping_rem(vector: Vector<4, Self, A>, rhs: Vector<4, Self, A>) -> Vector<4, Self, A> {
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
    fn wrapping_abs(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, T, A>::new(vector.x.wrapping_abs(), vector.y.wrapping_abs())
    }

    #[inline]
    fn signum(vector: Vector<2, Self, A>) -> Vector<2, Self, A> {
        Vector::<2, T, A>::new(vector.x.signum(), vector.y.signum())
    }

    #[inline]
    fn positive_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_positive(), vector.y.is_positive())
    }

    #[inline]
    fn negative_mask(vector: Vector<2, Self, A>) -> Mask<2, Self, A> {
        Mask::<2, Self, A>::new(vector.x.is_negative(), vector.y.is_negative())
    }
}

impl<T, A: Alignment> SignedVectorBackend<3, A> for T
where
    T: PrimitiveSigned + DefaultBackend<3, A>,
{
    #[inline]
    fn wrapping_abs(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, T, A>::new(
            vector.x.wrapping_abs(),
            vector.y.wrapping_abs(),
            vector.z.wrapping_abs(),
        )
    }

    #[inline]
    fn signum(vector: Vector<3, Self, A>) -> Vector<3, Self, A> {
        Vector::<3, T, A>::new(vector.x.signum(), vector.y.signum(), vector.z.signum())
    }

    #[inline]
    fn positive_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
        Mask::<3, Self, A>::new(
            vector.x.is_positive(),
            vector.y.is_positive(),
            vector.z.is_positive(),
        )
    }

    #[inline]
    fn negative_mask(vector: Vector<3, Self, A>) -> Mask<3, Self, A> {
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
    fn wrapping_abs(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, T, A>::new(
            vector.x.wrapping_abs(),
            vector.y.wrapping_abs(),
            vector.z.wrapping_abs(),
            vector.w.wrapping_abs(),
        )
    }

    #[inline]
    fn signum(vector: Vector<4, Self, A>) -> Vector<4, Self, A> {
        Vector::<4, T, A>::new(
            vector.x.signum(),
            vector.y.signum(),
            vector.z.signum(),
            vector.w.signum(),
        )
    }

    #[inline]
    fn positive_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_positive(),
            vector.y.is_positive(),
            vector.z.is_positive(),
            vector.w.is_positive(),
        )
    }

    #[inline]
    fn negative_mask(vector: Vector<4, Self, A>) -> Mask<4, Self, A> {
        Mask::<4, Self, A>::new(
            vector.x.is_negative(),
            vector.y.is_negative(),
            vector.z.is_negative(),
            vector.w.is_negative(),
        )
    }
}
