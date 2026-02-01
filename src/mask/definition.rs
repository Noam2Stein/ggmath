use core::{
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{Aligned, Alignment, Length, Scalar, SupportedLength, Unaligned, Vector};

/// A generic vector mask.
///
/// `Mask` is the generic form of:
///
/// - [`Mask2<T>`](crate::Mask2)
/// - [`Mask3<T>`](crate::Mask3)
/// - [`Mask4<T>`](crate::Mask4)
/// - [`Mask2U<T>`](crate::Mask2U)
/// - [`Mask3U<T>`](crate::Mask3U)
/// - [`Mask4U<T>`](crate::Mask4U)
///
/// `Mask` is generic over:
///
/// - `N`: Length (2, 3, or 4)
/// - `T`: Scalar type (see [`Scalar`])
/// - `A`: Alignment (see [`Alignment`])
///
/// To initialize masks, use the functions [`Mask2::new`](crate::Mask2::new),
/// [`Mask3::new`](crate::Mask3::new), [`Mask4::new`](crate::Mask4::new). To
/// initialize a mask of an unknown length, use [`Mask::from_array`].
///
/// # Guarantees
///
/// Mask types are currently missing all SIMD optimizations and have a temporary
/// internal representation of `[bool; N]`. This will be fixed before the next
/// release of the crate.
///
/// The exact guarantees of masks are not decided upon yet.
pub struct Mask<const N: usize, T, A: Alignment>(Vector<N, bool, A>, PhantomData<T>)
where
    Length<N>: SupportedLength,
    T: Scalar;

impl<const N: usize, T, A: Alignment> Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// Creates a mask from an array.
    ///
    /// The preferable way to create masks is using the functions
    /// [`Mask2::new`](crate::Mask2::new), [`Mask3::new`](crate::Mask3::new),
    /// [`Mask4::new`](crate::Mask4::new).
    ///
    /// `Mask::from_array` should only be used when the length of the mask is
    /// unknown or when directly converting from an array.
    #[inline]
    #[must_use]
    pub fn from_array(array: [bool; N]) -> Self {
        Self(Vector::from_array(array), PhantomData)
    }

    /// Creates a mask with all components set to the given value.
    #[inline]
    #[must_use]
    pub fn splat(value: bool) -> Self {
        Self(Vector::splat(value), PhantomData)
    }

    /// Creates a mask by calling function `f` for each component index.
    ///
    /// Equivalent to `(f(0), f(1), f(2), ...)`.
    #[inline]
    #[must_use]
    pub fn from_fn(f: impl FnMut(usize) -> bool) -> Self {
        Self(Vector::from_fn(f), PhantomData)
    }

    /// Converts the mask to the specified alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub fn to_alignment<A2: Alignment>(self) -> Mask<N, T, A2> {
        Mask(self.0.to_alignment(), PhantomData)
    }

    /// Converts the mask to [`Aligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub fn align(self) -> Mask<N, T, Aligned> {
        self.to_alignment()
    }

    /// Converts the mask to [`Unaligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub fn unalign(self) -> Mask<N, T, Unaligned> {
        self.to_alignment()
    }

    /// Converts the mask to an array.
    #[inline]
    #[must_use]
    pub fn to_array(self) -> [bool; N] {
        self.0.to_array()
    }

    /// Returns `true` if all of the mask's components are `true`.
    #[inline]
    #[must_use]
    pub fn all(self) -> bool {
        self.0.all()
    }

    /// Returns `true` if any of the mask's components are `true`.
    #[inline]
    #[must_use]
    pub fn any(self) -> bool {
        self.0.any()
    }

    /// Selects between the components of `if_true` and `if_false` based on the
    /// values of the mask.
    #[inline]
    #[must_use]
    pub fn select(self, if_true: Vector<N, T, A>, if_false: Vector<N, T, A>) -> Vector<N, T, A> {
        self.0.select(if_true, if_false)
    }

    /// Returns an iterator over the mask's components.
    #[inline]
    #[must_use]
    pub fn iter(self) -> core::array::IntoIter<bool, N> {
        self.0.iter()
    }

    /// Returns the component at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get(self, index: usize) -> bool {
        self.0[index]
    }

    /// Sets the component at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        self.0[index] = value;
    }
}

impl<T, A: Alignment> Mask<2, T, A>
where
    T: Scalar,
{
    /// Creates a 2-component mask.
    #[inline]
    #[must_use]
    pub fn new(x: bool, y: bool) -> Self {
        Self(Vector::<2, bool, A>::new(x, y), PhantomData)
    }
}

impl<T, A: Alignment> Mask<3, T, A>
where
    T: Scalar,
{
    /// Creates a 3-component mask.
    #[inline]
    #[must_use]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        Self(Vector::<3, bool, A>::new(x, y, z), PhantomData)
    }
}

impl<T, A: Alignment> Mask<4, T, A>
where
    T: Scalar,
{
    /// Creates a 4-component mask.
    #[inline]
    #[must_use]
    pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self(Vector::<4, bool, A>::new(x, y, z, w), PhantomData)
    }
}

impl<const N: usize, T, A: Alignment> Clone for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> IntoIterator for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Item = bool;
    type IntoIter = core::array::IntoIter<bool, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const N: usize, T, A: Alignment> IntoIterator for &Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Item = bool;
    type IntoIter = core::array::IntoIter<bool, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const N: usize, T, A: Alignment> Debug for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "({}, {})", self.0[0], self.0[1]),
            3 => write!(f, "({}, {}, {})", self.0[0], self.0[1], self.0[2]),
            4 => write!(
                f,
                "({}, {}, {}, {})",
                self.0[0], self.0[1], self.0[2], self.0[3]
            ),
            _ => unreachable!(),
        }
    }
}

impl<const N: usize, T, A: Alignment> Display for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "({}, {})", self.0[0], self.0[1]),
            3 => write!(f, "({}, {}, {})", self.0[0], self.0[1], self.0[2]),
            4 => write!(
                f,
                "({}, {}, {}, {})",
                self.0[0], self.0[1], self.0[2], self.0[3]
            ),
            _ => unreachable!(),
        }
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    #[expect(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl<const N: usize, T, A: Alignment> Eq for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> Hash for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<const N: usize, T, A: Alignment> Default for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

impl<const N: usize, T, A: Alignment> Not for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self(!self.0, PhantomData)
    }
}

macro_rules! impl_binary_op {
    ($Op:ident $op:ident) => {
        impl<const N: usize, T, A: Alignment> $Op for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: Self) -> Self::Output {
                Self(self.0.$op(rhs.0), PhantomData)
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<bool> for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: bool) -> Self::Output {
                self.$op(Self::splat(rhs))
            }
        }
    };
}
impl_binary_op!(BitAnd bitand);
impl_binary_op!(BitOr bitor);
impl_binary_op!(BitXor bitxor);

macro_rules! impl_assign_op {
    ($OpAssign:ident $op_assign:ident $op:ident) => {
        impl<const N: usize, T, A: Alignment> $OpAssign for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            #[inline]
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl<const N: usize, T, A: Alignment> $OpAssign<bool> for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            #[inline]
            fn $op_assign(&mut self, rhs: bool) {
                *self = self.$op(rhs);
            }
        }
    };
}
impl_assign_op!(BitAndAssign bitand_assign bitand);
impl_assign_op!(BitOrAssign bitor_assign bitor);
impl_assign_op!(BitXorAssign bitxor_assign bitxor);

unsafe impl<const N: usize, T, A: Alignment> Send for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

unsafe impl<const N: usize, T, A: Alignment> Sync for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}
