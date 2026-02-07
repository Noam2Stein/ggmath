use core::{
    fmt::{Debug, Display},
    hash::Hash,
    mem::transmute,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, Scalar, ScalarRepr, SupportedLength, Unaligned, Vector,
    utils::specialize,
};

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
/// The exact representation of masks isn't stable, but they do guarantee
/// certain properties.
///
/// Masks are guaranteed to have no uninitialized bytes.
///
/// Masks are guaranteed to be zeroable (to accept the zero bit-pattern).
///
/// Masks of scalars with the same [`Scalar::Repr`] are guaranteed to have the
/// same size, same alignment, and to be transmutable to each other. This
/// includes scalars where `Repr = ()`.
///
/// Keep in mind that scalars that have the same `Repr` today might silently
/// change their `Repr` in the future.
pub struct Mask<const N: usize, T, A: Alignment>(
    pub(crate) <T::Repr as ScalarRepr>::MaskRepr<N, A>,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

/*
When the compiler is smart enough to understand type equality based on const
generic equality, some of the function implementations in this module should
be simplifyed.
*/

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
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_from_array(array))
    }

    /// Creates a mask with all components set to the given value.
    #[inline]
    #[must_use]
    pub fn splat(value: bool) -> Self {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_splat(value))
    }

    /// Creates a mask by calling function `f` for each component index.
    ///
    /// Equivalent to `(f(0), f(1), f(2), ...)`.
    #[inline]
    #[must_use]
    pub fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> bool,
    {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_from_fn((f,)))
    }

    /// Converts the mask to the specified alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub fn to_alignment<A2: Alignment>(self) -> Mask<N, T, A2> {
        (const {
            if A::IS_ALIGNED == A2::IS_ALIGNED {
                // `A` and `A2` are guaranteed to be the same type as long as
                // `A::IS_ALIGNED == A2::IS_ALIGNED` which was just checked.
                // Thus the transmuted types are the same type.
                unsafe {
                    transmute::<
                        fn(Mask<N, T, A>) -> Mask<N, T, A>,
                        fn(Mask<N, T, A>) -> Mask<N, T, A2>,
                    >(|mask| mask)
                }
            } else {
                |mask: Self| Mask::from_array(mask.to_array())
            }
        })(self)
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
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_to_array(self))
    }

    /// Returns `true` if all of the mask's components are `true`.
    #[inline]
    #[must_use]
    pub fn all(self) -> bool {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_all(self))
    }

    /// Returns `true` if any of the mask's components are `true`.
    #[inline]
    #[must_use]
    pub fn any(self) -> bool {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_any(self))
    }

    /// Selects between the components of `if_true` and `if_false` based on the
    /// values of the mask.
    #[inline]
    #[must_use]
    pub fn select(self, if_true: Vector<N, T, A>, if_false: Vector<N, T, A>) -> Vector<N, T, A> {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_select(
            self, if_true, if_false
        ))
    }

    /// Returns an iterator over the mask's components.
    #[inline]
    #[must_use]
    pub fn iter(self) -> core::array::IntoIter<bool, N> {
        self.to_array().into_iter()
    }

    /// Returns the component at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get(self, index: usize) -> bool {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_get(self, index))
    }

    /// Sets the component at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_set(self, index, value))
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
        Self::from_array([x, y])
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
        Self::from_array([x, y, z])
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
        Self::from_array([x, y, z, w])
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
        write!(f, "{:?}", Vector::<N, bool, A>::from_array(self.to_array()))
    }
}

impl<const N: usize, T, A: Alignment> Display for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", Vector::<N, bool, A>::from_array(self.to_array()))
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_eq(self, other))
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
        self.to_array().hash(state);
    }
}

impl<const N: usize, T, A: Alignment> Default for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn default() -> Self {
        Self::splat(bool::default())
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
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_not(self))
    }
}

macro_rules! impl_binary_op {
    ($Op:ident $op:ident, $mask_op:ident) => {
        impl<const N: usize, T, A: Alignment> $Op for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: Self) -> Self::Output {
                specialize!(<T::Repr as MaskBackend<N, A>>::$mask_op(self, rhs))
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
impl_binary_op!(BitAnd bitand, mask_bitand);
impl_binary_op!(BitOr bitor, mask_bitor);
impl_binary_op!(BitXor bitxor, mask_bitxor);

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

// SAFETY: Mask representations must be either equivalent to `[bool; N]` or be
// simple intrinsic types. Both are `Send`.
unsafe impl<const N: usize, T, A: Alignment> Send for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

// SAFETY: Mask representations must be either equivalent to `[bool; N]` or be
// simple intrinsic types. Both are `Sync`.
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

/// Controls the implementation of mask functions.
///
/// Unlike other backend traits (e.g., [`ScalarBackend`](crate::ScalarBackend)),
/// `MaskBackend` is implemented for [`T::Repr`](Scalar::Repr) instead of `T`.
///
/// Unlike other backend traits, `MaskBackend`'s functions have no default
/// implementation. This is because there are not enough guarantees about the
/// memory layout of masks to make a default implementation.
pub(crate) trait MaskBackend<const N: usize, A: Alignment>
where
    Length<N>: SupportedLength,
{
    fn mask_from_array<T>(array: [bool; N]) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;

    fn mask_splat<T>(value: bool) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;

    /// A one member tuple is used to fix type inference in the macro
    /// [`specialize`], which fails for generic function types.
    fn mask_from_fn<T, F>(f: (F,)) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>,
        F: FnMut(usize) -> bool;

    fn mask_to_array<T>(mask: Mask<N, T, A>) -> [bool; N]
    where
        T: Scalar<Repr = Self>;

    fn mask_all<T>(mask: Mask<N, T, A>) -> bool
    where
        T: Scalar<Repr = Self>;

    fn mask_any<T>(mask: Mask<N, T, A>) -> bool
    where
        T: Scalar<Repr = Self>;

    fn mask_select<T>(
        mask: Mask<N, T, A>,
        if_true: Vector<N, T, A>,
        if_false: Vector<N, T, A>,
    ) -> Vector<N, T, A>
    where
        T: Scalar<Repr = Self>;

    fn mask_get<T>(mask: Mask<N, T, A>, index: usize) -> bool
    where
        T: Scalar<Repr = Self>;

    fn mask_set<T>(mask: &mut Mask<N, T, A>, index: usize, value: bool)
    where
        T: Scalar<Repr = Self>;

    fn mask_eq<T>(mask: &Mask<N, T, A>, other: &Mask<N, T, A>) -> bool
    where
        T: Scalar<Repr = Self>;

    fn mask_not<T>(mask: Mask<N, T, A>) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;

    fn mask_bitand<T>(mask: Mask<N, T, A>, rhs: Mask<N, T, A>) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;

    fn mask_bitor<T>(mask: Mask<N, T, A>, rhs: Mask<N, T, A>) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;

    fn mask_bitxor<T>(mask: Mask<N, T, A>, rhs: Mask<N, T, A>) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;
}
